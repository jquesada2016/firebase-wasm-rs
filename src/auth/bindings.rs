use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "firebase/auth")]
extern "C" {
    pub type Auth;
    #[derive(Debug)]
    #[wasm_bindgen(extends = UserInfo, typescript_type = "FirebaseAuthUser")]
    pub type User;
    pub type UserMetadata;
    #[derive(Debug)]
    pub type UserInfo;

    #[wasm_bindgen(js_name = getAuth)]
    pub fn get_auth() -> Auth;

    #[wasm_bindgen(js_name = onAuthStateChanged)]
    pub fn on_auth_state_changed(auth: Auth, callback: &Closure<dyn FnMut(Option<User>)>);

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

    #[wasm_bindgen(method, catch)]
    pub async fn delete(this: &User) -> Result<(), JsValue>;

    #[wasm_bindgen(method, js_name = getIdToken, catch)]
    pub async fn get_id_token(this: &User, force_refresh: bool) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(method, js_name = toJSON)]
    pub fn to_json(this: &User) -> js_sys::Object;
}
