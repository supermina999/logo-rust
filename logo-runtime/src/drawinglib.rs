use logo_interp::executor_state::*;
use crate::colors::{colors_count, get_color};
use crate::common::Pos;
use crate::state::{PenState, State};

pub fn add_drawinglib<S: 'static>(es: &mut EState<State<S>>) {
    es.functions.insert("cg".to_string(), Function::from_proc(cg::<S>));
    es.functions.insert("clean".to_string(), Function::from_proc(clean::<S>));
    es.functions.insert("fill".to_string(), Function::from_proc(fill::<S>));

    es.functions.insert("pu".to_string(), Function::from_proc(pu::<S>));
    es.functions.insert("pd".to_string(), Function::from_proc(pd::<S>));
    es.functions.insert("pe".to_string(), Function::from_proc(pe::<S>));

    es.functions.insert("rt".to_string(), Function::from_proc1(rt::<S>));
    es.functions.insert("right".to_string(), Function::from_proc1(rt::<S>));
    es.functions.insert("lt".to_string(), Function::from_proc1(lt::<S>));
    es.functions.insert("left".to_string(), Function::from_proc1(lt::<S>));
    es.functions.insert("fd".to_string(), Function::from_proc1(fd::<S>));
    es.functions.insert("bk".to_string(), Function::from_proc1(bk::<S>));

    es.functions.insert("heading".to_string(), Function::from_fn(heading::<S>));
    es.functions.insert("seth".to_string(), Function::from_proc1(seth::<S>));
    es.functions.insert("setheading".to_string(), Function::from_proc1(seth::<S>));
    es.functions.insert("setpos".to_string(), Function::from_proc1(setpos::<S>));
    es.functions.insert("setx".to_string(), Function::from_proc1(setx::<S>));
    es.functions.insert("sety".to_string(), Function::from_proc1(sety::<S>));
    es.functions.insert("pos".to_string(), Function::from_fn(pos::<S>));
    es.functions.insert("xcoor".to_string(), Function::from_fn(xcoor::<S>));
    es.functions.insert("ycoor".to_string(), Function::from_fn(ycoor::<S>));
    es.functions.insert("home".to_string(), Function::from_proc(home::<S>));

    es.functions.insert("pensize".to_string(), Function::from_fn(pensize::<S>));
    es.functions.insert("setpensize".to_string(), Function::from_proc1(setpensize::<S>));

    es.functions.insert("ht".to_string(), Function::from_proc(ht::<S>));
    es.functions.insert("st".to_string(), Function::from_proc(st::<S>));

    es.functions.insert("setc".to_string(), Function::from_proc1(setc::<S>));
    es.functions.insert("setcolor".to_string(), Function::from_proc1(setc::<S>));
    es.functions.insert("color".to_string(), Function::from_fn(color::<S>));
}

fn cg<S>(state: &mut EState<State<S>>) -> Result<(), String> {
    let state = &mut state.state;
    state.turtle_pos = Pos{x: 0f64, y: 0f64};
    state.turtle_angle = 0f64;
    state.delegates.clear_graphics();
    Ok(())
}

fn clean<S>(state: &mut EState<State<S>>) -> Result<(), String> {
    state.state.delegates.clear_graphics();
    Ok(())
}

fn fill<S>(state: &mut EState<State<S>>) -> Result<(), String> {
    let state = &mut state.state;
    state.delegates.fill(state.turtle_pos, get_color(state.color_idx));
    Ok(())
}

fn pu<S>(state: &mut EState<State<S>>) -> Result<(), String> {
    state.state.pen_state = PenState::Up;
    Ok(())
}

fn pd<S>(state: &mut EState<State<S>>) -> Result<(), String> {
    state.state.pen_state = PenState::Down;
    Ok(())
}

fn pe<S>(state: &mut EState<State<S>>) -> Result<(), String> {
    state.state.pen_state = PenState::Erase;
    Ok(())
}

