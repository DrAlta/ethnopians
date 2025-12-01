pub struct Box {
    pub min_x: i32,
    pub min_y: i32,
    pub width: i32,
    pub height: i32,
}
/// Generates four `Box` structs arranged in a ring shape around a central point `(x, y)`.
///
/// The function creates four rectangles that form a hollow square (or "ring") with the specified dimensions.
///
/// # Arguments
///
/// * `x`: The x-coordinate of the center of the ring.
/// * `y`: The y-coordinate of the center of the ring.
/// * `half_size`: Defines the inner boundary of the ring (half the width of the inner empty space).
/// * `side`: The width/thickness of each rectangular segment that makes up the ring.
///
/// # Returns
///
/// An array `[Box; 4]` representing the top-right, bottom-right, bottom-left, and top-left sections of the ring.
///
/// # Layout Details
///
/// The four boxes create an outline around an inner square defined by:
/// * min_x: `x - half_size`
/// * min_y: `y - half_size`
/// * max_x: `x + half_size`
/// * max_y: `y + half_size`
///
/// Each box has dimensions relative to the inner boundary and the `side` thickness.
///
/// # Examples
///
/// ```rust
/// # pub struct Box { pub min_x: i32, pub min_y: i32, pub width: i32, pub height: i32 }
/// # pub fn ring(x: i32, y: i32, half_size: i32, side: i32) -> [Box; 4] {
/// #    let size = half_size * 2;
/// #    let long_side = side + size;
/// #    [
/// #        Box { min_x: x + half_size, min_y: y - half_size, width: side, height: long_side },
/// #        Box { min_x: (x + half_size) - long_side, min_y: y + half_size, width: long_side, height: side },
/// #        Box { min_x: (x - half_size) - side, min_y: (y - half_size) - side, width: side, height: long_side },
/// #        Box { min_x: (x - half_size), min_y: (y - half_size) - side, width: long_side, height: side },
/// #    ]
/// # }
/// let ring_boxes = ring(0, 0, 10, 5);
/// // The first box is the right-side vertical segment:
/// assert_eq!(ring_boxes[0].min_x, 10);
/// assert_eq!(ring_boxes[0].min_y, -10);
/// assert_eq!(ring_boxes[0].width, 5);
/// assert_eq!(ring_boxes[0].height, 25); // 5 (side) + 20 (size)
/// ```

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
