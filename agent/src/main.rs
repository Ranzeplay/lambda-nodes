mod routes;

use deno_core::error::AnyError;
use std::rc::Rc;
use actix_web::{App, HttpServer};
use crate::routes::ping::ping;

async fn run_js(file_path: &str) -> Result<(), AnyError> {
    let main_module = deno_core::resolve_path(file_path, &std::env::current_dir()?)?;
    let mut js_runtime = deno_core::JsRuntime::new(deno_core::RuntimeOptions {
        module_loader: Some(Rc::new(deno_core::FsModuleLoader)),
        ..Default::default()
    });

    let internal_mod_id = js_runtime
        .load_side_es_module_from_code(
            &deno_core::ModuleSpecifier::parse("runjs:runtime.js")?,
            include_str!("../runtime.js"),
        )
        .await?;
    let internal_mod_result = js_runtime.mod_evaluate(internal_mod_id);

    let mod_id = js_runtime.load_main_es_module(&main_module).await?;
    let result = js_runtime.mod_evaluate(mod_id);
    js_runtime.run_event_loop(Default::default()).await?;
    internal_mod_result.await?;
    result.await.map_err(AnyError::from)
}

#[tokio::main]
#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    if let Err(error) = runtime.block_on(run_js("./example.js")) {
        eprintln!("error: {}", error);
    }

    HttpServer::new(|| {
        App::new()
            .service(ping)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await?;

    Ok(())
}
