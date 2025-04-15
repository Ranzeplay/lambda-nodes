use crate::executor::local_object_to_json;
use deno_core::v8;
use deno_core::v8::{Global, HandleScope, Object};
use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct LNBoolean {
    pub value: bool,
}

pub fn bool_true<'a>(scope: &mut HandleScope) -> Global<Object> {
    let obj = v8::Object::new(scope);
    let key = v8::String::new(scope, "value").unwrap();
    let value = v8::Boolean::new(scope, true);
    obj.set(scope, key.into(), value.into()).unwrap();

    println!(
        "[bool_true] obj: {:?}",
        local_object_to_json(scope, obj.clone())
    );
    Global::new(scope, obj)
}

pub fn bool_false<'a>(scope: &mut HandleScope) -> Global<Object> {
    let obj = v8::Object::new(scope);
    let key = v8::String::new(scope, "value").unwrap();
    let value = v8::Boolean::new(scope, false);
    obj.set(scope, key.into(), value.into()).unwrap();

    println!(
        "[bool_false] obj: {:?}",
        local_object_to_json(scope, obj.clone())
    );
    Global::new(scope, obj)
}
