use crate::Number;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AARect {
    min_x: Number,
    min_y: Number,
    max_y: Number,
    max_x: Number,
}
// constructures
impl AARect {
    pub fn from_min_max(min_x: Number, min_y: Number, max_x: Number, max_y: Number) -> Self {
        Self {
            min_x,
            min_y,
            max_x,
            max_y,
        }
    }

    pub fn from_min_w_h(min_x: Number, min_y: Number, width: Number, height: Number) -> Self {
        let max_x = (&min_x) + width;
        let max_y = (&min_y) + height;
        Self {
            min_x,
            min_y,
            max_x,
            max_y,
        }
    }
}


impl AARect {
    pub fn better_to_merg_with_a_than_b_ka (
        &self,
        a: &AARect,
        b: &AARect,
    ) -> bool {
        let a_self = self.union(a);
        let overlap_a_self_b = a_self.intersection(b);

        let Some(overlap_a) = overlap_a_self_b else {
            return true;
        };

        let b_self = self.union(b);
    
        let overlap_b_self_a = b_self.intersection(a);

        let Some(overlap_b) = overlap_b_self_a else {
            return false;
        };


        // Get the volumes of the overlaps, default to 0.0 if no overlap exists
        let vol_a_self_to_b = overlap_a.volume();
        let vol_b_self_to_a = overlap_b.volume();

        // Return true if the first overlap is smaller or equal
        vol_a_self_to_b <= vol_b_self_to_a
    }
    pub fn height(&self) -> Number { 
        self.max_y - self.min_y
    }
    pub fn inside(&self, x: Number, y: Number) -> bool {
        x >= self.min_x
            && x <= self.max_x
            && y >= self.min_y
            && y <= self.max_y
    }
    pub fn intersection(&self, other: &Self) -> Option<Self> {
        let min_x = self.min_x.max(other.min_x);
        let min_y = self.min_y.max(other.min_y);

        let max_x = self.max_x.min(other.max_x);
        let max_y = self.max_y.min(other.max_y);


        // An intersection only exists if min is less than max in all components.
        if min_x < max_x && min_y < max_y {
            Some(AARect { min_x, min_y, max_x, max_y })
        } else {
            None
        }
    }
    pub fn intersects(&self, other: &Self) -> bool {
        // Check if there's no intersection along the X-axis
        if self.min_x > other.max_x || self.max_x < other.min_x {
            return false;
        }
        
        // Check if there's no intersection along the Y-axis
        if self.min_y > other.max_y || self.max_y < other.min_y {
            return false;
        }
        
        // If we've passed both checks, the rectangles intersect
        true
    }

    pub fn min_x(&self) -> Number {
        self.min_x
    }
    pub fn min_y(&self) -> Number {
        self.min_y
    }
    pub fn max_x(&self) -> Number {
        self.max_x
    }
    pub fn max_y(&self) -> Number {
        self.max_y
    }
    // Creates the minimal AABB that contains both `self` and `other`.
    pub fn union(&self, other: &Self) -> Self {
        let min_x = self.min_x.min(other.min_x);
        let min_y = self.min_y.min(other.min_y);

        let max_x = self.max_x.max(other.max_x);
        let max_y = self.max_y.max(other.max_y);


        AARect { min_x, min_y, max_x, max_y }
    }

    // Creates the intersection AABB of `self` and `other`.
    pub fn width(&self) -> Number{
        self.max_x - self.min_x
    }

    // Calculates the volume (or area for 2D) of the AABB.
    pub fn volume(&self) -> Number {
       self.width() * self.height()
    }

}
