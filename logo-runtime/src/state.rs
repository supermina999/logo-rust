use wasm_bindgen::prelude::*;
use crate::colors::LogoColor;
use crate::common::*;

pub trait Delegate {
    fn clear_graphics(&mut self);
    fn draw_line(&mut self, from: Pos, to: Pos, pen_size: f64, color: LogoColor);
    fn fill(&mut self, pos: Pos, color: LogoColor);
}

pub struct NoOpDelegate {}

impl Delegate for NoOpDelegate {
    fn clear_graphics(&mut self) {}
    fn draw_line(&mut self, _from: Pos, _to: Pos, _pen_size: f64, _color: LogoColor) {}
    fn fill(&mut self, _pos: Pos, _color: LogoColor) {}
}

#[wasm_bindgen]
#[derive(Copy, Clone, PartialEq)]
pub enum PenState {
    Up,
    Down,
    Erase
}

#[wasm_bindgen]
#[derive(Copy, Clone)]
pub struct StateData {
    pub canvas_width: i32,
    pub canvas_height: i32,
    pub turtle_pos: Pos,
    pub turtle_angle: f64,
    pub turtle_visible: bool,
    pub pen_state: PenState,
    pub pen_size: f64,
    pub color_idx: i32,
}

pub struct State {
    pub data: StateData,
    pub delegate: Box<dyn Delegate>,
}

impl State {
    pub fn new(canvas_width: i32, canvas_height: i32, delegate: Box<dyn Delegate>) -> Self {
        State {
            data: StateData {
                canvas_width,
                canvas_height,
                turtle_pos: Pos { x: 0f64, y: 0f64 },
                turtle_angle: 0f64,
                turtle_visible: true,
                pen_state: PenState::Down,
                pen_size: 1f64,
                color_idx: 9,
            },
            delegate
        }
    }
}
