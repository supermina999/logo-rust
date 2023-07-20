use crate::common::*;

pub trait Delegates {
    fn clear_graphics(&mut self);
    fn draw_line(&mut self, from: Pos, to: Pos, pen_size: f64, color: (u8, u8, u8));
    fn fill(&mut self, pos: Pos, color: (u8, u8, u8));
}

pub enum PenState {
    Up,
    Down,
    Erase
}

pub struct State<UiState> {
    pub turtle_pos: Pos,
    pub turtle_angle: f64,
    pub turtle_visible: bool,
    pub pen_state: PenState,
    pub pen_size: f64,
    pub color_idx: i32,

    pub delegates: Box<dyn Delegates>,
    pub ui_state: UiState
}

impl<UiState> State<UiState> {
    pub fn new(delegates: Box<dyn Delegates>, ui_state: UiState) -> Self {
        State {
            turtle_pos: Pos{x: 0f64, y: 0f64},
            turtle_angle: 0f64,
            turtle_visible: true,
            pen_state: PenState::Down,
            pen_size: 1f64,
            color_idx: 0,
            delegates,
            ui_state,
        }
    }
}
