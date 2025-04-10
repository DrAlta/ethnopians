use crate::bvh::MortenCode;

pub fn expand_bits(t: u16) -> MortenCode {
    let mut t = t as u32;
    let mut acc = 0;
    let mut mask = 1;
    for _ in 0..16 {
        //print ln!("mask:{mask:b}\n   t:{t:b}");
        acc |= t & mask;
        mask <<= 2;
        t <<= 1;
    }
    //print ln!("    :{t:b}\nmax :{:b}", u32::MAX);
    acc
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expand_bits1_test() {
        assert_eq!(expand_bits(0b1), 0b1);
    }
    #[test]
    fn expand_bitsmax_test() {
        assert_eq!(expand_bits(u16::MAX), 0b1010101010101010101010101010101);
    }
    #[test]
    fn expand_bits2_test() {
        assert_eq!(expand_bits(0b10), 0b100);
    }
    #[test]
    fn expand_bits3_test() {
        assert_eq!(expand_bits(0b100), 0b10000);
    }
    #[test]
    fn expand_bits16_test() {
        assert_eq!(
            expand_bits(0b1000000000000000),
            0b1000000000000000000000000000000
        );
    }
}
