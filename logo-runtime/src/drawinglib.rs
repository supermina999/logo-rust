use logo_interp::executor_state::*;
use crate::colors::{LogoColor, colors_count, get_color};
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
    state.data.turtle_pos = Pos{x: 0f64, y: 0f64};
    state.data.turtle_angle = 0f64;
    state.delegate.clear_graphics();
    Ok(())
}

fn clean(state: &mut EState<State>) -> Result<(), String> {
    state.state.delegate.clear_graphics();
    Ok(())
}

fn fill(state: &mut EState<State>) -> Result<(), String> {
    let state = &mut state.state;
    state.delegate.fill(state.data.turtle_pos, get_color(state.data.color_idx));
    Ok(())
}

fn pu(state: &mut EState<State>) -> Result<(), String> {
    state.state.data.pen_state = PenState::Up;
    Ok(())
}

fn pd(state: &mut EState<State>) -> Result<(), String> {
    state.state.data.pen_state = PenState::Down;
    Ok(())
}

fn pe(state: &mut EState<State>) -> Result<(), String> {
    state.state.data.pen_state = PenState::Erase;
    Ok(())
}

fn rt(state: &mut EState<State>, val: f64) -> Result<(), String> {
    state.state.data.turtle_angle += val;
    Ok(())
}

fn lt(state: &mut EState<State>, val: f64) -> Result<(), String> {
    state.state.data.turtle_angle -= val;
    Ok(())
}

fn fd(state: &mut EState<State>, val: f64) -> Result<(), String> {
    let old_pos = state.state.data.turtle_pos;
    let angle = state.state.data.turtle_angle;
    let delta_x = angle.to_radians().sin() * val;
    let delta_y = angle.to_radians().cos() * val;
    let new_pos = Pos{x: old_pos.x + delta_x, y: old_pos.y + delta_y};
    move_turtle(&mut state.state, new_pos);
    Ok(())
}

fn bk(state: &mut EState<State>, val: f64) -> Result<(), String> {
    fd(state, -val)
}

fn heading(state: &mut EState<State>) -> Result<f64, String> {
    Ok(state.state.data.turtle_angle)
}

fn seth(state: &mut EState<State>, h: f64) -> Result<(), String> {
    state.state.data.turtle_angle = h;
    Ok(())
}

fn pos(state: &mut EState<State>) -> Result<Vec<f64>, String> {
    Ok(vec![state.state.data.turtle_pos.x, state.state.data.turtle_pos.y])
}

fn setpos(state: &mut EState<State>, pos: Vec<f64>) -> Result<(), String> {
    if pos.len() != 2 {
        Err("Setpos takes exactly 2 coordinates".to_string())
    }
    else {
        move_turtle(&mut state.state, Pos{ x: pos[0], y: pos[1] });
        Ok(())
    }
}

fn xcoor(state: &mut EState<State>) -> Result<f64, String> {
    Ok(state.state.data.turtle_pos.x)
}

fn ycoor(state: &mut EState<State>) -> Result<f64, String> {
    Ok(state.state.data.turtle_pos.y)
}

fn setx(state: &mut EState<State>, x: f64) -> Result<(), String> {
    let y = state.state.data.turtle_pos.y;
    move_turtle(&mut state.state, Pos{x, y});
    Ok(())
}

fn sety(state: &mut EState<State>, y: f64) -> Result<(), String> {
    let x = state.state.data.turtle_pos.x;
    move_turtle(&mut state.state, Pos{x, y});
    Ok(())
}

fn home(state: &mut EState<State>) -> Result<(), String> {
    move_turtle(&mut state.state, Pos{ x: 0f64, y: 0f64 });
    Ok(())
}

fn setpensize(state: &mut EState<State>, pen_size: f64) -> Result<(), String> {
    state.state.data.pen_size = pen_size;
    Ok(())
}

fn pensize(state: &mut EState<State>) -> Result<f64, String> {
    Ok(state.state.data.pen_size)
}

fn ht(state: &mut EState<State>) -> Result<(), String> {
    state.state.data.turtle_visible = false;
    Ok(())
}

fn st(state: &mut EState<State>) -> Result<(), String> {
    state.state.data.turtle_visible = true;
    Ok(())
}

fn setc(state: &mut EState<State>, color: i32) -> Result<(), String> {
    if color < 0 || color >= colors_count() {
        return Err("Invalid color number".to_string());
    }
    state.state.data.color_idx = color;
    Ok(())
}

