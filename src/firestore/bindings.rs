use wasm_bindgen::prelude::*;

#[wasm_bindgen(getter_with_clone)]
pub struct SetDocOptions {
    pub merge: Option<bool>,
}

#[wasm_bindgen(module = "firebase/firestore")]
extern "C" {
    pub type Firestore;
    pub type DocumentReference;
    pub type CollectionReference;
    #[derive(Debug)]
    pub type DocumentSnapshot;
    pub type Query;
    #[derive(Debug)]
    pub type QuerySnapshot;
    pub type QueryConstraint;

    #[wasm_bindgen(js_name = getFirestore)]
    pub fn get_firestore() -> Firestore;

    #[wasm_bindgen(catch)]
    pub fn doc(firestore: Firestore, path: &str) -> Result<DocumentReference, JsValue>;

    #[wasm_bindgen(js_name = getDoc, catch)]
    pub async fn get_doc(doc: DocumentReference) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(js_name = getDocs, catch)]
    pub async fn get_docs(query: Query) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(js_name = "setDoc", catch)]
    pub async fn set_doc(doc: DocumentReference, data: JsValue) -> Result<(), JsValue>;

    #[wasm_bindgen(js_name = "setDoc", catch)]
    pub async fn set_doc_with_options(
        doc: DocumentReference,
        data: JsValue,
        options: SetDocOptions,
    ) -> Result<(), JsValue>;

    #[wasm_bindgen(catch)]
    pub fn collection(firestore: Firestore, path: &str) -> Result<CollectionReference, JsValue>;

    #[wasm_bindgen(js_name = onSnapshot)]
    pub fn on_snapshot_doc(
        reference: DocumentReference,
        observer: &Closure<dyn FnMut(DocumentSnapshot)>,
    ) -> js_sys::Function;

    #[wasm_bindgen(js_name = onSnapshot)]
    pub fn on_snapshot_query(
        query: Query,
        observer: &Closure<dyn FnMut(QuerySnapshot)>,
    ) -> js_sys::Function;

    #[wasm_bindgen(variadic)]
    pub fn query(collection: CollectionReference, constraints: Vec<QueryConstraint>) -> Query;

    #[wasm_bindgen(js_name = "where")]
    pub fn where_(field_path: &str, op_str: &str, value: JsValue) -> QueryConstraint;

    #[wasm_bindgen(js_name = deleteDoc, catch)]
    pub async fn delete_doc(ref_: DocumentReference) -> Result<(), JsValue>;

    // =========================================================================
    //                            QuerySnapshot
    // =========================================================================

    #[wasm_bindgen(method, getter)]
    pub fn empty(this: &QuerySnapshot) -> bool;

    #[wasm_bindgen(method, getter)]
    pub fn size(this: &QuerySnapshot) -> usize;

    #[wasm_bindgen(method, getter)]
    pub fn docs(this: &QuerySnapshot) -> Vec<DocumentSnapshot>;

    // =========================================================================
    //                            DocumentSnapshot
    // =========================================================================

    #[wasm_bindgen(method)]
    pub fn exists(this: &DocumentSnapshot) -> bool;

    #[wasm_bindgen(method)]
    pub fn data(this: &DocumentSnapshot) -> JsValue;

    #[wasm_bindgen(method, getter)]
    pub fn id(this: &DocumentSnapshot) -> String;

    #[wasm_bindgen(method, js_name = "ref")]
    pub fn ref_(this: &DocumentSnapshot) -> DocumentReference;

}
