/*
Medley(vec![
    Ingredient(BreadOfGlory),
    Ingredient(Sunberry),
]).add(
    Ingredient(SunfirePeach)
) ->
Medley(vec![
    Ingredient(BreadOfGlory),
    Ingredient(Sunberry),
    Ingredient(SunfirePeach)
])
Ingredient(BreadOfGlory),
.add(Medley(vec![
    Ingredient(Sunberry),
    Ingredient(SunfirePeach)
])) ->
Medley(vec![
    Ingredient(BreadOfGlory),
    Ingredient(Sunberry),
    Ingredient(SunfirePeach)
])
Ingredient(BreadOfGlory).infuse(Ingredient(Sunberry)) ->
Infusion(vec![
    Ingredient(BreadOfGlory),
    Ingredient(Sunberry)
])

Infusion(vec![
    Ingredient(BreadOfGlory),
    Ingredient(Sunberry)
]).infuse(Ingredient(SunfirePeach)) ->
Infusion(vec![
    Ingredient(BreadOfGlory),
    Ingredient(Sunberry)
    Ingredient(SunfirePeach)
])

Infusion(vec![
    Ingredient(BreadOfGlory),
    Ingredient(SunfirePeach),
]).infuse(Infusion(vec![
    Ingredient(Apple),
    Ingredient(Sunberry)
])) ->
Infusion(vec![
    Ingredient(BreadOfGlory),
    Ingredient(SunfirePeach),
    Infusion(vec![
        Ingredient(Apple),
        Ingredient(Sunberry)
    ])
])
    */
#[test]
fn test(){
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
        },
        specimen
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
    pub fn fold(self, v: Self) -> Self{
        if let Amydgam { base, mut amylgdules } = self {
            amylgdules.push(v);
            return Amydgam { base, amylgdules }
        };
        let base = Box::new(self);
        let amylgdules = vec![v];
        Amydgam { base, amylgdules }
    }
    
}