use qol::logy;

use crate::sandbox::ai::{
    cpu::{tick_action, tick_selector, tick_sequence, Instruction, Prayer, ProgramCounter, ReturnStack, Stack},
    Blackboard, BlackboardKey, BlackboardValue, StackItem, Status,
};

impl Instruction{
    pub fn tick(
        &self,
        stack: &mut Stack,
        return_stack: &mut ReturnStack,
        pc: &mut ProgramCounter,
        blackboard: &mut Blackboard<BlackboardKey, BlackboardValue>,
    ) -> Prayer {
        logy!("debug", "\nticking:{self:?}");
        match self {
            Instruction::Action(action_id) => tick_action(action_id, stack, return_stack, pc),
            Instruction::Combine(_, _) => todo!(),
            Instruction::Eat(_) => todo!(),
            Instruction::InventoryGE(key, amount) => {
                let Some(StackItem::String(x)) = stack.last() else {
                    return Err("tos was not an Init".to_owned());
                };
                if x != "Init" {
                    return Err("tos was not an Init".to_owned());
                };
                stack.pop();
                let Some(&BlackboardValue::EntityId(agent)) = blackboard.get("self") else {
                    return Err(format!("self not found in blackboard"));
                };
                let Some(BlackboardValue::String(item_class_string)) = blackboard.get(key) else {
                    return Err(format!("{key} not found in blackboard"));
                };

                let item_class_str: &str = &item_class_string;
                let Ok(item_class) = item_class_str.try_into() else {
                    return Err(format!("{item_class_string} is not a valid item class"));
                };

                let Some(parent_token) = return_stack.pop() else {
                    return Err("nothing to return to".to_owned());
                };
                // return to calling fuction
                *pc = Some(parent_token);
                return Ok(Status::GetIsInventoryGE {
                    agent,
                    item_class,
                    amount: *amount,
                });
            }
            Instruction::Selector(children) => tick_selector(children, stack, return_stack, pc),
            Instruction::Sequence(children) => tick_sequence(children, stack, return_stack, pc),
            Instruction::Use(_, _) => todo!(),
            Instruction::ForthAdd => {
                if let Ok((nos, tos)) = Self::get_two_ints(stack) {
                    stack.push(StackItem::Int(nos + tos));
                    Self::next(Status::None, pc)
                } else {
                    let (nos, tos) = Self::get_two_coords(stack)?;
                    stack.push(StackItem::Coord {
                        x: nos.0 + tos.0,
                        y: nos.1 + tos.1,
                    });
                    Self::next(Status::None, pc)
                }
            }
            Instruction::ForthCall(token, idx) => {
                *pc = Some((token.clone(), *idx));
                Ok(Status::None)
            }
            Instruction::ForthDistance => {
                if stack.len() < 2 {
                    return Err("less that 2 items on stack".to_owned())
                };
                let Some(StackItem::Coord { .. }) = stack.last() else {
                    return Err("top of stack not a number".into());
                };
                let Some(StackItem::Coord { .. }) = stack.get(stack.len() - 2) else {
                    return Err("next of stack not a number".into());
                };
                let Some(StackItem::Coord { x: tos_x, y: tos_y }) = stack.pop() else {
                    unreachable!()
                };
                let Some(StackItem::Coord { x: nos_x, y: nos_y }) = stack.pop() else {
                    unreachable!()
                };
                let distance =
                    ((nos_x - tos_x).abs().pow(2) + (nos_y - tos_y).abs().pow(2)).isqrt();
                stack.push(StackItem::Int(distance));
                Self::exit(Status::None, return_stack, pc)
            }
            Instruction::ForthDiv => {
                let (nos, tos) = Self::get_two_ints(stack)?;
                stack.push(StackItem::Int(nos / tos));
                Self::next(Status::None, pc)
            }
            Instruction::ForthDrop => {
                if stack.is_empty() {
                    return Err("nothing on sack".into());
                };
                stack.pop();
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
            Instruction::ForthFindInInventory => {
                let Some(StackItem::String(_)) = stack.last() else {
                    return Err("tos wasn't a sting".to_owned());
                };
                let Some(StackItem::String(item_class_string)) = stack.pop() else {
                    unreachable!()
                };
                let Ok(item_class) = item_class_string.try_into() else {
                    return Err("item class was not valid".to_owned());
                };
                Self::next(Status::FindInInventory { item_class }, pc)
                /* this the old pre bevy impl
                match world.find_nearest(
                    crate::Vec2 {
                        x: x as f32,
                        y: y as f32,
                    },
                    &item_class,
                ) {
                    Some(thing) => {
                        stack.push(StackItem::some(StackItem::EntityId(thing)));
                        Self::next(Status::None, pc)
                    }
                    None => {
                        stack.push(StackItem::Option(None));
                        Self::next(Status::None, pc)
                    }
                }
                */
            }
            // ForthFindNearest should set up the CPU for runing the next instruction when it it ticked then pray for the answer to be put on the stack
            Instruction::ForthFindNearest => {
                if stack.len() < 2 {
                    return Err("less that 2 items on stack".to_owned())
                };
                let Some(StackItem::String(_)) = stack.last() else {
                    return Err("tos wasn't a sting".to_owned());
                };
                let Some(StackItem::Coord { .. }) = stack.get(stack.len() - 2) else {
                    return Err("nos wasn't an coord".to_owned());
                };
                let Some(StackItem::String(item_class_string)) = stack.pop() else {
                    unreachable!()
                };
                let Some(StackItem::Coord { x, y }) = stack.pop() else {
                    unreachable!()
                };
                let Ok(item_class) = item_class_string.try_into() else {
                    return Err("item class was not valid".to_owned());
                };
                Self::next(Status::FindNearest { x, y, item_class }, pc)
                /* this the old pre bevy impl
                match world.find_nearest(
                    crate::Vec2 {
                        x: x as f32,
                        y: y as f32,
                    },
                    &item_class,
                ) {
                    Some(thing) => {
                        stack.push(StackItem::some(StackItem::EntityId(thing)));
                        Self::next(Status::None, pc)
                    }
                    None => {
                        stack.push(StackItem::Option(None));
                        Self::next(Status::None, pc)
                    }
                }
                */
            }
            // ForthGetEnergy should set up the CPU for runing the next instruction when it it ticked then pray for the answer to be put on the stack
            Instruction::ForthGetEnergy => {
                let Some(StackItem::EntityId(_)) = stack.last() else {
                    return Err("tos wasn't an EntityId".to_owned());
                };
                let Some(StackItem::EntityId(entity_id)) = stack.pop() else {
                    unreachable!()
                };
                return Self::next(Status::GetEnergy(entity_id.clone()), pc);
                /* this is the pre bevy impl
                let Some(energy) = world.get_energy(entity_id) else {
                    stack.push(StackItem::none());
                    return Self::next(Status::None, pc);
                };
                stack.push(StackItem::some(StackItem::Int(*energy as i32)));
                Self::next(Status::None, pc)
                */
            }
            // ForthGetLocation should set up the CPU for runing the next instruction when it it ticked then pray for the answer to be put on the stack
            Instruction::ForthGetLocation => {
                let Some(StackItem::EntityId(_)) = stack.last() else {
                    return Err("tos wasn't an EntityId".to_owned());
                };
                let Some(StackItem::EntityId(entity_id)) = stack.pop() else {
                    unreachable!()
                };
                Self::next(Status::GetLocation(entity_id.clone()), pc)
                /* this is the pre bevy impl
                match world.get_location(&entity_id) {
                    Some(crate::sandbox::Location::World { x, y }) => {
                        stack.push(StackItem::some(StackItem::Coord {
                            x: *x as i32,
                            y: *y as i32,
                        }));
                        Self::next(Status::None, pc)
                    }
                    _ => {
                        stack.push(StackItem::none());
                        Self::next(Status::None, pc)
                    }
                }
                */
            }
            // ForthGetHP should set up the CPU for runing the next instruction when it it ticked then pray for the answer to be put on the stack
            Instruction::ForthGetHP => {
                let Some(StackItem::String(_)) = stack.last() else {
                    return Err("tos wasn't a sting".to_owned());
                };
                let Some(StackItem::String(key)) = stack.pop() else {
                    unreachable!()
                };
                let Some(BlackboardValue::EntityId(entity_id)) = blackboard.get(&key) else {
                    return Err(format!("{key} not found in blackboard"));
                };
                Self::next(Status::GetHp(entity_id.clone()), pc)
                /* this is the pre bevy impl
                let Some(hp) = world.get_hp(entity_id) else {
                    stack.push(StackItem::none());
                    return Self::next(Status::None, pc);
                };
                stack.push(StackItem::some(StackItem::Int(*hp as i32)));
                Self::next(Status::None, pc)
                */
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
                    return Err("tos wasn't a sting".to_owned());
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
                stack.push(match blackboard.get(&key) {
                    Some(x) => StackItem::Option(match x {
                        BlackboardValue::EntityId(y) => Box::new(StackItem::EntityId(y.clone())),
                        BlackboardValue::String(a) => Box::new(StackItem::String(a.clone())),
                    }),
                    None => StackItem::none(),
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
            Instruction::ForthInventoryGE => {
                if stack.len() < 2 {
                    return Err("less that 2 items on stack".to_owned())
                };
                let Some(StackItem::String(_)) = stack.last() else {
                    return Err("tos was not an string".to_owned());
                };
                let Some(StackItem::Int(_)) = stack.get(stack.len() - 2) else {
                    return Err("nos was not an Int".to_owned());
                };
                let Some(StackItem::String(item_class_string)) = stack.pop() else {
                    return Err("tos was not an string".to_owned());
                };
                let Some(StackItem::Int(amount)) = stack.pop() else {
                    return Err("nos was not an INt".to_owned());
                };
                let item_class_str: &str = &item_class_string;
                let Ok(item_class) = item_class_str.try_into() else {
                    return Err(format!("{item_class_string} is not a valid item class"));
                };

                let Some(&BlackboardValue::EntityId(agent)) = blackboard.get("self") else {
                    return Err(format!("self not found in blackboard"));
                };
                Self::next(
                    Status::GetIsInventoryGE {
                        agent,
                        item_class,
                        amount,
                    },
                    pc
                )
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
            Instruction::ForthJump(token, idx) => {
                *pc = Some((token.clone(), idx.clone()));
                Ok(Status::None)
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
            Instruction::ForthNotTrue => {
                //stack.push(ifSome(StackItem::True) == 
                match stack.pop() {
                    Some(StackItem::True) => {
                        stack.push(StackItem::False);
                    },
                    Some(_) => {
                        stack.push(StackItem::True);
                    },
                    None => return Err("no top of stack".to_owned()),
                };
                Self::next(Status::None, pc)
            }
            Instruction::ForthOr => {
                if stack.len() < 2 {
                    return Err("less that 2 items on stack".to_owned())
                };
                let tos = stack.pop().unwrap();
                let nos = stack.pop().unwrap();
                stack.push(if tos == StackItem::True || nos == StackItem::True {
                    StackItem::True
                } else {
                    StackItem::False
                });
                Self::next(Status::None, pc)
            }

            Instruction::ForthRem => {
                let (nos, tos) = Self::get_two_ints(stack)?;
                stack.push(StackItem::Int(nos % tos));
                Self::next(Status::None, pc)
            }
            Instruction::ForthSetBlackboard => {
                if stack.len() < 2 {
                    return Err("less that 2 items on stack".to_owned())
                };
                let Some(StackItem::String(_)) = stack.get(stack.len() - 2) else {
                    return Err("no nos".to_owned());
                };
                let Some(tos) = stack.pop() else {
                    unreachable!()
                };
                let Some(StackItem::String(key)) = stack.pop() else {
                    unreachable!()
                };
                blackboard.insert(key, crate::sandbox::ai::Variable::Chit(tos.into()));
                Self::next(Status::None, pc)
            }
            Instruction::ForthStuff => {
                let Some(StackItem::Table(_)) = stack.get(stack.len() - 3) else {
                    return Err("3rd item wasn't a table".to_owned());
                };
                let Some(key) = stack.pop() else {
                    unreachable!()
                };
                let Some(value) = stack.pop() else {
                    unreachable!()
                };
                let Some(table) = stack.last_mut() else {
                    unreachable!()
                };
                let ret = match table.stuff(value, key) {
                    Ok(_) => StackItem::True,
                    Err(_) => StackItem::False,
                };
                stack.push(ret);
                Self::next(Status::None, pc)
            }
            Instruction::ForthSomeCoord => {
                let Some(StackItem::Option(x)) = stack.last() else {
                    stack.push(StackItem::False);
                    return Self::next(Status::None, pc);
                };
                match x.as_ref() {
                    StackItem::Coord { .. } => (),
                    _ => {
                        stack.push(StackItem::False);
                        return Self::next(Status::None, pc);
                    }
                }
                let Some(StackItem::Option(y)) = stack.pop() else {
                    unreachable!()
                };
                stack.push(Box::into_inner(y));
                stack.push(StackItem::True);
                Self::next(Status::None, pc)
            }
            Instruction::ForthSomeEntityId => {
                let Some(StackItem::Option(x)) = stack.last() else {
                    stack.push(StackItem::False);
                    return Self::next(Status::None, pc);
                };
                match x.as_ref() {
                    StackItem::EntityId(_) => (),
                    _ => {
                        stack.push(StackItem::False);
                        return Self::next(Status::None, pc);
                    }
                }
                let Some(StackItem::Option(y)) = stack.pop() else {
                    unreachable!()
                };
                stack.push(Box::into_inner(y));
                stack.push(StackItem::True);
                Self::next(Status::None, pc)
            }
            Instruction::ForthIsEmpty => {
                let Some(StackItem::Table(x)) = stack.last() else {
                    return Err("TOS wasn't a table".to_owned());
                };
                let map_empty_ka = x.map.borrow().is_empty();
                stack.push(if map_empty_ka {
                    StackItem::True
                } else {
                    StackItem::False
                });
                Self::next(Status::None, pc)
            }
            Instruction::ForthPopLast => {
                let Some(StackItem::Table(x)) = stack.last() else {
                    return Err("TOS wasn't a table".to_owned());
                };
                let last_maybe = x.map.borrow_mut().pop_last();

                if let Some((_, last)) = last_maybe {
                    stack.push(StackItem::some(last));
                } else {
                    stack.push(StackItem::False);
                };
                Self::next(Status::None, pc)
            }
            Instruction::ForthSomeInt => {
                let Some(StackItem::Option(x)) = stack.last() else {
                    stack.push(StackItem::False);
                    return Self::next(Status::None, pc);
                };
                match x.as_ref() {
                    StackItem::Int(_) => (),
                    _ => {
                        stack.push(StackItem::False);
                        return Self::next(Status::None, pc);
                    }
                }
                let Some(StackItem::Option(y)) = stack.pop() else {
                    unreachable!()
                };
                stack.push(Box::into_inner(y));
                stack.push(StackItem::True);
                Self::next(Status::None, pc)
            }
            Instruction::ForthSub => {
                if let Ok((nos, tos)) = Self::get_two_ints(stack) {
                    stack.push(StackItem::Int(nos - tos));
                    Self::next(Status::None, pc)
                } else {
                    let (nos, tos) = Self::get_two_coords(stack)?;
                    stack.push(StackItem::Coord {
                        x: nos.0 - tos.0,
                        y: nos.1 - tos.1,
                    });
                    Self::next(Status::None, pc)
                }
            }
            Instruction::ForthSwap => {
                if stack.len() < 2 {
                    return Err("less that 2 items on stack".to_owned())
                };
                let Some(_) = stack.get(stack.len() - 2) else {
                    return Err("no nos".to_owned());
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
            // ToDoGetEntities should set up the CPU for runing the next instruction when it it ticked then pray for the answer to be put on the stack
            Instruction::ForthGetEntities => {
                let ((min_x, min_y), (max_x, max_y)) = Self::get_two_coords(stack)?;
                Self::next(
                    Status::GetEntities {
                        min_x,
                        min_y,
                        max_x,
                        max_y,
                    },
                    pc,
                )
                /* pre bevy impl
                let Some((sb, map)) = world.get_spatial_bloom() else {
                    return Err("world had no SpatailBloom".to_owned());
                };
                let mut x =Vec::new();
                for y in sb.qurry(nos.0 as f32,nos.1 as f32, tos.0 as f32, tos.1 as f32) {
                    let Some(thing) = map.get(&y) else {
                        return Err(format!("SpatialBloom returned {y:?} but that id isn't mapped to any entities"))
                    };
                    x.push(thing.clone());
                };
                stack.push(StackItem::Todo(x));
                Self::next(Status::None, pc)
                */
            }
            Instruction::ForthRemoveEntitiesOfType => {
                let Some(StackItem::String(stack_string)) = stack.last() else {
                    return Err("top of stack not a number".into());
                };
                let stack_str: &str = stack_string;
                let Ok(item_type_from_stack) = stack_str.try_into() else {
                    return Err(format!("couldn't convert {stack_str:?} to type"));
                };
                Self::next(Status::RemoveEntitiesOfType(item_type_from_stack), pc)

                /* pre bevy impl
                let Some(StackItem::Todo(_)) = stack.get(stack.len() - 2) else {
                    return Err("next of stack not a number".into());
                };
                let Some(StackItem::String(_)) = stack.pop() else {
                    unreachable!()
                };
                let Some(StackItem::Todo(entities)) = stack.last_mut() else {
                    unreachable!()
                };

                entities.retain(|id|{
                    if let Some(this_items_type) = world.get_type(id) {
                        !(this_items_type == &item_type_from_stack)
                    } else {
                        true
                    }
                });
                Self::next(Status::None, pc)
                */
            }
            Instruction::ForthRetainEntitiesOfType => {
                let Some(StackItem::String(stack_string)) = stack.last() else {
                    return Err("top of stack not a number".into());
                };
                let stack_str: &str = stack_string;
                let Ok(item_type_from_stack) = stack_str.try_into() else {
                    return Err(format!("couldn't convert {stack_str:?} to type"));
                };
                Self::next(Status::RetainEntitiesOfType(item_type_from_stack), pc)

                /* pre bevy impl
                let Some(StackItem::Todo(_)) = stack.get(stack.len() - 2) else {
                    return Err("next of stack not a number".into());
                };
                let Some(StackItem::String(_)) = stack.pop() else {
                    unreachable!()
                };
                let Some(StackItem::Todo(entities)) = stack.last_mut() else {
                    unreachable!()
                };

                entities.retain(|id|{
                    if let Some(this_items_type) = world.get_type(id) {
                        !(this_items_type == &item_type_from_stack)
                    } else {
                        true
                    }
                });
                Self::next(Status::None, pc)
                */
            }
            Instruction::ForthTree(token) => {
                let Some(StackItem::String(x)) = stack.last() else {
                    return Err("top was not init".to_owned());
                };
                if x != "Init" {
                    return Err("top was not init".to_owned());
                };
                stack.pop();
                *pc = Some((token.clone(), 0));
                Ok(Status::None)
            }
        }
    }
}