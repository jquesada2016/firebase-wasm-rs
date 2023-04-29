use wasm_bindgen::prelude::*;

use crate::FirebaseError;
use js_sys::Date;

#[derive(Clone, Copy, Debug, Default)]
#[wasm_bindgen(getter_with_clone)]
pub struct SetDocOptions {
    pub merge: Option<bool>,
}

#[wasm_bindgen(module = "firebase/firestore")]
extern "C" {
    #[derive(Clone, Debug)]
    pub type Firestore;
    #[derive(Clone, Debug)]
    pub type DocumentReference;
    #[derive(Clone, Debug)]
    pub type CollectionReference;
    #[derive(Clone, Debug)]
    pub type DocumentSnapshot;
    #[derive(Clone, Debug)]
    pub type Query;
    #[derive(Clone, Debug)]
    pub type QuerySnapshot;
    #[derive(Clone, Debug)]
    pub type QueryConstraint;
    #[derive(Clone, Debug)]
    pub type Transaction;
    #[derive(Clone, Debug)]
    pub type Timestamp;

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
    pub fn collection(
        firestore: Firestore,
        path: &str,
    ) -> Result<CollectionReference, FirebaseError>;

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
    pub async fn delete_doc(doc: DocumentReference) -> Result<(), JsValue>;

    #[wasm_bindgen(js_name = runTransaction, catch)]
    pub async fn run_transaction(
        firestore: Firestore,
        update_fn: &Closure<dyn FnMut(Transaction) -> js_sys::Promise>,
    ) -> Result<(), JsValue>;

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

    // =========================================================================
    //                            DocumentSnapshot
    // =========================================================================

    #[wasm_bindgen(method, catch, js_name = get)]
    pub(crate) async fn get_js(
        this: &Transaction,
        doc: DocumentReference,
    ) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(method, js_name = set, catch)]
    pub(crate) fn set_js(
        this: &Transaction,
        doc: DocumentReference,
        data: JsValue,
    ) -> Result<Transaction, FirebaseError>;

    #[wasm_bindgen(method, js_name = update, catch)]
    pub(crate) fn update_js(
        this: &Transaction,
        doc: DocumentReference,
        data: JsValue,
    ) -> Result<Transaction, FirebaseError>;

    #[wasm_bindgen(method, js_name = delete, catch)]
    pub(crate) fn delete_js(
        this: &Transaction,
        doc: DocumentReference,
    ) -> Result<Transaction, FirebaseError>;

    // =========================================================================
    //                            Timestamp
    // =========================================================================

    #[wasm_bindgen(js_namespace = Timestamp)]
    pub fn now() -> Timestamp;

    #[wasm_bindgen(method, js_name = toDate)]
    pub fn to_date(this: &Timestamp) -> Date;

    #[wasm_bindgen(method, js_name = isEqual)]
    pub fn is_equal(this: &Timestamp, other: &Timestamp) -> bool;
}

impl PartialEq for Timestamp {
    fn eq(&self, other: &Self) -> bool {
        self.is_equal(other)
    }
}
