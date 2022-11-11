mod bindings;

use bindings::{
    collection as b_collection, get_doc as b_get_doc, get_docs as b_get_docs,
    set_doc_with_options as b_set_doc_with_options, where_ as b_where,
};
pub use bindings::{
    delete_doc, doc, get_firestore, on_snapshot_doc, on_snapshot_query, query, set_doc,
    CollectionReference, DocumentReference, DocumentSnapshot, Firestore, Query, QueryConstraint,
    QuerySnapshot, SetDocOptions,
};
use std::{error::Error, fmt};
use wasm_bindgen::{JsCast, JsValue};

use crate::FirebaseError;

#[derive(Clone, Debug, derive_more::Deref)]
pub struct FirestoreError {
    pub kind: FirestoreErrorKind,
    #[deref]
    pub source: FirebaseError,
}

impl fmt::Display for FirestoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.source.fmt(f)
    }
}

impl Error for FirestoreError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.source)
    }
}

impl From<FirebaseError> for FirestoreError {
    fn from(err: FirebaseError) -> Self {
        let kind = err.code().parse().unwrap();

        Self { kind, source: err }
    }
}

#[derive(Clone, Debug, strum_macros::EnumString)]
#[non_exhaustive]
pub enum FirestoreErrorKind {
    #[strum(serialize = "cancelled")]
    Cancelled,
    #[strum(serialize = "unknown")]
    Unknown,
    #[strum(serialize = "invalid-argument")]
    InvalidArgument,
    #[strum(serialize = "deadline-exceeded")]
    DeadlineExceeded,
    #[strum(serialize = "not-found")]
    NotFound,
    #[strum(serialize = "already-exists")]
    AlreadyExists,
    #[strum(serialize = "permission-denied")]
    PermissionDenied,
    #[strum(serialize = "resource-exhausted")]
    ResourceExhausted,
    #[strum(serialize = "failed-precondition")]
    FailedPrecondition,
    #[strum(serialize = "aborted")]
    Aborted,
    #[strum(serialize = "out-of-range")]
    OutOfRange,
    #[strum(serialize = "unimplemented")]
    Unimplemented,
    #[strum(serialize = "internal")]
    Internal,
    #[strum(serialize = "unavailable")]
    Unavailable,
    #[strum(serialize = "data-loss")]
    DataLoss,
    #[strum(serialize = "unauthenticated")]
    Unauthenticated,
    #[strum(default)]
    Other(String),
}

pub fn where_<V: Into<JsValue>>(
    field_path: &str,
    op: QueryConstraintOp,
    value: V,
) -> QueryConstraint {
    let value = value.into();

    b_where(field_path, &op.to_string(), value)
}

pub enum QueryConstraintOp {
    /// `<`o
    ///
    LessThan,
    /// `<=`
    LessThanEq,
    /// `>`
    GreaterThan,
    /// `>=`
    GreaterThanEq,
    /// `==`
    Eq,
    /// `!=`
    NotEq,
    /// `array-contains`
    ArrayContains,
    /// `in`
    In,
    /// `array-contains-any`
    ArrayContainsAny,
    /// `not-in`
    NotIn,
}

impl fmt::Display for QueryConstraintOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str = match self {
            Self::LessThan => "<",
            Self::LessThanEq => "<=",
            Self::GreaterThan => ">",
            Self::GreaterThanEq => ">=",
            Self::Eq => "==",
            Self::NotEq => "!=",
            Self::ArrayContains => "array-contains",
            Self::In => "in",
            Self::ArrayContainsAny => "array-contains-any",
            Self::NotIn => "not-in",
        };

        f.write_str(str)
    }
}

pub async fn get_doc(doc: &DocumentReference) -> Result<DocumentSnapshot, FirestoreError> {
    b_get_doc(doc)
        .await
        .map_err(|err| err.unchecked_into::<FirebaseError>().into())
        .map(|snapshot| snapshot.unchecked_into())
}

pub async fn get_docs(query: &Query) -> Result<QuerySnapshot, FirestoreError> {
    b_get_docs(query)
        .await
        .map_err(|err| err.unchecked_into::<FirebaseError>().into())
        .map(|snapshot| snapshot.unchecked_into())
}

pub async fn set_doc_with_options<D: Into<JsValue>>(
    doc: &DocumentReference,
    data: D,
    options: SetDocOptions,
) -> Result<(), FirestoreError> {
    b_set_doc_with_options(doc, data.into(), options)
        .await
        .map_err(|err| err.unchecked_into::<FirebaseError>().into())
}

pub fn collection(
    firestore: &Firestore,
    path: &str,
) -> Result<CollectionReference, FirestoreError> {
    b_collection(firestore, path).map_err(|err| err.into())
}
