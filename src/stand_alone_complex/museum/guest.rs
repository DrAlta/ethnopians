use qol::AsA;

use crate::Number;

use super::Exhibit;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Guest {
    pub enjoyment: Number,
    pub max_enjoyment: Number,
}

impl Guest {
/*
In Two Point Museum, exhibits have knowledge and buzz. I was thinking in my game 
they have an attractiveness. Buzz would be whether they tell their friends about 
the exhibit when they go home from the museum, while attractiveness is how they 
enjoy the exhibit while in the museum. I was thinking attractiveness is an area 
effect where the enjoyment is based on all the exhibits around them. 

Maybe the enjoyment level of a guest is like a running average of all the a
ttractiveness of the exhibits, with a maximum that can't be increased beyond the 
attractiveness and buzz values of the exhibits.
*/

    pub fn update_enjoyment<'a,'b, I: IntoIterator<Item = &'b Exhibit>>(&'a mut self, near_by_exhibits: I) {
        let mut high_oomph_attractiveness = Number::ZERO;
        let mut low_oomph_base = Number::ZERO;
        let mut low_oomph_count = 0;

        for exhibit in near_by_exhibits.into_iter() {
            let oomph = exhibit.buzz + exhibit.attractiveness; 
            
            if oomph >= self.enjoyment { 
                // High-oomph exhibits directly boost attractiveness
                high_oomph_attractiveness += exhibit.attractiveness; 
            } else { 
                // Low-oomph exhibits provide a base stability
                low_oomph_base += oomph;
                low_oomph_count += 1;
            }
        } 

        // Calculate base value from low-oomph exhibits
        let base_stability = if low_oomph_count > 0 {
            low_oomph_base / low_oomph_count.as_a::<Number>()
        } else {
            Number::ZERO
        };

        // Ensure some contribution even when high-oomph exhibits are absent
        let enjoyment_increase = high_oomph_attractiveness.max(base_stability);

        // Update enjoyment with weighted calculation
        self.enjoyment = (self.enjoyment * Number::from((3,4))) + (Number::FOURTH * enjoyment_increase);
    }
}