fn rt<S>(state: &mut EState<State<S>>, val: f64) -> Result<(), String> {
    state.state.turtle_angle += val;
    Ok(())
}

fn lt<S>(state: &mut EState<State<S>>, val: f64) -> Result<(), String> {
    state.state.turtle_angle -= val;
    Ok(())
}

fn fd<S>(state: &mut EState<State<S>>, val: f64) -> Result<(), String> {
    let old_pos = state.state.turtle_pos;
    let angle = state.state.turtle_angle;
    let delta_x = angle.to_radians().sin() * val;
    let delta_y = angle.to_radians().cos() * val;
    let new_pos = Pos{x: old_pos.x + delta_x, y: old_pos.y + delta_y};
    move_turtle(state, new_pos);
    Ok(())
}

fn bk<S>(state: &mut EState<State<S>>, val: f64) -> Result<(), String> {
    fd(state, -val)
}

fn heading<S>(state: &mut EState<State<S>>) -> Result<f64, String> {
    Ok(state.state.turtle_angle)
}

fn seth<S>(state: &mut EState<State<S>>, h: f64) -> Result<(), String> {
    state.state.turtle_angle = h;
    Ok(())
}

fn pos<S>(state: &mut EState<State<S>>) -> Result<Vec<f64>, String> {
    Ok(vec![state.state.turtle_pos.x, state.state.turtle_pos.y])
}

fn setpos<S>(state: &mut EState<State<S>>, pos: Vec<f64>) -> Result<(), String> {
    if pos.len() != 2 {
        Err("Setpos takes exactly 2 coordinates".to_string())
    }
    else {
        move_turtle(state, Pos{ x: pos[0], y: pos[1] });
        Ok(())
    }
}

fn xcoor<S>(state: &mut EState<State<S>>) -> Result<f64, String> {
    Ok(state.state.turtle_pos.x)
}

fn ycoor<S>(state: &mut EState<State<S>>) -> Result<f64, String> {
    Ok(state.state.turtle_pos.y)
}

fn setx<S>(state: &mut EState<State<S>>, x: f64) -> Result<(), String> {
    move_turtle(state, Pos{x, y: state.state.turtle_pos.y});
    Ok(())
}

fn sety<S>(state: &mut EState<State<S>>, y: f64) -> Result<(), String> {
    move_turtle(state, Pos{x: state.state.turtle_pos.y, y});
    Ok(())
}

fn home<S>(state: &mut EState<State<S>>) -> Result<(), String> {
    move_turtle(state, Pos{ x: 0f64, y: 0f64 });
    Ok(())
}

fn move_turtle<S>(state: &mut EState<State<S>>, pos: Pos) {
    let state = &mut state.state;
    let old_pos = state.turtle_pos;
    state.turtle_pos = pos;
    let color = get_color(state.color_idx);
    match state.pen_state {
        PenState::Down => state.delegates.draw_line(old_pos, pos, state.pen_size, color),
        PenState::Erase => state.delegates.draw_line(old_pos, pos, state.pen_size, (255u8, 255u8, 255u8)),
        _ => {}
    }
}

fn setpensize<S>(state: &mut EState<State<S>>, pen_size: f64) -> Result<(), String> {
    state.state.pen_size = pen_size;
    Ok(())
}

fn pensize<S>(state: &mut EState<State<S>>) -> Result<f64, String> {
    Ok(state.state.pen_size)
}

fn ht<S>(state: &mut EState<State<S>>) -> Result<(), String> {
    state.state.turtle_visible = false;
    Ok(())
}

fn st<S>(state: &mut EState<State<S>>) -> Result<(), String> {
    state.state.turtle_visible = true;
    Ok(())
}

fn setc<S>(state: &mut EState<State<S>>, color: i32) -> Result<(), String> {
    if color < 0 || color >= colors_count() {
        return Err("Invalid color number".to_string());
    }
    state.state.color_idx = color;
    Ok(())
}

fn color<S>(state: &mut EState<State<S>>) -> Result<i32, String> {
    Ok(state.state.color_idx)
}
