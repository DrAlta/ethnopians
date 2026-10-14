use crate::stand_alone_complex::catchelmy::Concoction::*;

use super::IngredientId::*;

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

