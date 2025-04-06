use std::collections::BTreeSet;

use crate::Number;

pub type SpatialId = usize;

#[derive(Debug, Clone, PartialEq)]
pub struct AARect {
    pub min_x: Number,
    pub min_y: Number,
    pub width: Number,
    pub height: Number,
}
impl AARect {
    pub fn new(min_x: Number, min_y: Number, width: Number, height: Number) -> Self {
        Self {
            min_x,
            min_y,
            width,
            height,
        }
    }
    pub fn get_min_x(&self) -> Number {
        self.min_x
    }
    pub fn get_min_y(&self) -> Number {
        self.min_y
    }
    pub fn inside(&self, x: Number, y: Number) -> bool {
        x >= self.min_x
            && x <= self.min_x + self.width
            && y >= self.min_y
            && y <= self.min_y + self.height
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Entry {
    aabb: AARect,
    entity_id: SpatialId,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SweepAndPrune {
    sorted: Vec<Entry>,
    dirty: bool,
}

fn sortie(
    Entry {
        aabb: AARect { min_x: a, .. },
        ..
    }: &Entry,
    Entry {
        aabb: AARect { min_x: b, .. },
        ..
    }: &Entry,
) -> std::cmp::Ordering {
    a.total_cmp(b)
}

/*

        function sweepAndPruneCollisions(spheres) {
            const sortedSpheres = spheres.sort((a, b) => a.left - b.left);

            for (let i = 0; i < sortedSpheres.length; i++) {
                const sphere1 = sortedSpheres[i];

                for (let j = i + 1; j < sortedSpheres.length; j++) {
                    const sphere2 = sortedSpheres[j];

                    if (sphere2.left > sphere1.right) {
                        break;
                    }

                    if (Math.abs(sphere1.y - sphere2.y) <= sphere1.radius + sphere2.radius) {
                        solveCollision(sphere1, sphere2);
                    }
                }
            }
        }
*/
