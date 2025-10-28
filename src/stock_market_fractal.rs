use qol::VecStuff;
use thats_so_random::{remove_random_item::RemoveRandomItem, RandoRange, RandomNumberGenerator};

use crate::Number;
// A conceptual struct to hold the state of one "layer"
#[derive(Debug, Clone, Copy)]
pub struct FractalState {
    run_idx: usize,
    s: Number, // start point
    h1: Number, // high point 1
    l1: Number, // low point 1
    h2: Number, // high point 2
    l2: Number, // low point 2
    e: Number, // end point
}

impl FractalState {
    pub fn new<RNG: RandoRange<Number> >(s:Number, e:Number, rng: &mut RNG) -> Self {
        let h1 = rng.random_range(s, e);
        let l1 = rng.random_range(s, h1);
        let h2 = rng.random_range(h1, e);
        let l2 = rng.random_range(h1, l1);
        FractalState { run_idx: 0, s, e, h1, h2, l1, l2 }
    }
}

#[derive(Debug, Clone)]
pub struct Fractal<const N: usize, I>{
    samples:[I;N],
    layer_states: [FractalState;N],
    turns: Vec<usize>,
    last: [usize;N],

}

impl<const N: usize, I:Iterator<Item = Number>> Fractal<N, I> {
    pub fn sample<RNG: RandomNumberGenerator + RandoRange<Number>>(&mut self, rng: &mut RNG ) -> Option<Number>{
        let now = *self.last.iter().max().unwrap();
        let start = now + 1;
        let next: [usize; N] =std::array::from_fn(
            |idx|
            {
                start + match self.turns.find_first(&idx){
                    Some(x) => x,
                    None => {
                        fix::<N, RNG>(&mut self.turns, rng);
                        self.turns.find_first(&idx).expect("we added it if it was missing")
                    },
                }
            }
        );
        let now1 = now as f32;
        let mut ret = Number::ZERO;

        for idx in 0..N {
            let layer = &mut self.layer_states[idx];
            let a = self.last[idx] as f32;
            let t = (now1 - a) / (next[idx] as f32 - a);
            ret += match layer.run_idx % 5 {
                0 => {
                    (&layer.s) + (&layer.h1 - &layer.s) * t

                }
                1 => {
                    (&layer.h1) + (&layer.l1 - &layer.h1) * t

                }
                2 => {
                    (&layer.l1) + (&layer.h2 - &layer.l1) * t

                }
                3 => {
                    (&layer.h2) + (&layer.l2 - &layer.h2) * t

                }
                4 => {
                    let x = (&layer.h2) + (&layer.e - &layer.h2) * t;

                    let shuffle = FractalState::new(layer.e, self.samples[idx].next()?, rng);
                    *layer = shuffle;
                    self.last[idx] = now;

                    x

                }
                _ => unreachable!("this should be reachal as be modulo 5 the idx")
            }
        }
        self.turns.pop();
        Some(ret)

    }
}


pub fn foo<const N: usize, I:Iterator<Item = Number>, RNG: RandomNumberGenerator + RandoRange<Number>>(mut samples:[I;N], rng: &mut RNG) -> Option<Fractal<N, I>> {

    let mut layers_runs= Vec::new();
    for idx in 0..N {
        let s= samples[idx].next()?;
        let e= samples[idx].next()?;
        layers_runs.push(FractalState::new(s, e, rng))
    }
    let mut turns = Vec::new();
    fix::<N, RNG>(&mut turns, rng);
    let mut last1: Vec<usize> = (0..N).collect();
    let last = std::array::from_fn(|_| last1.remove_random_item(rng).unwrap());
    Some(Fractal { samples, layer_states: layers_runs.try_into().unwrap(), turns, last})
}

fn fix<const N: usize, RNG: RandomNumberGenerator>(selfie: &mut Vec<usize>, rng: &mut RNG){
    let mut needed: Vec<usize> = (0..N).collect();
    needed.retain(
        |x|
        {
            !selfie.contains(x)
        }
    );
    while !needed.is_empty() {
        let dice = rng.random_range(0, (N* 2) + needed.len());
        if dice < N {
            selfie.push(dice);
            needed.retain(
                |x|
                {
                    x != &dice
                }
            );
        } else {
            selfie.push(needed.remove_random_item(rng).unwrap());
        };
    }
    
}