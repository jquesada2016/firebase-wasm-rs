use wasm_bindgen::prelude::*;

#[wasm_bindgen(typescript_custom_section)]
const TYPES: &str = r#"
    import type { User as FirebaseAuthUser } from "firebase/auth";
"#;
