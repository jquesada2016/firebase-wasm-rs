use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "firebase/auth")]
extern "C" {
    pub type Auth;
    #[derive(Debug)]
    pub type User;

    #[wasm_bindgen(js_name = getAuth)]
    pub fn get_auth() -> Auth;

    #[wasm_bindgen(js_name = onAuthStateChanged)]
    pub fn on_auth_state_changed(
        auth: Auth,
        callback: &Closure<dyn FnMut(Option<User>)>,
    );
}
