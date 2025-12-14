use std::sync::Arc;

use qol::logy;

use crate::sandbox::ai::{
    cpu::{
        instruction::{tick_selector, tick_sequence},
        Instruction, Prayer, ProgramCounter, ReturnStack, Stack,
    },
    Blackboard, BlackboardKey, BlackboardValue, InpulseId, StackItem, Status,
};

impl Instruction {
    pub fn tick(
        &self,
        stack: &mut Stack,
        return_stack: &mut ReturnStack,
        pc: &mut ProgramCounter,
        blackboard: &mut Blackboard<BlackboardKey, BlackboardValue>,
    ) -> Prayer {
        logy!("debug", "ticking:{pc:?}:{self:?}");
        println!("Stack is:");
        for c in &*stack {
            println!("    {c:?}");
        }

        let ret = match self {
            Instruction::ForthAction(action_id) => {
                //*pc = return_stack.pop();

                return Self::next(Status::Running(action_id.clone()), pc);
            }
            Instruction::Combine(a, b) => {
                let Some(thing) = blackboard.get(a) else {
                    return Err(format!("{}:{}:couldn't find first object:{a} in blackboard", file!(), line!()));
                };
                let aa = thing.into();

                let Some(thing) = blackboard.get(b) else {
                    return Err(format!("{}:{}:couldn't find second object:{b} in blackboard", file!(), line!()));
                };
                let bb = thing.into();

                stack.pop();
                stack.push(aa);
                stack.push(bb);
                *pc = Some(("combine@i".to_owned(), 0));
                Ok(Status::None)
            }
            Instruction::Debug(_x) => {
                logy!("debug", "{_x}");
                Self::next(Status::None, pc)
            }
            Instruction::Eat(x) => {
                let Some(class_id) = blackboard.get(x) else {
                    return Err(format!("{}:{}:couldn't find {x} in blackboard", file!(), line!()));
                };
                match class_id {
                    BlackboardValue::Coord { .. } | BlackboardValue::EntityId(_) => {
                        Err(format!("{}:{}:{x} was an EntityId not a class", file!(), line!()))
                    }
                    BlackboardValue::String(y) => {
                        assert_eq!(Some(StackItem::init()), stack.pop()); // popping in init off the stack
                        let Some(parent_token) = return_stack.pop() else {
                            return Err(format!("{}:{}:nothing to return to", file!(), line!()));
                        };
                        // return to calling fuction
                        *pc = Some(parent_token);
                        Ok(Status::Running(InpulseId::EatClass(y.clone())))
                    }
                }
            }
            Instruction::InventoryGE(key, amount) => {
                let Some(StackItem::String(x)) = stack.last() else {
                    return Err(format!("{}:{}:tos was not an Init", file!(), line!()));
                };
                if **x != "Init" {
                    return Err(format!("{}:{}:tos was not an Init", file!(), line!()));
                };
                stack.pop();
                let Some(&BlackboardValue::EntityId(agent)) = blackboard.get("self") else {
                    return Err(format!("{}:{}:self not found in blackboard", file!(), line!()));
                };
                let Some(BlackboardValue::String(item_class_string)) = blackboard.get(key) else {
                    return Err(format!("{}:{}:{key} not found in blackboard", file!(), line!()));
                };

                let item_class_str: &str = &item_class_string;
                let Ok(item_class) = item_class_str.try_into() else {
                    return Err(format!("{}:{}:{item_class_string} is not a valid item class", file!(), line!()));
                };

                let Some(parent_token) = return_stack.pop() else {
                    return Err(format!("{}:{}:nothing to return to", file!(), line!()));
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
                stack.push(StackItem::init());
                let old_pc = std::mem::replace(pc, Some((token.clone(), *idx))).unwrap();
                return_stack.push((old_pc.0, old_pc.1 + 1));
                Ok(Status::None)
            }
            Instruction::ForthDistance => {
                if stack.len() < 2 {
                    return Err(format!("{}:{}:less that 2 items on stack", file!(), line!()));
                };
                let Some(StackItem::Coord { .. }) = stack.last() else {
                    return Err(format!("{}:{}:top of stack not a number", file!(), line!()));
                };
                let Some(StackItem::Coord { .. }) = stack.get(stack.len() - 2) else {
                    return Err(format!("{}:{}:next of stack not a number", file!(), line!()));
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
                Self::next(Status::None, pc)
            }
            Instruction::ForthDiv => {
                let (nos, tos) = Self::get_two_ints(stack)?;
                stack.push(StackItem::Int(nos / tos));
                Self::next(Status::None, pc)
            }
            Instruction::ForthDrop => {
                if stack.is_empty() {
                    return Err(format!("{}:{}:nothing on sack", file!(), line!()));
                };
                stack.pop();
                Self::next(Status::None, pc)
            }
            Instruction::ForthDup => {
                let Some(tos) = stack.last() else {
                    return Err(format!("{}:{}:top of stack not a number", file!(), line!()));
                };
                stack.push(tos.clone());
                Self::next(Status::None, pc)
            }
            Instruction::ForthEq => {
                if stack.len() < 2 {
                    return Err(format!("{}:{}:less that two items on the stack", file!(), line!()));
                };
                let tos = stack.pop().unwrap();
                let nos = stack.pop().unwrap();
                if nos == tos {
                    stack.push(StackItem::True);
                } else {
                    stack.push(StackItem::False);
                }
                Self::next(Status::None, pc)
            }
            Instruction::ForthFindInInventory => {
                let Some(StackItem::String(_)) = stack.last() else {
                    return Err(format!("{}:{}:tos wasn't a sting", file!(), line!()));
                };
                let Some(StackItem::String(item_class_string)) = stack.pop() else {
                    unreachable!()
                };
                let Ok(item_class) = item_class_string.try_into() else {
                    return Err(format!("{}:{}:item class was not valid", file!(), line!()));
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
                    return Err(format!("{}:{}:less that 2 items on stack", file!(), line!()));
                };
                let Some(StackItem::String(_)) = stack.last() else {
                    return Err(format!("{}:{}:tos wasn't a sting", file!(), line!()));
                };
                let Some(StackItem::Coord { .. }) = stack.get(stack.len() - 2) else {
                    return Err(format!("{}:{}:nos wasn't an coord", file!(), line!()));
                };
                let Some(StackItem::String(item_class_string)) = stack.pop() else {
                    unreachable!()
                };
                let Some(StackItem::Coord { x, y }) = stack.pop() else {
                    unreachable!()
                };
                let Ok(item_class) = item_class_string.try_into() else {
                    return Err(format!("{}:{}:item class was not valid", file!(), line!()));
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
                    return Err(format!("{}:{}:tos wasn't an EntityId", file!(), line!()));
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
                    return Err(format!("{}:{}:tos wasn't an EntityId", file!(), line!()));
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
                let Some(StackItem::EntityId(_)) = stack.last() else {
                    return Err(format!("{}:{}:tos wasn't an EntityId", file!(), line!()));
                };
                let Some(StackItem::EntityId(entity_id)) = stack.pop() else {
                    unreachable!()
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
                    return Err(format!("{}:{}:tos wasn't a sting", file!(), line!()));
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
                stack.push(match blackboard.get(&*key) {
                    Some(x) => StackItem::Option(Box::new(x.into())),
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
                    return Err(format!("{}:{}:unexptect end of program", file!(), line!()));
                };
                *idx += 1;
                if Some(StackItem::True) != stack.pop() {
                    *idx += skip;
                }
                Ok(Status::None)
            }
            Instruction::ForthInventoryGE => {
                if stack.len() < 2 {
                    return Err(format!("{}:{}:less that 2 items on stack", file!(), line!()));
                };
                let Some(StackItem::String(_)) = stack.last() else {
                    return Err(format!("{}:{}:tos was not an string", file!(), line!()));
                };
                let Some(StackItem::Int(_)) = stack.get(stack.len() - 2) else {
                    return Err(format!("{}:{}:nos was not an Int", file!(), line!()));
                };
                let Some(StackItem::String(item_class_string)) = stack.pop() else {
                    return Err(format!("{}:{}:tos was not an string", file!(), line!()));
                };
                let Some(StackItem::Int(amount)) = stack.pop() else {
                    return Err(format!("{}:{}:nos was not an INt", file!(), line!()));
                };
                let item_class_str: &str = &item_class_string;
                let Ok(item_class) = item_class_str.try_into() else {
                    return Err(format!("{}:{}:{item_class_string} is not a valid item class", file!(), line!()));
                };

                let Some(&BlackboardValue::EntityId(agent)) = blackboard.get("self") else {
                    return Err(format!("{}:{}:self not found in blackboard", file!(), line!()));
                };
                Self::next(
                    Status::GetIsInventoryGE {
                        agent,
                        item_class,
                        amount,
                    },
                    pc,
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
                stack.push(StackItem::init());
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
                    }
                    Some(_) => {
                        stack.push(StackItem::True);
                    }
                    None => return Err(format!("{}:{}:no top of stack", file!(), line!())),
                };
                Self::next(Status::None, pc)
            }
            Instruction::ForthOr => {
                if stack.len() < 2 {
                    return Err(format!("{}:{}:less that 2 items on stack", file!(), line!()));
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
            Instruction::ForthRot => {
                if stack.len() < 3 {
                    return Err(format!("{}:{}:less that 3 items on stack", file!(), line!()));
                };
                let x = stack.remove(stack.len() - 3);
                stack.push(x);
                Self::next(Status::None, pc)
            }
            // like "!"'s stack diagram is "( n adr -- ). This uses TOS of the key and stores NOS under it
            Instruction::ForthSetBlackboard => {
                if stack.len() < 2 {
                    return Err(format!("{}:{}:less that 2 items on stack", file!(), line!()));
                };
                let Some(StackItem::String(_)) = stack.last() else {
                    return Err(format!("{}:{}:tos not a string", file!(), line!()));
                };
                let Some(StackItem::String(key)) = stack.pop() else {
                    unreachable!()
                };
                let Some(nos) = stack.pop() else {
                    unreachable!()
                };
                blackboard.insert(
                    (*key).clone(),
                    crate::sandbox::ai::Variable::Chit(nos.into()),
                );
                Self::next(Status::None, pc)
            }
            Instruction::ForthStuff => {
                let Some(StackItem::Table(_)) = stack.get(stack.len() - 3) else {
                    return Err(format!("{}:{}:3rd item wasn't a table", file!(), line!()));
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
                    return Err(format!("{}:{}:TOS wasn't a table", file!(), line!()));
                };
                let map_empty_ka = x.map.is_empty();
                stack.push(if map_empty_ka {
                    StackItem::True
                } else {
                    StackItem::False
                });
                Self::next(Status::None, pc)
            }
            Instruction::ForthPopLast => {
                let Some(StackItem::Table(x)) = stack.last_mut() else {
                    return Err(format!("{}:{}:TOS wasn't a table", file!(), line!()));
                };
                let last_maybe = Arc::make_mut(x).map.pop_last();

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
                    return Err(format!("{}:{}:less that 2 items on stack", file!(), line!()));
                };
                let Some(_) = stack.get(stack.len() - 2) else {
                    return Err(format!("{}:{}:no nos", file!(), line!()));
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
                    return Err(format!("{}:{}:world had no SpatailBloom".to_owned());
                };
                let mut x =Vec::new();
                for y in sb.qurry(nos.0 as f32,nos.1 as f32, tos.0 as f32, tos.1 as f32) {
                    let Some(thing) = map.get(&y) else {
                        return Err(format!("{}:{}:SpatialBloom returned {y:?} but that id isn't mapped to any entities", file!(), line!()))
                    };
                    x.push(thing.clone());
                };
                stack.push(StackItem::Todo(x));
                Self::next(Status::None, pc)
                */
            }
            Instruction::ForthRemoveEntitiesOfType => {
                let Some(StackItem::String(stack_string)) = stack.last() else {
                    return Err(format!("{}:{}:top of stack not a number", file!(), line!()));
                };
                let stack_str: &str = stack_string;
                let Ok(item_type_from_stack) = stack_str.try_into() else {
                    return Err(format!("{}:{}:couldn't convert {stack_str:?} to type", file!(), line!()));
                };
                stack.pop();
                Self::next(Status::RemoveEntitiesOfType(item_type_from_stack), pc)

                /* pre bevy impl
                let Some(StackItem::Todo(_)) = stack.get(stack.len() - 2) else {
                    return Err(format!("{}:{}:next of stack not a number".into());
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
                    return Err(format!("{}:{}:top of stack not a number", file!(), line!()));
                };
                let stack_str: &str = stack_string;
                let Ok(item_type_from_stack) = stack_str.try_into() else {
                    return Err(format!("{}:{}:couldn't convert {stack_str:?} to type", file!(), line!()));
                };
                stack.pop();
                Self::next(Status::RetainEntitiesOfType(item_type_from_stack), pc)

                /* pre bevy impl
                let Some(StackItem::Todo(_)) = stack.get(stack.len() - 2) else {
                    return Err(format!("{}:{}:next of stack not a number".into());
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
                    return Err(format!("{}:{}:top was not init", file!(), line!()));
                };
                if **x != "Init" {
                    return Err(format!("{}:{}:top was not init", file!(), line!()));
                };
                stack.pop();
                *pc = Some((token.clone(), 0));
                Ok(Status::None)
            }
        };
        println!("ending Stack is:");
        for c in &*stack {
            println!("    {c:?}");
        }
        ret
    }
}
