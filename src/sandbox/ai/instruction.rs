use std::collections::BTreeSet;

use crate::sandbox::{
    ai::{Blackboard, BlackboardKey, BlackboardValue, StackItem, ThreadName, TreePool,
        cpu::{Prayer, tick_action, tick_selector, tick_sequence, ProgramCounter, ReturnStack, Stack},
        ExecutionToken, InpulseId, Status,
    }, World
};

///
/// ForthFindNearest{entity_id: ObjectId, item_class: ItemClass},
/// ForthGetHP(BlackboardKey),
/// and ForthGetEnergy(BlackboardKey),
/// should probably take their argumants off the stack
///
/// should Combine, Use, Eat take a BlackboardKey that points to a ItemClass or the ItemClass directly? ether way InventoryGE should probably do the same
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Instruction {
    Action(InpulseId),
    // takes two Blackboard keys that points to ItemClass
    Combine(BlackboardKey, BlackboardKey),
    // takes a Blackboard key that points to an ItemClass
    Eat(BlackboardKey),
    // takes a Blackboard key that points to an ItemClass and u8 of the number to compare to
    InventoryGE(BlackboardKey, u8),
    Selector(Vec<ExecutionToken>),
    Sequence(Vec<ExecutionToken>),
    // takes two Blackboard keys that points to ItemClass
    Use(BlackboardKey, BlackboardKey),
    //--------------------------------------------------------------------------
    ForthAdd,
    ForthCall(ThreadName, usize),
    //(Coord Coord -- Int)
    ForthDistance,
    ForthDiv,
    ForthDup,
    //(Coord ItemClass -- Option<ObjectId>) finds the neared item of ItemClass to ObjectId
    ForthFindNearest,
    ForthEq,
    ForthGE,
    //(BlackboardKey -- Option<_>)
    ForthGetBlackboard,
    ForthGetEnergy,
    //(BlackboardKey -- Option<Int>)
    ForthGetHP,
    //(BlackboardKey -- Option<Coord>)
    ForthGetLocation,
    ForthGT,
    ForthIf(usize),
    //(_ -- (_ false or Int true))
    ForthIsInt,
    ForthLE,
    ForthLit(StackItem),
    ForthLT,
    ForthMul,
    ForthRem,
    ForthReturn,
    ForthSub,
    //(_ -- (_ false or coord true))
    ForthSomeCoord,
    //(_ -- (_ false or EntityId true))
    ForthSomeEntityId,
    //(_ -- (_ false or Int true))
    ForthSomeInt,
    ForthSwap,
}

