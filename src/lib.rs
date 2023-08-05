//#![no_std]

const WIDTH: usize = 10;
const HEIGHT: usize = 9;

mod bresenham;

#[inline]
fn mixpixel(position: (usize, usize), value: (u8, u8, u8)) {
    unsafe {
        display[position.0][position.1].0 =
            display[position.0][position.1].0.saturating_add(value.0);
        display[position.0][position.1].1 =
            display[position.0][position.1].1.saturating_add(value.1);
        display[position.0][position.1].2 =
            display[position.0][position.1].2.saturating_add(value.2);
    }
}

#[inline]
fn overwritepixel(position: (usize, usize), value: (u8, u8, u8)) {
    unsafe {
        display[position.0][position.1] = value;
    }
}

#[allow(non_upper_case_globals)]
static mut display: [[(u8, u8, u8); HEIGHT]; WIDTH] = [[(0, 0, 0); HEIGHT]; WIDTH];

pub type Vectormap<'a> = &'a [((u8, u8, u8), &'a [((usize, usize), (usize, usize))])];

pub fn render_vectormap(map: Vectormap) {
    for (color, array) in map {
        for points in *array {
            draw_line_internal(points.0, points.1, *color, overwritepixel);
        }
    }
}

pub fn buffer() -> &'static [[(u8, u8, u8); HEIGHT]; WIDTH] {
    unsafe { &display }
}

pub fn draw_bitmap(map: &[&[(u8, u8, u8)]], position: (usize, usize)) {
    let mut xindex = position.0;
    let mut yindex = position.1;
    let mut xcounter = 0;
    let mut ycounter = 0;
    while xindex < WIDTH && xcounter < map.len() {
        while yindex < HEIGHT && ycounter < map[0].len() {
            unsafe {
                display[xindex][yindex] = map[xcounter][ycounter];
            }
            yindex += 1;
            ycounter += 1;
        }
        yindex = 0;
        ycounter = 0;
        xindex += 1;
        xcounter += 1;
    }
}

pub fn draw_line(point1: (usize, usize), point2: (usize, usize), color: (u8, u8, u8)) {
    draw_line_internal(point1, point2, color, mixpixel);
}

fn draw_line_internal(point1: (usize, usize), point2: (usize, usize), color: (u8, u8, u8), render_func: fn(position: (usize, usize), value: (u8, u8, u8))) {
    bresenham::draw_line(
        (point1.0 as isize, point1.1 as isize),
        (point2.0 as isize, point2.1 as isize),
        color,
        render_func,
    )
}