fn color(state: &mut EState<State>) -> Result<i32, String> {
    Ok(state.state.data.color_idx)
}

fn move_turtle(state: &mut State, pos: Pos) {
    let old_pos = state.data.turtle_pos;
    let w2 = state.data.canvas_width as f64 / 2f64;
    let h2 = state.data.canvas_height as f64 / 2f64;
    if pos.y > old_pos.y + f64::EPSILON {
        let xp = intersect_horizontal(old_pos, pos, h2, -w2, w2);
        if xp.is_some() {
            draw_line(state, old_pos, Pos{x: xp.unwrap(), y: h2});
            state.data.turtle_pos = Pos{x: xp.unwrap(), y: -h2};
            move_turtle(state, Pos{x: pos.x, y: pos.y - state.data.canvas_height as f64});
            return;
        }
    }
    if pos.y + f64::EPSILON < old_pos.y {
        let xp = intersect_horizontal(old_pos, pos, -h2, -w2, w2);
        if xp.is_some() {
            draw_line(state, old_pos, Pos{x: xp.unwrap(), y: -h2});
            state.data.turtle_pos = Pos{x: xp.unwrap(), y: h2};
            move_turtle(state, Pos{x: pos.x, y: pos.y + state.data.canvas_height as f64});
            return;
        }
    }
    if pos.x > old_pos.x + f64::EPSILON {
        let yp = intersect_vertical(old_pos, pos, w2, -h2, h2);
        if yp.is_some() {
            draw_line(state, old_pos, Pos{x: w2, y: yp.unwrap()});
            state.data.turtle_pos = Pos{x: -w2, y: yp.unwrap()};
            move_turtle(state, Pos{x: pos.x - state.data.canvas_width as f64, y: pos.y});
            return;
        }
    }
    if pos.x + f64::EPSILON < old_pos.x {
        let yp = intersect_vertical(old_pos, pos, -w2, -h2, h2);
        if yp.is_some() {
            draw_line(state, old_pos, Pos{x: -w2, y: yp.unwrap()});
            state.data.turtle_pos = Pos{x: w2, y: yp.unwrap()};
            move_turtle(state, Pos{x: pos.x + state.data.canvas_width as f64, y: pos.y});
            return;
        }
    }
    state.data.turtle_pos = pos;
    draw_line(state, old_pos, pos);
}

fn draw_line(state: &mut State, p1: Pos, p2: Pos) {
    let mut color = get_color(state.data.color_idx);
    if state.data.pen_state == PenState::Erase {
        color = LogoColor{r: 255, g: 255, b: 255};
    }
    if state.data.pen_state != PenState::Up {
        state.delegate.draw_line(p1, p2, state.data.pen_size, color);
    }
}

fn intersect_horizontal(p1: Pos, p2: Pos, y: f64, x1: f64, x2: f64) -> Option<f64> {
    if p1.y.min(p2.y) > y || p1.y.max(p2.y) < y {
        return None;
    }
    let xp = p1.x - (p1.y - y) / (p1.y - p2.y) * (p1.x - p2.x);
    if xp >= x1 && xp <= x2 {
        Some(xp)
    }
    else {
        None
    }
}

fn intersect_vertical(p1: Pos, p2: Pos, x: f64, y1: f64, y2: f64) -> Option<f64> {
    if p1.x.min(p2.x) > x || p1.x.max(p2.x) < x {
        return None;
    }
    let yp = p1.y - (p1.x - x) / (p1.x - p2.x) * (p1.y - p2.y);
    if yp >= y1 && yp <= y2 {
        Some(yp)
    }
    else {
        None
    }
}

#[test]
fn test_move_turtle() {
    use crate::state::NoOpDelegate;
    use approx::assert_relative_eq;

    let mut state = EState::new(State::new(800, 450, Box::new(NoOpDelegate{})));
    fd(&mut state, 50.0).unwrap();
    assert_relative_eq!(state.state.data.turtle_pos.y, 50.0, epsilon = 0.00001);
    fd(&mut state, 400.0).unwrap();
    assert_relative_eq!(state.state.data.turtle_pos.y, 0.0, epsilon = 0.00001);
    rt(&mut state, 90.0).unwrap();
    fd(&mut state, 500.0).unwrap();
    assert_relative_eq!(state.state.data.turtle_pos.x, -300.0, epsilon = 0.00001);
}