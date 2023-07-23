use wasm_bindgen::prelude::*;

use logo_renderer::Context;
use logo_renderer::logo_runtime::state::StateData;

#[wasm_bindgen]
pub fn context_create(width: i32, height: i32) -> Context {
   Context::new(width, height)
}

#[wasm_bindgen]
pub fn context_render(context: &mut Context, proc_source: &str, cmd_source: &str) -> Result<Vec<u8>, String> {
    context.render(proc_source, cmd_source)
}

#[wasm_bindgen]
pub fn context_get_state(context: &mut Context) -> StateData {
    context.state.state.data
}

#[wasm_bindgen]
pub fn render(proc_source: &str, cmd_source: &str, width: i32, height: i32) -> Result<Vec<u8>, String> {
    let mut context = Context::new(width, height);
    context.render(proc_source, cmd_source)
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn context_set_show_fn(context: &mut Context, f: js_sys::Function) {
    context.state.state.delegate.show_fn = Some(Box::new(move |msg: &str| {
        let this = JsValue::null();
        let _ = f.call1(&this, &JsValue::from(msg));
    }));
}
