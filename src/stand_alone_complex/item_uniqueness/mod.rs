use itertools::Itertools;

type Uniqueness = f32;
pub fn main<const N: usize>(items: &[[f32; N]], dist_at_full_uniqueness: f32) -> Vec<Uniqueness> {
    if items.is_empty() {
        return Vec::new();
    }
    if items.len() == 1 {
        return vec![1.0];
    }
    let items_len = items.len() as f32;
    let mut numors: Vec<f32> = (0..items.len()).map(|_| 0.0).collect();
    let mut denoms: Vec<f32> = (0..items.len()).map(|_| 0.0).collect();

    let correction = 0.5 * items_len.powi(2) - 0.5; // 2=1.5, 3=4, 4=7.50, 5=12

    for mut pair in items.iter().enumerate().combinations(2) {
        let (a_idx, a) = pair.pop().unwrap();
        let (b_idx, b) = pair.pop().unwrap();

        // compute the distance scale to uniqise it one as the desires distane and caped so the a item can't have more the 100 uniqwness
        let mut dist = 0.0;
        for i in 0..a.len() {
            dist += a[i].powi(2) + b[i].powi(2);
        }
        dist = correction.min(dist.sqrt() * (correction / dist_at_full_uniqueness));

        let n = ((2 * dist) + 1) / ((dist * items_len) + items_len);
        numors[a_idx] += n;
        numors[b_idx] += n;

        let d = 1 / (dist + 1);
        denoms[a_idx] += d;
        denoms[b_idx] += d;
    }
    numors
        .into_iter()
        .zip(denoms.into_iter())
        .map(|(n, d)| {
            if d < 0.0001 {
                return 0.0;
            }
            n / d
        })
        .collect()
}