impl Instruction {
    pub fn missing_threads_used(&self, bt: &TreePool) -> BTreeSet<ExecutionToken> {
        let mut missing = BTreeSet::new();
        match self {
            Instruction::Selector(vec) | Instruction::Sequence(vec) => {
                for token in vec {
                    if !bt.contains_key(token) {
                        missing.insert(token.clone());
                    }
                }
            }
            Instruction::ForthCall(token, _idx) => {
                if !bt.contains_key(token) {
                    missing.insert(token.clone());
                }
            }
            Instruction::Action(..)
            | Instruction::Combine(_, _)
            | Instruction::Eat(_)
            | Instruction::InventoryGE(_, _)
            | Instruction::Use(_, _)
            | Instruction::ForthGetHP
            | Instruction::ForthGetEnergy
            | Instruction::ForthLit(..)
            | Instruction::ForthAdd
            | Instruction::ForthSub
            | Instruction::ForthMul
            | Instruction::ForthDiv
            | Instruction::ForthRem
            | Instruction::ForthGT
            | Instruction::ForthLT
            | Instruction::ForthGE
            | Instruction::ForthLE
            | Instruction::ForthIsInt
            | Instruction::ForthReturn
            | Instruction::ForthFindNearest
            | Instruction::ForthGetBlackboard
            | Instruction::ForthGetLocation
            | Instruction::ForthSomeCoord
            | Instruction::ForthSomeInt
            | Instruction::ForthSomeEntityId
            | Instruction::ForthDistance
            | Instruction::ForthDup
            | Instruction::ForthSwap
            | Instruction::ForthEq
            | Instruction::ForthIf(_) => (),
        }
        missing
    }
    pub fn tick(
        &self,
        stack: &mut Stack,
        return_stack: &mut ReturnStack,
        pc: &mut ProgramCounter,
        blackboard: &mut Blackboard<BlackboardKey, BlackboardValue>,
        world: &World,
    ) -> Prayer {
        match self {
            Instruction::Action(action_id) => tick_action(action_id, stack, return_stack, pc),
            Instruction::Combine(_, _) => todo!(),
            Instruction::Eat(_) => todo!(),
            Instruction::InventoryGE(_, _) => todo!(),
            Instruction::Selector(children) => tick_selector(children, stack, return_stack, pc),
            Instruction::Sequence(children) => tick_sequence(children, stack, return_stack, pc),
            Instruction::Use(_, _) => todo!(),
            Instruction::ForthAdd => {
                let (nos, tos) = Self::get_two_ints(stack)?;
                stack.push(StackItem::Int(nos + tos));
                Self::next(Status::None, pc)
            }
            Instruction::ForthCall(token, idx) => {
                *pc = Some((token.clone(), *idx));
                Ok(Status::None)
            }
            Instruction::ForthDistance => {
                let Some(StackItem::Coord{..}) = stack.last() else {
                    return Err("top of stack not a number".into());
                };
                let Some(StackItem::Coord{..}) = stack.get(stack.len() - 2) else {
                    return Err("next of stack not a number".into());
                };
                let Some(StackItem::Coord{x: tos_x, y: tos_y}) = stack.pop() else {
                    unreachable!()
                };
                let Some(StackItem::Coord{x: nos_x, y: nos_y}) = stack.pop() else {
                    unreachable!()
                };
                let distance = ((nos_x - tos_x).abs().pow(2) + (nos_y - tos_y).abs().pow(2)).isqrt();
                stack.push(StackItem::Int(distance));
                Self::exit(Status::None, return_stack, pc)
            }
            Instruction::ForthDiv => {
                let (nos, tos) = Self::get_two_ints(stack)?;
                stack.push(StackItem::Int(nos / tos));
                Self::next(Status::None, pc)
            }
            Instruction::ForthDup => {
                let Some(tos) = stack.last() else {
                    return Err("top of stack not a number".into());
                };
                stack.push(tos.clone());
                Self::next(Status::None, pc)
            }
            Instruction::ForthEq => {
                let (nos, tos) = Self::get_two_ints(stack)?;
                if nos == tos {
                    stack.push(StackItem::True);
                } else {
                    stack.push(StackItem::False);
                }
                Self::next(Status::None, pc)
            }
            Instruction::ForthFindNearest => {
                let Some(StackItem::String(_)) = stack.last() else {
                    return Err("tos wasn't a sting".to_owned())
                };
                let Some(StackItem::Coord { .. }) = stack.get(stack.len() - 2) else {
                    return Err("nos wasn't an coord".to_owned())
                };
                let Some(StackItem::String(item_class)) = stack.pop() else {
                    unreachable!()
                };
                let Some(StackItem::Coord { x, y }) = stack.pop() else {
                    unreachable!()
                };
                match world.find_nearest(crate::Vec2 { x: x as f32, y: y as f32 }, &item_class) {
                    Some(thing) => {
                        stack.push(StackItem::some(StackItem::EntityId(thing)));
                        Self::next(Status::None, pc)
                    },
                    None => {
                        stack.push(StackItem::Option(None));
                        Self::next(Status::None, pc)
                    },
                }
            }
            Instruction::ForthGetEnergy => {
                let Some(StackItem::String(_)) = stack.last() else {
                    return Err("tos wasn't a sting".to_owned())
                };
                let Some(StackItem::String(key)) = stack.pop() else {
                    unreachable!()
                };
                let Some(BlackboardValue::EntityId(entity_id)) = blackboard.get(&key) else {
                    stack.push(StackItem::none());
                    return Self::next(Status::None, pc);
                };
                let Some(energy) = world.get_energy(entity_id) else {
                    stack.push(StackItem::none());
                    return Self::next(Status::None, pc);
                };
                stack.push(StackItem::some(StackItem::Int(*energy as i32)));
                Self::next(Status::None, pc)
            }
            Instruction::ForthGetLocation => {
                let Some(StackItem::EntityId(_)) = stack.last() else {
                    return Err("tos wasn't an EntityId".to_owned())
                };
                let Some(StackItem::EntityId(entity_id)) = stack.pop() else {
                    unreachable!()
                };
                match world.get_location(&entity_id) {
                    Some(crate::sandbox::Location::World { x, y }) => {
                        stack.push(StackItem::some(StackItem::Coord { x: *x as i32, y: *y as i32 }));
                        Self::next(Status::None, pc)
                    },
                    _ => {
                        stack.push(StackItem::none());
                        Self::next(Status::None, pc)
                    },
                }
            }
            Instruction::ForthGetHP => {
                let Some(StackItem::String(_)) = stack.last() else {
                    return Err("tos wasn't a sting".to_owned())
                };
                let Some(StackItem::String(key)) = stack.pop() else {
                    unreachable!()
                };
                let Some(BlackboardValue::EntityId(entity_id)) = blackboard.get(&key) else {
                    stack.push(StackItem::none());
                    return Self::next(Status::None, pc);
                };
                let Some(hp) = world.get_hp(entity_id) else {
                    stack.push(StackItem::none());
                    return Self::next(Status::None, pc);
                };
                stack.push(StackItem::some(StackItem::Int(*hp as i32)));
                Self::next(Status::None, pc)
            }
            Instruction::ForthGE => {
                let (nos, tos) = Self::get_two_ints(stack)?;
                if nos >= tos {
                    stack.push(StackItem::True);
                } else {
                    stack.push(StackItem::False);
                }
                Self::next(Status::None, pc)
            }
            Instruction::ForthGetBlackboard => {
                let Some(StackItem::String(_)) = stack.last() else {
                    return Err("tos wasn't a sting".to_owned())
                };
                let Some(StackItem::String(key)) = stack.pop() else {
                    unreachable!()
                };
                /*
                stack.push( match blackboard.get(&key) {
                    Some(x) => match x {
                        BlackboardValue::EntityId(y) => StackItem::EntityId(y.clone()),
                    }
                    None => StackItem::False
                });
                */
                stack.push( match blackboard.get(&key) {
                    Some(x) => StackItem::Option(
                        match x {
                            BlackboardValue::EntityId(y) => Some(
                                Box::new(
                                    StackItem::EntityId(y.clone())
                                )
                            ),
                        }
                    ),
                    None => StackItem::Option(None)

                });
                Self::next(Status::None, pc)

            }
            Instruction::ForthGT => {
                let (nos, tos) = Self::get_two_ints(stack)?;
                if nos > tos {
                    stack.push(StackItem::True);
                } else {
                    stack.push(StackItem::False);
                }
                Self::next(Status::None, pc)
            }
            Instruction::ForthIf(skip) => {
                let Some((_, idx)) = pc else {
                    return Err("unexptect end of program".to_owned());
                };
                *idx += 1;
                if Some(StackItem::True) != stack.pop() {
                    *idx += skip;
                }
                Ok(Status::None)
            }
            Instruction::ForthIsInt => {
                let value = if let Some(StackItem::Int(_)) = stack.last() {
                    StackItem::True
                } else {
                    StackItem::False
                };
                stack.push(value);
                Self::next(Status::None, pc)
            }
            Instruction::ForthLE => {
                let (nos, tos) = Self::get_two_ints(stack)?;
                if nos <= tos {
                    stack.push(StackItem::True);
                } else {
                    stack.push(StackItem::False);
                }
                Self::next(Status::None, pc)
            }
            Instruction::ForthLT => {
                let (nos, tos) = Self::get_two_ints(stack)?;
                if nos < tos {
                    stack.push(StackItem::True);
                } else {
                    stack.push(StackItem::False);
                }
                Self::next(Status::None, pc)
            }
            Instruction::ForthLit(value) => {
                stack.push(value.clone());
                Self::next(Status::None, pc)
            }
            Instruction::ForthMul => {
                let (nos, tos) = Self::get_two_ints(stack)?;
                stack.push(StackItem::Int(nos * tos));
                Self::next(Status::None, pc)
            }
            Instruction::ForthRem => {
                let (nos, tos) = Self::get_two_ints(stack)?;
                stack.push(StackItem::Int(nos % tos));
                Self::next(Status::None, pc)
            }
            Instruction::ForthSomeCoord => {
                let Some(StackItem::Option(Some(x))) = stack.last() else{
                    stack.push(StackItem::False);
                    return Self::next(Status::None, pc)
                };
                match x.as_ref() {
                    StackItem::Coord{..} => (),
                    _ => {
                        stack.push(StackItem::False);
                        return Self::next(Status::None, pc)
                    }
                }
                let Some(StackItem::Option(Some(y))) = stack.pop() else {
                    unreachable!()
                };
                stack.push(Box::into_inner(y));
                stack.push(StackItem::True);
                Self::next(Status::None, pc)
            }
            Instruction::ForthSomeEntityId => {
                let Some(StackItem::Option(Some(x))) = stack.last() else{
                    stack.push(StackItem::False);
                    return Self::next(Status::None, pc)
                };
                match x.as_ref() {
                    StackItem::EntityId(_) => (),
                    _ => {
                        stack.push(StackItem::False);
                        return Self::next(Status::None, pc)
                    }
                }
                let Some(StackItem::Option(Some(y))) = stack.pop() else {
                    unreachable!()
                };
                stack.push(Box::into_inner(y));
                stack.push(StackItem::True);
                Self::next(Status::None, pc)
            }
            Instruction::ForthSomeInt => {
                let Some(StackItem::Option(Some(x))) = stack.last() else{
                    stack.push(StackItem::False);
                    return Self::next(Status::None, pc)
                };
                match x.as_ref() {
                    StackItem::Int(_) => (),
                    _ => {
                        stack.push(StackItem::False);
                        return Self::next(Status::None, pc)
                    }
                }
                let Some(StackItem::Option(Some(y))) = stack.pop() else {
                    unreachable!()
                };
                stack.push(Box::into_inner(y));
                stack.push(StackItem::True);
                Self::next(Status::None, pc)
            }
            Instruction::ForthSub => {
                let (nos, tos) = Self::get_two_ints(stack)?;
                stack.push(StackItem::Int(nos - tos));
                Self::next(Status::None, pc)
            }
            Instruction::ForthSwap => {
                let Some(_) = stack.get(stack.len() - 2) else {
                    return Err("no nos".to_owned())
                };
                let Some(tos) = stack.pop() else {
                    unreachable!()
                };
                let Some(nos) = stack.pop() else {
                    unreachable!()
                };
                stack.push(tos);
                stack.push(nos);
                Self::next(Status::None, pc)
            }
            Instruction::ForthReturn => Self::exit(Status::None, return_stack, pc),
        }
    }
    pub fn correct(&mut self, prefix: &str) {
        match self {
            Instruction::Selector(vec) | Instruction::Sequence(vec) => {
                vec.into_iter().for_each(|x| {
                    if x.starts_with('_') {
                        let y = format!("{prefix}{x}");
                        *x = y
                    };
                });
            }
            Instruction::ForthCall(token, ..) => {
                if token.starts_with('_') {
                    let y = format!("{prefix}{token}");
                    *token = y
                };
            }
            Instruction::Action(_)
            | Instruction::Combine(_, _)
            | Instruction::Eat(_)
            | Instruction::InventoryGE(_, _)
            | Instruction::Use(_, _)
            | Instruction::ForthGetHP
            | Instruction::ForthGetEnergy
            | Instruction::ForthLit(_)
            | Instruction::ForthAdd
            | Instruction::ForthSub
            | Instruction::ForthMul
            | Instruction::ForthDiv
            | Instruction::ForthRem
            | Instruction::ForthGT
            | Instruction::ForthLT
            | Instruction::ForthGE
            | Instruction::ForthLE
            | Instruction::ForthIsInt
            | Instruction::ForthReturn
            | Instruction::ForthFindNearest
            | Instruction::ForthGetBlackboard
            | Instruction::ForthGetLocation
            | Instruction::ForthSomeCoord
            | Instruction::ForthSomeInt
            | Instruction::ForthSomeEntityId
            | Instruction::ForthDistance
            | Instruction::ForthDup
            | Instruction::ForthSwap
            | Instruction::ForthEq
            | Instruction::ForthIf(_) => (),
        }
    }
}
impl Instruction {
    pub fn next(status: Status, pc: &mut ProgramCounter) -> Prayer {
        if let Some((_, idx)) = pc {
            *idx += 1;
        }
        return Ok(status);
    }
    pub fn exit(status: Status, return_stack: &mut ReturnStack, pc: &mut ProgramCounter) -> Prayer {
        if let Some(parent_token) = return_stack.pop() {
            // return to calling fuction
            *pc = Some(parent_token);
            return Ok(status);
        } else {
            // the program finished
            *pc = None;
            return Ok(status);
        };
    }
    pub fn get_two_ints(stack: &mut Stack) -> Result<(i32, i32), String> {
        let Some(StackItem::Int(_)) = stack.last() else {
            return Err("top of stack not a number".into());
        };
        let Some(StackItem::Int(_)) = stack.get(stack.len() - 2) else {
            return Err("next of stack not a number".into());
        };
        let Some(StackItem::Int(tos)) = stack.pop() else {
            unreachable!()
        };
        let Some(StackItem::Int(nos)) = stack.pop() else {
            unreachable!()
        };
        Ok((nos, tos))
    }
}

#[test]
fn correct_test() {
    let mut i = Instruction::Selector(vec!["_2".to_owned(), "_3".to_owned()]);
    i.correct("prefix");
    assert_eq!(
        i,
        Instruction::Selector(vec!["prefix_2".to_owned(), "prefix_3".to_owned()])
    )
}
