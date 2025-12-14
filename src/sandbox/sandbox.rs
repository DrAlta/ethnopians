/*
trees: take water and sunlight; produced sticks when used; produce wood when axe in used on them
veggies: take water and sunlight; produce food when used;

food: when used restores energy
wood: used on wood to build house

stone: when used on Stick makes axe
stick: when used on stone makes axe

house: when used restores energy, any excess restores health


---this stuff seems to complex
axe head: make from using stone on stone; used on stick to make axe; used on agent deal damage
grass: take water and sunlight and produce fiber when harvested

Spear: used long range(father than reach of axe) deals damage; use close range pushed target away

*/

use crate::Number;

pub fn within_range(x1: Number, y1: Number, x2: Number, y2: Number, dist: Number) -> bool {
    let x_off = x1 - x2;
    let y_off = y1 - y2;
    ((x_off * x_off) + (y_off * y_off)) > (dist * dist)
}