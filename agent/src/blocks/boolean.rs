use deno_core::_ops::RustToV8NoScope;
use deno_core::v8;
use deno_core::v8::{Global, HandleScope};

pub fn bool_true(scope: &mut HandleScope) -> Global<v8::Value> {
    let obj = v8::Boolean::new(scope, true).to_v8();
    Global::new(scope, obj)
}

pub fn bool_false(scope: &mut HandleScope) -> Global<v8::Value> {
    let obj = v8::Boolean::new(scope, false).to_v8();
    Global::new(scope, obj)
}
