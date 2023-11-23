use super::AuthError;
use crate::FirebaseError;
use serde::Deserialize;
use wasm_bindgen::{prelude::*, JsCast};

impl ParsedToken {
    pub fn custom_claims<T>(&self) -> Result<T, serde_wasm_bindgen::Error>
    where
        T: for<'de> Deserialize<'de>,
    {
        serde_wasm_bindgen::from_value::<T>(self.unchecked_ref::<JsValue>().to_owned())
    }
}

#[wasm_bindgen_struct]
#[opts(module = "firebase/auth", getter)]
#[derive(Clone, Debug)]
pub struct UserInfo {
    pub display_name: Option<String>,
    pub email: Option<String>,
    pub phone_number: Option<String>,
    #[opts(js_name = "photoURL")]
    pub photo_url: Option<String>,
    pub provider_id: String,
    pub uid: String,
}

#[wasm_bindgen_struct]
#[opts(module = "firestore/auth", getter)]
#[derive(Clone, Debug)]
pub struct UserMetadata {
    pub creation_time: String,
    pub last_sign_in_time: String,
}

#[wasm_bindgen_struct]
#[opts(module = "firebase/auth", getter)]
#[derive(Clone, Debug)]
pub struct IdTokenResult {
    pub auth_time: String,
    pub expiration_time: String,
    pub issued_at_time: String,
    pub sign_in_provider: Option<String>,
    pub sign_in_second_factor: Option<String>,
    pub token: Option<String>,
    pub claims: ParsedToken,
}

#[wasm_bindgen_struct]
#[opts(module = "firebase/auth", getter)]
#[derive(Clone, Debug)]
pub struct Firebase {
    #[opts(js_name = "sign_in_provider")]
    pub sign_in_provider: Option<Firebase>,
    #[opts(js_name = "sign_in_second_factor")]
    pub sign_in_second_factor: Option<Firebase>,
    pub identities: Option<js_sys::Object>,
}

#[wasm_bindgen_struct]
#[opts(module = "firebase/auth", getter)]
#[derive(Clone, Debug)]
pub struct ParsedToken {
    pub exp: Option<String>,
    pub sub: Option<String>,
    #[opts(js_name = "auth_time")]
    pub auth_time: Option<String>,
    pub iat: Option<String>,
    pub firebase: Option<Firebase>,
}

#[wasm_bindgen_struct]
#[opts(module = "firebase/auth", getter, extends = UserInfo)]
#[derive(Clone, Debug)]
#[wasm_bindgen(typescript_type = r#"import("firebase/auth").User"#)]
pub struct User {
    pub email_verified: bool,
    pub is_anonymous: bool,
    pub metadata: UserMetadata,
    pub provider_data: Vec<UserInfo>,
    pub refresh_token: String,
    pub tenant_id: String,
}

#[wasm_bindgen_struct]
#[opts(module = "firebase/auth")]
impl User {
    pub async fn delete(&self) -> MapValue<Result<(), JsValue>, Result<(), AuthError>> {
        self.delete_js()
            .await
            .map_err(|err| err.unchecked_into::<FirebaseError>().into())
    }

    async fn get_id_token(
        &self,
        force_refresh: bool,
    ) -> MapValue<Result<JsValue, JsValue>, Result<String, JsValue>> {
        self.get_id_token_js(force_refresh)
            .await
            .map(JsCast::unchecked_into::<js_sys::JsString>)
            .map(|token| String::try_from(token).expect("token to be a string"))
    }

    async fn get_id_token_result(
        &self,
        force_refresh: bool,
    ) -> MapValue<Result<JsValue, JsValue>, Result<IdTokenResult, JsValue>> {
        self.get_id_token_result_js(force_refresh)
            .await
            .map(Into::into)
    }

    async fn reload(&self) -> Result<(), JsValue>;

    pub fn to_json(&self) -> js_sys::Object;
}
