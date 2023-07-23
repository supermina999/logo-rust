mod fill;

use wasm_bindgen::prelude::*;

pub use logo_runtime;

use std::cell::RefCell;
use std::rc::Rc;
use raqote::*;
use logo_runtime::colors::LogoColor;
use logo_runtime::common::Pos;
use logo_runtime::drawinglib::add_drawinglib;
use logo_runtime::logo_interp::executor::execute_str;
use logo_runtime::logo_interp::executor_state::EState;
use logo_runtime::logo_interp::stdlib::add_stdlib;
use logo_runtime::state::{Delegate, State, StateData};
use crate::fill::flood_fill;

struct DrawingDelegate {
    dt: Rc<RefCell<DrawTarget>>,
    #[cfg(target_arch = "wasm32")]
    show_fn: Rc<RefCell<Option<js_sys::Function>>>
}

impl DrawingDelegate {
    fn transform_coords(&self, pos: Pos) -> (f32, f32) {
        let width = self.dt.borrow().width() as f64;
        let height = self.dt.borrow().height() as f64;
        ((pos.x + width / 2f64 + 0.5) as f32, (-pos.y + height / 2f64 + 0.5) as f32)
    }
}

impl Delegate for DrawingDelegate {
    fn clear_graphics(&mut self) {
        self.dt.borrow_mut().clear(SolidSource{
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
        self.dt.borrow_mut().stroke(&path, &Source::Solid(SolidSource {
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
        let mut dt_mut = self.dt.borrow_mut();
        flood_fill(dt_mut.width(), dt_mut.height(), dt_mut.get_data_u8_mut(),
            upd_pos.0 as i32, upd_pos.1 as i32, color);
    }

    fn show(&mut self, _message: &str) {
        #[cfg(target_arch = "wasm32")] {
            if let Some(show_fn) = self.show_fn.borrow().as_ref() {
                let this = JsValue::null();
                let _ = show_fn.call1(&this, &JsValue::from(_message));
            }
        }
    }
}

#[wasm_bindgen]
pub struct Context {
    #[wasm_bindgen(skip)]
    pub dt: Rc<RefCell<DrawTarget>>,
    #[wasm_bindgen(skip)]
    pub state: EState<State>,
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen(skip)]
    pub show_fn: Rc<RefCell<Option<js_sys::Function>>>
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
    let dt = Rc::new(RefCell::new(DrawTarget::new(width, height)));
    #[cfg(target_arch = "wasm32")]
    let show_fn = Rc::new(RefCell::new(None));
    #[cfg(target_arch = "wasm32")]
    let dd = DrawingDelegate { dt: dt.clone(), show_fn: show_fn.clone() };
    #[cfg(not(target_arch = "wasm32"))]
    let dd = DrawingDelegate { dt: dt.clone() };
    let mut state = EState::new(State::new(width, height, Box::new(dd)));
    state.state.delegate.clear_graphics();
    add_stdlib(&mut state);
    add_drawinglib(&mut state);
    #[cfg(target_arch = "wasm32")]
    return Context {dt, state, show_fn};
    #[cfg(not(target_arch = "wasm32"))]
    return Context {dt, state}
}

#[wasm_bindgen]
pub fn context_render(context: &mut Context, proc_source: &str, cmd_source: &str) -> Result<Vec<u8>, String> {
    execute_str(&mut context.state, proc_source, cmd_source)?;
    let dt_mut = context.dt.borrow_mut();
    Ok(Vec::from(dt_mut.get_data_u8()))
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
pub fn context_set_show_fn(context: &mut Context, f: &js_sys::Function) {
    *context.show_fn.borrow_mut() = Some(f.clone());
}
