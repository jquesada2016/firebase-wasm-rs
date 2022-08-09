use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "firebase/storage")]
extern "C" {
    pub type Storage;
    pub type Ref;
    pub type UploadTask;
    pub type UploadTaskSnapshot;

    #[wasm_bindgen(js_name = getStorage)]
    pub fn get_storage() -> Storage;

    #[wasm_bindgen(js_name = ref)]
    pub fn ref_(storage: Storage, path: &str) -> Ref;

    #[wasm_bindgen(js_name = uploadBytesResumable, catch)]
    pub fn upload_bytes(
        ref_: Ref,
        data: &web_sys::Blob,
    ) -> Result<UploadTask, JsValue>;

    #[wasm_bindgen(js_name = getDownloadURL, catch)]
    pub async fn get_download_url(ref_: Ref) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(js_name = deleteObject, catch)]
    pub async fn delete_object(ref_: Ref) -> Result<(), JsValue>;

    // =========================================================================
    //                              UploadTask
    // =========================================================================

    #[wasm_bindgen(method)]
    pub fn on(
        this: &UploadTask,
        event: &str,
        on_snapshot: &Closure<dyn FnMut(UploadTaskSnapshot)>,
        on_error: Option<&Closure<dyn FnMut(JsValue)>>,
        on_complete: Option<&Closure<dyn FnMut()>>,
    ) -> js_sys::Function;

    #[wasm_bindgen(method)]
    pub fn cancel(this: &UploadTask) -> bool;

    #[wasm_bindgen(method)]
    pub fn pause(this: &UploadTask) -> bool;

    #[wasm_bindgen(method)]
    pub fn resume(this: &UploadTask) -> bool;

    #[wasm_bindgen(method, getter)]
    pub fn snapshot(this: &UploadTask) -> UploadTaskSnapshot;

    // =========================================================================
    //                            UploadTaskSnapshot
    // =========================================================================

    #[wasm_bindgen(method, getter, js_name = bytesTransferred)]
    pub fn bytes_transferred(this: &UploadTaskSnapshot) -> usize;

    #[wasm_bindgen(method, getter, js_name = totalBytes)]
    pub fn total_bytes(this: &UploadTaskSnapshot) -> usize;

    #[wasm_bindgen(method, getter, js_name = "ref")]
    pub fn ref_(this: &UploadTaskSnapshot) -> Ref;
}
