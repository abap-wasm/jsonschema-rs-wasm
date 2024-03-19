use wasm_bindgen::prelude::*;
use jsonschema::JSONSchema;
use serde_json::json;

#[wasm_bindgen]
pub fn main() -> String {
    console_error_panic_hook::set_once();

    let schema = json!({"maxLength": 5});
    let instance = json!("fosssso");
    let compiled = JSONSchema::compile(&schema).expect("A valid schema");
    let result = compiled.validate(&instance);

    let mut ret: String = "".to_owned();
    if let Err(errors) = result {
        for error in errors {
            ret.push_str(format!("{error}").as_str());

            println!("Validation error: {}", error);
            println!("Instance path: {}", error.instance_path);
        }
    } else {
      ret.push_str(format!("ok").as_str());
    }
    return ret;
}