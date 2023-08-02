use serde::{Deserialize, Serialize};
use std::{future::Future, marker::PhantomData};
use wasm_bindgen::{prelude::*, JsCast};

#[derive(Clone, Debug, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(default, setter(strip_option)))]
pub struct HttpsCallableOptions {
    pub timeout: Option<usize>,
}

#[derive(Deserialize)]
struct HttpsCallableResponse<T> {
    data: T,
}

pub struct HttpsCallable<Req, Res>
where
    Req: Serialize,
    Res: for<'de> Deserialize<'de>,
{
    callable: js_sys::Function,
    _request_data: PhantomData<Req>,
    _response_data: PhantomData<Res>,
}

impl<Req, Res> HttpsCallable<Req, Res>
where
    Req: Serialize,
    Res: for<'de> Deserialize<'de>,
{
    #[track_caller]
    pub fn call_(
        &self,
        data: Req,
    ) -> impl Future<Output = Result<Result<Res, serde_wasm_bindgen::Error>, JsValue>> {
        let data = serde_wasm_bindgen::to_value(&data).expect("data to serialize successfully");

        let res = self.callable.call1(&JsValue::UNDEFINED, &data).unwrap();

        let fut = wasm_bindgen_futures::JsFuture::from(res.unchecked_into::<js_sys::Promise>());

        async move {
            fut.await.map(|res| {
                serde_wasm_bindgen::from_value::<HttpsCallableResponse<Res>>(res)
                    .map(|res| res.data)
            })
        }
    }
}

#[cfg(feature = "nightly")]
impl<Req, Res> FnOnce<(Req,)> for HttpsCallable<Req, Res>
where
    Req: Serialize + 'static,
    Res: for<'de> Deserialize<'de> + 'static,
{
    type Output =
        Pin<Box<dyn Future<Output = Result<Result<Res, serde_wasm_bindgen::Error>, JsValue>>>>;

    extern "rust-call" fn call_once(self, args: (Req,)) -> Self::Output {
        Box::pin(self.call_(args.0))
    }
}

#[cfg(feature = "nightly")]
impl<Req, Res> FnMut<(Req,)> for HttpsCallable<Req, Res>
where
    Req: Serialize + 'static,
    Res: for<'de> Deserialize<'de> + 'static,
{
    extern "rust-call" fn call_mut(&mut self, args: (Req,)) -> Self::Output {
        Box::pin(self.call_(args.0))
    }
}

#[cfg(feature = "nightly")]
impl<Req, Res> Fn<(Req,)> for HttpsCallable<Req, Res>
where
    Req: Serialize + 'static,
    Res: for<'de> Deserialize<'de> + 'static,
{
    extern "rust-call" fn call(&self, args: (Req,)) -> Self::Output {
        Box::pin(self.call_(args.0))
    }
}

#[track_caller]
pub fn https_callable<Req, Res>(
    functions_instance: &Functions,
    name: &str,
    options: Option<HttpsCallableOptions>,
) -> HttpsCallable<Req, Res>
where
    Req: Serialize,
    Res: for<'de> Deserialize<'de>,
{
    let options = serde_wasm_bindgen::to_value(&options).unwrap();

    let callable = https_callable_(functions_instance, name, options);

    HttpsCallable {
        callable,
        _request_data: PhantomData,
        _response_data: PhantomData,
    }
}

#[wasm_bindgen(module = "firebase/functions")]
extern "C" {
    pub type Functions;

    #[wasm_bindgen(js_name = getFunctions)]
    pub fn get_functions() -> Functions;

    #[wasm_bindgen(js_name = httpsCallable)]
    fn https_callable_(
        functions_instance: &Functions,
        name: &str,
        options: JsValue,
    ) -> js_sys::Function;
}
