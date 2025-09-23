pub struct Box {
    pub min_x: i32,
    pub min_y: i32,
    pub width: i32,
    pub height: i32,
}

pub fn ring(x: i32, y: i32, half_size: i32, side: i32) -> [Box; 4] {
    let size = half_size * 2;
    let long_side = side + size;

    [
        Box {
            min_x: x + half_size,
            min_y: y - half_size,
            width: side,
            height: long_side,
        },
        Box {
            min_x: (x + half_size) - long_side,
            min_y: y + half_size,
            width: long_side,
            height: side,
        },
        Box {
            min_x: (x - half_size) - side,
            min_y: (y - half_size) - side,
            width: side,
            height: long_side,
        },
        Box {
            min_x: (x - half_size),
            min_y: (y - half_size) - side,
            width: long_side,
            height: side,
        },
    ]
}
