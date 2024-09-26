use thats_so_random::Pcg32;

pub fn sample<'a, 'b, const N: usize>(a: &'a [u8; N], rng: &'b mut Pcg32) -> usize {
    let b = a.iter().enumerate();
    let mut x = rng.random_range(0, a.iter().map(|x| *x as i32).sum());
    for (idx, i) in b {
        let i = *i as i32;
        if x < i {
            return idx;
        } else {
            x = x - i;
        }
    }
    a.len()
}
#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn sample_test() {
        let mut rng = thats_so_random::Pcg32::new(
            thats_so_random::DEFAULT_STATE,
            thats_so_random::DEFAULT_STREAM,
        );
        let a = [1_u8, 2, 3, 4, 5];

        println!("{}", sample(&a, &mut rng))
    }
}
