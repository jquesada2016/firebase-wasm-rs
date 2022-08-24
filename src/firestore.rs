mod bindings;

use std::fmt;

pub use bindings::{
    collection, delete_doc, doc, get_firestore, on_snapshot_doc, on_snapshot_query, query, set_doc,
    set_doc_with_options, CollectionReference, DocumentReference, DocumentSnapshot, Firestore,
    Query, QueryConstraint, QuerySnapshot, SetDocOptions,
};
use bindings::{get_doc as b_get_doc, get_docs as b_get_docs, where_ as b_where};
use wasm_bindgen::{JsCast, JsValue};

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

pub async fn get_doc(doc: DocumentReference) -> Result<DocumentSnapshot, JsValue> {
    let snapshot = b_get_doc(doc).await?;

    Ok(snapshot.unchecked_into())
}

pub async fn get_docs(query: Query) -> Result<QuerySnapshot, JsValue> {
    let snapshot = b_get_docs(query).await?;

    Ok(snapshot.unchecked_into())
}
