use fraction64::Fraction;

pub trait Sqrt {
    fn sqrt(self) -> Self;
}

impl Sqrt for Fraction {
    fn sqrt(self) -> Self {
        let x = format!("{self:.9}").parse::<f32>().unwrap();
        Self::try_from(x.sqrt()).unwrap()
    }
}
