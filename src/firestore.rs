mod bindings;

use crate::FirebaseError;
use bindings as b;
pub use bindings::{
    add_doc, delete_doc, doc, get_firestore, limit, on_snapshot_doc, on_snapshot_query, query,
    set_doc, update_doc, CollectionReference, DocumentReference, DocumentSnapshot, Firestore,
    Query, QueryConstraint, QuerySnapshot, SetDocOptions, Timestamp, Transaction,
};
use futures::Future;
use std::{cell::RefCell, error::Error, fmt, rc::Rc, time::SystemTime};
use wasm_bindgen::{
    prelude::{Closure, *},
    JsCast, JsValue,
};

#[derive(Clone, Debug, derive_more::Deref)]
#[wasm_bindgen(getter_with_clone)]
pub struct FirestoreError {
    #[wasm_bindgen(skip)]
    pub kind: FirestoreErrorKind,
    #[deref]
    #[wasm_bindgen(readonly)]
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

#[derive(Clone, Debug, strum::EnumString)]
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

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Asc,
    Desc,
}

pub fn order_by(field_path: &str, direction: Direction) -> QueryConstraint {
    b::order_by(
        field_path,
        match direction {
            Direction::Asc => "asc",
            Direction::Desc => "desc",
        },
    )
}

pub fn where_<V: Into<JsValue>>(
    field_path: &str,
    op: QueryConstraintOp,
    value: V,
) -> QueryConstraint {
    let value = value.into();

    b::where_(field_path, &op.to_string(), value)
}

pub enum QueryConstraintOp {
    /// `<`o
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

pub async fn get_doc(doc: DocumentReference) -> Result<DocumentSnapshot, FirestoreError> {
    b::get_doc(doc)
        .await
        .map_err(|err| err.unchecked_into::<FirebaseError>().into())
        .map(|snapshot| snapshot.unchecked_into())
}

pub async fn get_docs(query: Query) -> Result<QuerySnapshot, FirestoreError> {
    b::get_docs(query)
        .await
        .map_err(|err| err.unchecked_into::<FirebaseError>().into())
        .map(|snapshot| snapshot.unchecked_into())
}

pub async fn set_doc_with_options<D: Into<JsValue>>(
    doc: DocumentReference,
    data: D,
    options: SetDocOptions,
) -> Result<(), FirestoreError> {
    b::set_doc_with_options(doc, data.into(), options)
        .await
        .map_err(|err| err.unchecked_into::<FirebaseError>().into())
}

pub fn collection(firestore: Firestore, path: &str) -> Result<CollectionReference, FirestoreError> {
    b::collection(firestore, path).map_err(|err| err.into())
}

impl Transaction {
    pub async fn get(&self, doc: DocumentReference) -> Result<DocumentSnapshot, FirestoreError> {
        self.get_js(doc)
            .await
            .map_err(|err| err.unchecked_into::<FirebaseError>().into())
            .map(|snapshot| snapshot.unchecked_into())
    }

    pub fn set(&self, doc: DocumentReference, data: JsValue) -> Result<Self, FirestoreError> {
        self.set_js(doc, data).map_err(Into::into)
    }

    pub fn update(&self, doc: DocumentReference, data: JsValue) -> Result<Self, FirestoreError> {
        self.update_js(doc, data).map_err(Into::into)
    }

    pub fn delete(&self, doc: DocumentReference) -> Result<Self, FirestoreError> {
        self.delete_js(doc).map_err(Into::into)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum TransactionError<Err> {
    #[error("firestore error: {0}")]
    Firestore(
        #[from]
        #[source]
        FirestoreError,
    ),
    #[error("user-thrown error: {0:#?}")]
    User(#[source] Err),
}

impl<Err: Clone> Clone for TransactionError<Err> {
    fn clone(&self) -> Self {
        match self {
            Self::Firestore(arg0) => Self::Firestore(arg0.clone()),
            Self::User(arg0) => Self::User(arg0.clone()),
        }
    }
}

impl<Err> TransactionError<Err> {
    pub fn is_firestore_err(&self) -> bool {
        matches!(self, Self::Firestore(_))
    }

    pub fn is_user_err(&self) -> bool {
        matches!(self, Self::User(_))
    }

    pub fn firestore_err(self) -> Option<FirestoreError> {
        if let Self::Firestore(err) = self {
            Some(err)
        } else {
            None
        }
    }

    pub fn user_err(self) -> Option<Err> {
        if let Self::User(err) = self {
            Some(err)
        } else {
            None
        }
    }
}

#[wasm_bindgen]
pub struct UserAbortedTransaction;

pub async fn run_transaction<F, Fut, T, Err>(
    firestore: Firestore,
    update_fn: F,
) -> Result<T, TransactionError<Err>>
where
    F: FnMut(Transaction) -> Fut + 'static,
    Fut: Future<Output = Result<T, Err>>,
    T: 'static,
    Err: 'static,
{
    let result: Rc<RefCell<Option<Result<T, Err>>>> = Default::default();

    let update_fn = Rc::new(RefCell::new(update_fn));

    let update_fn = Closure::new(clone!([result], move |t| {
        wasm_bindgen_futures::future_to_promise(clone!([update_fn, result], async move {
            let mut update_fn_borrow = update_fn.borrow_mut();
            let mut result_borrow = result.borrow_mut();

            match update_fn_borrow(t).await {
                Ok(v) => {
                    *result_borrow = Some(Ok(v));

                    Ok(JsValue::UNDEFINED)
                }
                Err(err) => {
                    *result_borrow = Some(Err(err));

                    Err(UserAbortedTransaction.into())
                }
            }
        }))
    }));

    // Check to see if the error is a firebase error
    if let Err(err) = b::run_transaction(firestore, &update_fn).await {
        if let Ok(err) = err.dyn_into::<js_sys::Object>() {
            if err.constructor().name() == "FirebaseError" {
                return Err(TransactionError::Firestore(
                    err.unchecked_into::<FirebaseError>().into(),
                ));
            }
        }
    }

    let mut result_borrow = result.borrow_mut();

    result_borrow
        .take()
        .unwrap()
        .map_err(|err| TransactionError::User(err))
}

impl From<SystemTime> for Timestamp {
    fn from(time: SystemTime) -> Self {
        Self::from_millis(
            time.duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs_f64()
                * 1000.,
        )
    }
}
