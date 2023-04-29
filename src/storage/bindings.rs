use serde::{
  Deserialize,
  Serialize,
};
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

#[derive(Debug, Clone, TypedBuilder, Serialize)]
#[serde(rename_all = "camelCase")]
#[builder(field_defaults(default, setter(strip_option, into)))]
pub struct UploadMetadataOptions {
  pub cache_control: Option<String>,
  pub content_disposition: Option<String>,
  pub content_encoding: Option<String>,
  pub content_language: Option<String>,
  pub content_type: Option<String>,
  #[builder(default, setter(skip))]
  custom_metadata: HashMap<String, String>,
  pub md5_hash: Option<String>,
}

impl UploadMetadataOptions {
  pub fn add_custom_metadata(
    mut self,
    key: impl ToString,
    value: impl ToString,
  ) -> Self {
    self
      .custom_metadata
      .insert(key.to_string(), value.to_string());

    self
  }
}

impl UploadMetadata {
  pub fn get_custom_metadata<T>(
    &self,
  ) -> Result<Option<T>, serde_wasm_bindgen::Error>
  where
    T: for<'de> Deserialize<'de>,
  {
    self
      .custom_metadata()
      .map(|o| serde_wasm_bindgen::from_value::<T>(o.into()))
      .transpose()
  }
}

pub fn upload_bytes(
  ref_: Ref,
  data: &web_sys::Blob,
  metadata: Option<UploadMetadataOptions>,
) -> Result<UploadTask, JsValue> {
  let serializer =
    serde_wasm_bindgen::Serializer::new().serialize_maps_as_objects(true);

  let metadata = metadata.serialize(&serializer).unwrap();

  upload_bytes_(ref_, data, metadata)
}

#[wasm_bindgen(module = "firebase/storage")]
extern "C" {
  pub type Storage;
  pub type Ref;
  pub type UploadTask;
  pub type UploadTaskSnapshot;
  pub type SettableMetadata;
  #[wasm_bindgen(extends = SettableMetadata)]
  pub type UploadMetadata;
  #[wasm_bindgen(extends = UploadMetadata)]
  pub type FullMetadata;
  pub type TaskState;

  #[wasm_bindgen(js_name = getStorage)]
  pub fn get_storage() -> Storage;

  #[wasm_bindgen(js_name = ref)]
  pub fn ref_(storage: Storage, path: &str) -> Ref;

  #[wasm_bindgen(js_name = uploadBytesResumable, catch)]
  fn upload_bytes_(
    ref_: Ref,
    data: &web_sys::Blob,
    metadata: JsValue,
  ) -> Result<UploadTask, JsValue>;

  #[wasm_bindgen(js_name = getDownloadURL, catch)]
  pub async fn get_download_url(ref_: Ref) -> Result<JsValue, JsValue>;

  #[wasm_bindgen(js_name = deleteObject, catch)]
  pub async fn delete_object(ref_: Ref) -> Result<(), JsValue>;

  #[wasm_bindgen(js_name = getMetadata, catch)]
  pub async fn get_metadata(ref_: Ref) -> Result<JsValue, JsValue>;

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

  #[wasm_bindgen(method, getter, js_name = ref)]
  pub fn ref_(this: &UploadTaskSnapshot) -> Ref;

  #[wasm_bindgen(method, getter)]
  pub fn metadata(this: &UploadTaskSnapshot) -> FullMetadata;

  #[wasm_bindgen(method, getter)]
  pub fn state(this: &UploadTaskSnapshot) -> String;

  #[wasm_bindgen(method, getter)]
  pub fn task(this: &UploadTaskSnapshot) -> UploadTask;

  // =========================================================================
  //                            SettableMetadata
  // =========================================================================

  #[wasm_bindgen(method, getter, js_name = cacheControl)]
  pub fn cache_control(this: &SettableMetadata) -> Option<String>;

  #[wasm_bindgen(method, getter, js_name = contentDisposition)]
  pub fn content_disposition(this: &SettableMetadata) -> Option<String>;

  #[wasm_bindgen(method, getter, js_name = contentEncoding)]
  pub fn content_encoding(this: &SettableMetadata) -> Option<String>;

  #[wasm_bindgen(method, getter, js_name = contentLanguage)]
  pub fn content_language(this: &SettableMetadata) -> Option<String>;

  #[wasm_bindgen(method, getter, js_name = contentType)]
  pub fn content_type(this: &SettableMetadata) -> Option<String>;

  #[wasm_bindgen(method, getter, js_name = customMetadata)]
  pub fn custom_metadata(this: &SettableMetadata) -> Option<js_sys::Object>;

  // =========================================================================
  //                            UploadMetadata
  // =========================================================================

  #[wasm_bindgen(method, getter, js_name = md5Hash)]
  pub fn md5_hash(this: &UploadMetadata) -> Option<String>;

  // =========================================================================
  //                            FullMetadata
  // =========================================================================

  #[wasm_bindgen(method, getter)]
  pub fn bucket(this: &FullMetadata) -> String;

  #[wasm_bindgen(method, getter, js_name = fullPath)]
  pub fn full_path(this: &FullMetadata) -> String;

  #[wasm_bindgen(method, getter)]
  pub fn generation(this: &FullMetadata) -> String;

  #[wasm_bindgen(method, getter)]
  pub fn metageneration(this: &FullMetadata) -> String;

  #[wasm_bindgen(method, getter)]
  pub fn name(this: &FullMetadata) -> String;

  #[wasm_bindgen(method, getter)]
  pub fn size(this: &FullMetadata) -> u64;

  #[wasm_bindgen(method, getter, js_name = timeCreated)]
  pub fn time_created(this: &FullMetadata) -> String;

  #[wasm_bindgen(method, getter)]
  pub fn updated(this: &FullMetadata) -> String;

  #[wasm_bindgen(method, getter, js_name = downloadTokens)]
  pub fn download_tokens(this: &FullMetadata) -> Vec<js_sys::JsString>;

  #[wasm_bindgen(method, getter, js_name = ref)]
  pub fn ref_(this: &FullMetadata) -> Option<Ref>;
}
