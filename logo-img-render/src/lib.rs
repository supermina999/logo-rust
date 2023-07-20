use wasm_bindgen::prelude::*;

pub use logo_runtime;

use std::cell::RefCell;
use std::rc::Rc;
use raqote::*;
use logo_runtime::common::Pos;
use logo_runtime::drawinglib::add_drawinglib;
use logo_runtime::logo_interp::executor::execute_str;
use logo_runtime::logo_interp::executor_state::EState;
use logo_runtime::logo_interp::stdlib::add_stdlib;
use logo_runtime::state::{Delegate, State};

struct DrawingDelegate {
    dt: Rc<RefCell<DrawTarget>>
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

    fn draw_line(&mut self, from: Pos, to: Pos, pen_size: f64, color: (u8, u8, u8)) {
        let upd_from = self.transform_coords(from);
        let upd_to = self.transform_coords(to);
        let mut pb = PathBuilder::new();
        pb.move_to(upd_from.0, upd_from.1);
        pb.line_to(upd_to.0, upd_to.1);
        let path = pb.finish();
        self.dt.borrow_mut().stroke(&path, &Source::Solid(SolidSource {
                r: color.0,
                g: color.1,
                b: color.2,
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

    fn fill(&mut self, pos: Pos, color: (u8, u8, u8)) {
        todo!()
    }
}

pub struct Context {
    pub dt: Rc<RefCell<DrawTarget>>,
    pub state: EState<State>
}

impl Context {
    pub fn new(width: i32, height: i32) -> Self {
        let dt = Rc::new(RefCell::new(DrawTarget::new(width, height)));
        let dd = DrawingDelegate{ dt: dt.clone() };
        let mut state = EState::new(State::new(Box::new(dd)));
        state.state.delegate.clear_graphics();
        add_stdlib(&mut state);
        add_drawinglib(&mut state);
        Self {dt, state}
    }

    pub fn render(&mut self, proc_source: &str, cmd_source: &str) -> Result<Vec<u8>, String> {
        execute_str(&mut self.state, proc_source, cmd_source)?;
        let dt_mut = self.dt.borrow_mut();
        Ok(Vec::from(dt_mut.get_data_u8()))
    }
}

#[wasm_bindgen]
pub fn render(proc_source: &str, cmd_source: &str, width: i32, height: i32) -> Result<Vec<u8>, String> {
    let mut context = Context::new(width, height);
    context.render(proc_source, cmd_source)
}
