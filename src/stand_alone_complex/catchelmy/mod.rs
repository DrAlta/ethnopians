#[test]
fn add_test_1(){
    let specimen = Medley(vec![
        Ingredient(BreadOfGlory),
        Ingredient(Sunberry),
    ]).add(
        Ingredient(SunfirePeach)
    );
    qol::assert_specimen!(
        specimen,
        Medley(vec![
            Ingredient(BreadOfGlory),
            Ingredient(Sunberry),
            Ingredient(SunfirePeach)
        ])
    );
}
#[test]
fn add_test_2(){

    let specimen = Ingredient(BreadOfGlory)
        .add(Medley(vec![
            Ingredient(Sunberry),
            Ingredient(SunfirePeach)
        ])
    );
    qol::assert_specimen!(
        specimen,
        Medley(vec![
            Ingredient(BreadOfGlory),
            Ingredient(Sunberry),
            Ingredient(SunfirePeach)
        ])
    );
}
#[test]
fn infuse_test_1(){
    let specimen = Ingredient(BreadOfGlory).infuse(Ingredient(Sunberry));
    qol::assert_specimen!(
        specimen,
        Infusion(vec![
            Ingredient(BreadOfGlory),
            Ingredient(Sunberry)
        ])
    );
}
#[test]
fn infuse_test_2() {
    let specimen = Infusion(vec![
        Ingredient(BreadOfGlory),
        Ingredient(Sunberry)
    ]).infuse(Ingredient(SunfirePeach));
    qol::assert_specimen!(
        specimen,
        Infusion(vec![
            Ingredient(BreadOfGlory),
            Ingredient(Sunberry),
            Ingredient(SunfirePeach)
        ])
    );
}
#[test]
fn infuse_test_3(){
    let specimen = Infusion(vec![
        Ingredient(BreadOfGlory),
        Ingredient(SunfirePeach),
    ]).infuse(Infusion(vec![
        Ingredient(Apple),
        Ingredient(Sunberry)
    ]));
    qol::assert_specimen!(
        specimen,
        Infusion(vec![
            Ingredient(BreadOfGlory),
            Ingredient(SunfirePeach),
            Infusion(vec![
                Ingredient(Apple),
                Ingredient(Sunberry)
            ])
        ])
    )
}
#[test]
fn fold_test(){
    let specimen = Amydgam{
        base:Box::new(Ingredient(BreadOfGlory)), 
        amylgdules: vec![
            Ingredient(SunfirePeach)
        ]
    }.fold(
        Amydgam{
            base:Box::new(Ingredient(Sunberry)), 
            amylgdules: vec![
                Ingredient(Apple)
            ]
        }
    );

    qol::assert_specimen!(
        specimen,
        Amydgam{
            base:Box::new(Ingredient(BreadOfGlory)), 
            amylgdules: vec![
                Ingredient(SunfirePeach), 
                Amydgam{
                    base:Box::new(Ingredient(Sunberry)), 
                    amylgdules: vec![
                        Ingredient(Apple)
                    ]
                }
            ]
        }
    );
}

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
use IngredientId::*;
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