use crate::{vec2, Number, Vec2};
pub struct Layout {
    child: Vec<Layout>,
}

impl Layout {
    pub fn size(&self, spacing: Number) -> Vec2 {
        let mut iter = self.child.iter();
        let init = if let Some(first) = iter.next() {
            first.size(spacing)
        } else {
            vec2(0.0, 0.0)
        };

        let x = iter.fold(init, |acc, item| {
            let item_size = item.size(spacing);
            vec2(acc.x + spacing + item_size.x, acc.y.max(item_size.y))
        });
        vec2(x.x, x.y + spacing)
    }
}
