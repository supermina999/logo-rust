mod fill;

use wasm_bindgen::prelude::*;

pub use logo_runtime;

use raqote::*;
use logo_runtime::colors::LogoColor;
use logo_runtime::common::Pos;
use logo_runtime::drawinglib::add_drawinglib;
use logo_runtime::logo_interp::executor::execute_str;
use logo_runtime::logo_interp::executor_state::EState;
use logo_runtime::logo_interp::stdlib::add_stdlib;
use logo_runtime::state::{Delegate, State, StateData};
use crate::fill::flood_fill;

pub struct DrawingDelegate {
    pub dt: DrawTarget,
    pub show_fn: Option<Box<dyn Fn(&str)>>,
}

impl DrawingDelegate {
    fn transform_coords(&self, pos: Pos) -> (f32, f32) {
        let width = self.dt.width() as f64;
        let height = self.dt.height() as f64;
        ((pos.x + width / 2f64 + 0.5) as f32, (-pos.y + height / 2f64 + 0.5) as f32)
    }
}

impl Delegate for DrawingDelegate {
    fn clear_graphics(&mut self) {
        self.dt.clear(SolidSource{
            r: 255,
            g: 255,
            b: 255,
            a: 255,
        });
    }

    fn draw_line(&mut self, from: Pos, to: Pos, pen_size: f64, color: LogoColor) {
        let upd_from = self.transform_coords(from);
        let upd_to = self.transform_coords(to);
        let mut pb = PathBuilder::new();
        pb.move_to(upd_from.0, upd_from.1);
        pb.line_to(upd_to.0, upd_to.1);
        let path = pb.finish();
        self.dt.stroke(&path, &Source::Solid(SolidSource {
                r: color.r,
                g: color.g,
                b: color.b,
                a: 255
            }),
            &StrokeStyle {
                width: pen_size as f32,
                cap: LineCap::Square,
                ..StrokeStyle::default()
            },
            &DrawOptions::new()
        );
    }

    fn fill(&mut self, pos: Pos, color: LogoColor) {
        let upd_pos = self.transform_coords(pos);
        flood_fill(self.dt.width(), self.dt.height(), self.dt.get_data_u8_mut(),
            upd_pos.0 as i32, upd_pos.1 as i32, color);
    }

    fn show(&mut self, message: &str) {
        if let Some(show_fn) = &self.show_fn {
            (show_fn)(message);
        }
    }
}

#[wasm_bindgen]
pub struct Context {
    #[wasm_bindgen(skip)]
    pub state: EState<State<DrawingDelegate>>,
}

impl Context {
    pub fn new(width: i32, height: i32) -> Self {
        context_create(width, height)
    }

    pub fn render(&mut self, proc_source: &str, cmd_source: &str) -> Result<Vec<u8>, String> {
        context_render(self, proc_source, cmd_source)
    }
}

#[wasm_bindgen]
pub fn context_create(width: i32, height: i32) -> Context {
    let dt = DrawTarget::new(width, height);
    let dd = DrawingDelegate { dt, show_fn: None };
    let mut state = EState::new(State::new(width, height, dd));
    state.state.delegate.clear_graphics();
    add_stdlib(&mut state);
    add_drawinglib(&mut state);
    return Context {state}
}

#[wasm_bindgen]
pub fn context_render(context: &mut Context, proc_source: &str, cmd_source: &str) -> Result<Vec<u8>, String> {
    execute_str(&mut context.state, proc_source, cmd_source)?;
    Ok(Vec::from(context.state.state.delegate.dt.get_data_u8()))
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
