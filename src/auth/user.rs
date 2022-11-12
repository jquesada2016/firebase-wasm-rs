use serde::Deserialize;
use wasm_bindgen::{prelude::*, JsCast};

use crate::FirebaseError;

use super::AuthError;

impl User {
    pub async fn delete(&self) -> Result<(), AuthError> {
        self.delete_js()
            .await
            .map_err(|err| err.unchecked_into::<FirebaseError>().into())
    }

    pub async fn get_id_token_result(&self, force_refresh: bool) -> Result<IdTokenResult, JsValue> {
        self.get_id_token_result_js(force_refresh)
            .await
            .map(Into::into)
    }

    pub async fn get_id_token(&self, force_refresh: bool) -> Result<String, JsValue> {
        self.get_id_token_js(force_refresh)
            .await
            .map(JsCast::unchecked_into::<js_sys::JsString>)
            .map(|token| String::try_from(token).expect("token to be a string"))
    }
}

impl ParsedToken {
    pub fn custom_claims<T>(&self) -> Result<T, serde_wasm_bindgen::Error>
    where
        T: for<'de> Deserialize<'de>,
    {
        serde_wasm_bindgen::from_value::<T>(self.unchecked_ref::<JsValue>().to_owned())
    }
}

#[wasm_bindgen(module = "firebase/auth")]
extern "C" {
    #[derive(Clone, Debug)]
    #[wasm_bindgen(extends = UserInfo, typescript_type = r#"import("firebase/auth").User"#)]
    pub type User;
    #[derive(Clone, Debug)]
    pub type UserMetadata;
    #[derive(Clone, Debug)]
    pub type UserInfo;
    #[derive(Clone, Debug)]
    pub type IdTokenResult;
    #[derive(Clone, Debug)]
    pub type ParsedToken;
    #[derive(Clone, Debug)]
    pub type Firebase;

    // =========================================================================
    //                                UserInfo
    // =========================================================================

    #[wasm_bindgen(method, getter, js_name = displayName)]
    pub fn display_name(this: &UserInfo) -> Option<String>;

    #[wasm_bindgen(method, getter, js_name = email)]
    pub fn email(this: &UserInfo) -> Option<String>;

    #[wasm_bindgen(method, getter, js_name = phoneNumber)]
    pub fn phone_number(this: &UserInfo) -> Option<String>;

    #[wasm_bindgen(method, getter, js_name = photoURL)]
    pub fn photo_url(this: &UserInfo) -> Option<String>;

    #[wasm_bindgen(method, getter, js_name = providerId)]
    pub fn provider_id(this: &UserInfo) -> String;

    #[wasm_bindgen(method, getter, js_name = uid)]
    pub fn uid(this: &UserInfo) -> String;

    // =========================================================================
    //                              UserMetadata
    // =========================================================================

    #[wasm_bindgen(method, getter, js_name = creationTime)]
    pub fn creation_time(this: &UserMetadata) -> String;

    #[wasm_bindgen(method, getter, js_name = lastSignInTime)]
    pub fn last_sign_in_time(this: &UserMetadata) -> String;

    // =========================================================================
    //                                  User
    // =========================================================================

    #[wasm_bindgen(method, getter, js_name = emailVerified)]
    pub fn email_verified(this: &User) -> bool;

    #[wasm_bindgen(method, getter, js_name = isAnonymous)]
    pub fn is_anonymous(this: &User) -> bool;

    #[wasm_bindgen(getter)]
    pub fn metadata(this: &User) -> UserMetadata;

    #[wasm_bindgen(method, getter, js_name = providerData)]
    pub fn provider_data(this: &User) -> Vec<UserInfo>;

    #[wasm_bindgen(method, getter, js_name = refreshToken)]
    pub fn refresh_token(this: &User) -> String;

    #[wasm_bindgen(method, getter, js_name = tenantId)]
    pub fn tenant_id(this: &User) -> String;

    #[wasm_bindgen(method, js_name = delete, catch)]
    async fn delete_js(this: &User) -> Result<(), JsValue>;

    #[wasm_bindgen(method, js_name = getIdToken, catch)]
    async fn get_id_token_js(this: &User, force_refresh: bool) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(method, js_name = getIdTokenResult, catch)]
    async fn get_id_token_result_js(this: &User, force_refresh: bool) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(method, catch)]
    async fn reload(this: &User) -> Result<(), JsValue>;

    #[wasm_bindgen(method, js_name = toJSON)]
    pub fn to_json(this: &User) -> js_sys::Object;

    // =========================================================================
    //                              IdTokenResult
    // =========================================================================

    #[wasm_bindgen(method, getter, js_name = authTime)]
    pub fn auth_time(this: &IdTokenResult) -> String;

    #[wasm_bindgen(method, getter, js_name = expirationTime)]
    pub fn expiration_time(this: &IdTokenResult) -> String;

    #[wasm_bindgen(method, getter, js_name = issuedAtTime)]
    pub fn issued_at_time(this: &IdTokenResult) -> String;

    #[wasm_bindgen(method, getter, js_name = signInProvider)]
    pub fn sign_in_provider(this: &IdTokenResult) -> Option<String>;

    #[wasm_bindgen(method, getter, js_name = signInSecondFactor)]
    pub fn sign_in_second_factor(this: &IdTokenResult) -> Option<String>;

    #[wasm_bindgen(method, getter, js_name = token)]
    pub fn token(this: &IdTokenResult) -> Option<String>;

    #[wasm_bindgen(method, getter, js_name = claims)]
    pub fn claims(this: &IdTokenResult) -> ParsedToken;

    // =========================================================================
    //                              ParsedToken
    // =========================================================================

    #[wasm_bindgen(method, getter)]
    pub fn exp(this: &ParsedToken) -> Option<String>;

    #[wasm_bindgen(method, getter)]
    pub fn sub(this: &ParsedToken) -> Option<String>;

    #[wasm_bindgen(method, getter)]
    pub fn auth_time(this: &ParsedToken) -> Option<String>;

    #[wasm_bindgen(method, getter)]
    pub fn iat(this: &ParsedToken) -> Option<String>;

    #[wasm_bindgen(method, getter)]
    pub fn firebase(this: &ParsedToken) -> Option<Firebase>;

    // =========================================================================
    //                                Firebase
    // =========================================================================

    #[wasm_bindgen(method, getter)]
    pub fn sign_in_provider(this: &Firebase) -> Option<Firebase>;

    #[wasm_bindgen(method, getter)]
    pub fn sign_in_second_factor(this: &Firebase) -> Option<Firebase>;

    #[wasm_bindgen(method, getter)]
    pub fn identities(this: &Firebase) -> Option<js_sys::Object>;
}
