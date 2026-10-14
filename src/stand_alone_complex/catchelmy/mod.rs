#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq, Eq)]
pub enum Concoction {
    Ingredient(IngredientId),
    Medley(Vec<Self>),
    Infusion(Vec<Self>),
    Amydgam{
        base: Box<Self>,
        amylgdules:Vec<Self>,
    },
}
use Concoction::*;
#[derive(Debug, PartialEq, Eq)]
pub enum IngredientId{
    Apple,
    BreadOfGlory,
    Sunberry,
    SunfirePeach,
}
impl  Concoction{
    pub fn add(self, v: Self) -> Self{
        match (self, v) {
            (Medley(mut a), Medley(mut b)) => {
                a.append(&mut b);
                return Medley(a)
            },
            (Medley(mut a), b) => {
                a.push(b);
                return Medley(a)
            },
            (a, Medley(mut b)) => {
                let mut m =vec![a];
                m.append(&mut b);
                return Medley(m)
            },
            (a, b) =>  Medley(vec![a,b])
        }
    }
    pub fn fold(self, v: Self) -> Self{
        if let Amydgam { base, mut amylgdules } = self {
            amylgdules.push(v);
            return Amydgam { base, amylgdules }
        };
        let base = Box::new(self);
        let amylgdules = vec![v];
        Amydgam { base, amylgdules }
    }
    pub fn infuse(self, v: Self) -> Self{
        if let Infusion(mut infusion) = self {
            infusion.push(v);
            return Infusion(infusion)
        }
        Infusion(vec![self, v])
    }
    
}