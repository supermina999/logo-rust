use std::fs;
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

fn render(logo_source: &str, out_path: &str) {
    let dt = Rc::new(RefCell::new(DrawTarget::new(800, 450)));
    let dd = DrawingDelegate{ dt: dt.clone() };
    let mut state = EState::new(State::new(Box::new(dd)));
    state.state.delegate.clear_graphics();
    add_stdlib(&mut state);
    add_drawinglib(&mut state);
    let result = execute_str(&mut state, logo_source);
    if let Err(err) = result {
        panic!("Error occurred while executing: {}", err);
    }

    dt.borrow_mut().write_png(out_path).expect("Failed to write file");
}

fn main() {
    let logo_path = std::env::args().nth(1).expect("Please provide path to a file with Logo code");
    let out_path = std::env::args().nth(2).expect("Please provide output path");
    let logo_source = fs::read_to_string(logo_path).expect("Failed to open file");
    render(logo_source.as_str(), out_path.as_str());
}
