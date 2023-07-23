use logo_runtime::colors::LogoColor;

pub fn flood_fill(width: i32, height: i32, data: &mut [u8], x: i32, y: i32, color: LogoColor) {
    let get_color = |data: &[u8], x: i32, y: i32| -> LogoColor {
        let pos = ((x + y * width) * 4) as usize;
        LogoColor{r: data[pos], g: data[pos+1], b: data[pos+2]}
    };
    let set_color = |data: &mut [u8], x: i32, y: i32| {
        let pos = ((x + y * width) * 4) as usize;
        data[pos] = color.r;
        data[pos+1] = color.g;
        data[pos+2] = color.b;
    };
    let push_next = |queue: &mut Vec<(i32, i32)>, used: &mut Vec<bool>, x: i32, y: i32| {
        if x < 0 || x >= width || y < 0 || y >= height {
            return;
        }
        if used[(x + y * width) as usize] {
            return;
        }
        used[(x + y * width) as usize] = true;
        queue.push((x, y));
    };

    let orig_color = get_color(data, x, y);
    let mut used = vec![false; (width * height) as usize];
    let mut queue = Vec::new();
    push_next(&mut queue, &mut used, x, y);
    while !queue.is_empty() {
        let cur = queue.pop().unwrap();
        let cur_color = get_color(data, cur.0, cur.1);
        if cur_color != orig_color {
            continue;
        }
        set_color(data, cur.0, cur.1);
        push_next(&mut queue, &mut used, cur.0 - 1, cur.1);
        push_next(&mut queue, &mut used, cur.0 + 1, cur.1);
        push_next(&mut queue, &mut used, cur.0, cur.1 - 1);
        push_next(&mut queue, &mut used, cur.0, cur.1 + 1);
    }
}
