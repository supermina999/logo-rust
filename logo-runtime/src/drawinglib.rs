use logo_interp::executor_state::*;
use crate::colors::{colors_count, get_color};
use crate::common::Pos;
use crate::state::{PenState, State};

pub fn add_drawinglib(es: &mut EState<State>) {
    es.functions.insert("cg".to_string(), Function::from_proc(cg));
    es.functions.insert("clean".to_string(), Function::from_proc(clean));
    es.functions.insert("fill".to_string(), Function::from_proc(fill));

    es.functions.insert("pu".to_string(), Function::from_proc(pu));
    es.functions.insert("pd".to_string(), Function::from_proc(pd));
    es.functions.insert("pe".to_string(), Function::from_proc(pe));

    es.functions.insert("rt".to_string(), Function::from_proc1(rt));
    es.functions.insert("right".to_string(), Function::from_proc1(rt));
    es.functions.insert("lt".to_string(), Function::from_proc1(lt));
    es.functions.insert("left".to_string(), Function::from_proc1(lt));
    es.functions.insert("fd".to_string(), Function::from_proc1(fd));
    es.functions.insert("bk".to_string(), Function::from_proc1(bk));

    es.functions.insert("heading".to_string(), Function::from_fn(heading));
    es.functions.insert("seth".to_string(), Function::from_proc1(seth));
    es.functions.insert("setheading".to_string(), Function::from_proc1(seth));
    es.functions.insert("setpos".to_string(), Function::from_proc1(setpos));
    es.functions.insert("setx".to_string(), Function::from_proc1(setx));
    es.functions.insert("sety".to_string(), Function::from_proc1(sety));
    es.functions.insert("pos".to_string(), Function::from_fn(pos));
    es.functions.insert("xcoor".to_string(), Function::from_fn(xcoor));
    es.functions.insert("ycoor".to_string(), Function::from_fn(ycoor));
    es.functions.insert("home".to_string(), Function::from_proc(home));

    es.functions.insert("pensize".to_string(), Function::from_fn(pensize));
    es.functions.insert("setpensize".to_string(), Function::from_proc1(setpensize));

    es.functions.insert("ht".to_string(), Function::from_proc(ht));
    es.functions.insert("st".to_string(), Function::from_proc(st));

    es.functions.insert("setc".to_string(), Function::from_proc1(setc));
    es.functions.insert("setcolor".to_string(), Function::from_proc1(setc));
    es.functions.insert("color".to_string(), Function::from_fn(color));
}

fn cg(state: &mut EState<State>) -> Result<(), String> {
    let state = &mut state.state;
    state.turtle_pos = Pos{x: 0f64, y: 0f64};
    state.turtle_angle = 0f64;
    state.delegate.clear_graphics();
    Ok(())
}

fn clean(state: &mut EState<State>) -> Result<(), String> {
    state.state.delegate.clear_graphics();
    Ok(())
}

fn fill(state: &mut EState<State>) -> Result<(), String> {
    let state = &mut state.state;
    state.delegate.fill(state.turtle_pos, get_color(state.color_idx));
    Ok(())
}

fn pu(state: &mut EState<State>) -> Result<(), String> {
    state.state.pen_state = PenState::Up;
    Ok(())
}

fn pd(state: &mut EState<State>) -> Result<(), String> {
    state.state.pen_state = PenState::Down;
    Ok(())
}

fn pe(state: &mut EState<State>) -> Result<(), String> {
    state.state.pen_state = PenState::Erase;
    Ok(())
}

fn rt(state: &mut EState<State>, val: f64) -> Result<(), String> {
    state.state.turtle_angle += val;
    Ok(())
}

fn lt(state: &mut EState<State>, val: f64) -> Result<(), String> {
    state.state.turtle_angle -= val;
    Ok(())
}

fn fd(state: &mut EState<State>, val: f64) -> Result<(), String> {
    let old_pos = state.state.turtle_pos;
    let angle = state.state.turtle_angle;
    let delta_x = angle.to_radians().sin() * val;
    let delta_y = angle.to_radians().cos() * val;
    let new_pos = Pos{x: old_pos.x + delta_x, y: old_pos.y + delta_y};
    move_turtle(state, new_pos);
    Ok(())
}

fn bk(state: &mut EState<State>, val: f64) -> Result<(), String> {
    fd(state, -val)
}

fn heading(state: &mut EState<State>) -> Result<f64, String> {
    Ok(state.state.turtle_angle)
}

fn seth(state: &mut EState<State>, h: f64) -> Result<(), String> {
    state.state.turtle_angle = h;
    Ok(())
}

fn pos(state: &mut EState<State>) -> Result<Vec<f64>, String> {
    Ok(vec![state.state.turtle_pos.x, state.state.turtle_pos.y])
}

fn setpos(state: &mut EState<State>, pos: Vec<f64>) -> Result<(), String> {
    if pos.len() != 2 {
        Err("Setpos takes exactly 2 coordinates".to_string())
    }
    else {
        move_turtle(state, Pos{ x: pos[0], y: pos[1] });
        Ok(())
    }
}

fn xcoor(state: &mut EState<State>) -> Result<f64, String> {
    Ok(state.state.turtle_pos.x)
}

fn ycoor(state: &mut EState<State>) -> Result<f64, String> {
    Ok(state.state.turtle_pos.y)
}

fn setx(state: &mut EState<State>, x: f64) -> Result<(), String> {
    move_turtle(state, Pos{x, y: state.state.turtle_pos.y});
    Ok(())
}

fn sety(state: &mut EState<State>, y: f64) -> Result<(), String> {
    move_turtle(state, Pos{x: state.state.turtle_pos.y, y});
    Ok(())
}

fn home(state: &mut EState<State>) -> Result<(), String> {
    move_turtle(state, Pos{ x: 0f64, y: 0f64 });
    Ok(())
}

fn move_turtle(state: &mut EState<State>, pos: Pos) {
    let state = &mut state.state;
    let old_pos = state.turtle_pos;
    state.turtle_pos = pos;
    let color = get_color(state.color_idx);
    match state.pen_state {
        PenState::Down => state.delegate.draw_line(old_pos, pos, state.pen_size, color),
        PenState::Erase => state.delegate.draw_line(old_pos, pos, state.pen_size, (255u8, 255u8, 255u8)),
        _ => {}
    }
}

fn setpensize(state: &mut EState<State>, pen_size: f64) -> Result<(), String> {
    state.state.pen_size = pen_size;
    Ok(())
}

fn pensize(state: &mut EState<State>) -> Result<f64, String> {
    Ok(state.state.pen_size)
}

fn ht(state: &mut EState<State>) -> Result<(), String> {
    state.state.turtle_visible = false;
    Ok(())
}

fn st(state: &mut EState<State>) -> Result<(), String> {
    state.state.turtle_visible = true;
    Ok(())
}

fn setc(state: &mut EState<State>, color: i32) -> Result<(), String> {
    if color < 0 || color >= colors_count() {
        return Err("Invalid color number".to_string());
    }
    state.state.color_idx = color;
    Ok(())
}

fn color(state: &mut EState<State>) -> Result<i32, String> {
    Ok(state.state.color_idx)
}
