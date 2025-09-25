use std::sync::Arc;

use qol::logy;

use crate::sandbox::new_ai::{forth::{Instruction, Prayer, ProgramCounter, ReturnStack, Stack, StackItem, Status}, Blackboard, BlackboardKey, BlackboardValue, Variable};

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
            Instruction::Action(action_id) => {
                //*pc = return_stack.pop();

                return Self::next(Status::Running(action_id.clone()), pc);
            }
            Instruction::Debug(_x) => {
                logy!("debug", "{_x}");
                Self::next(Status::None, pc)
            }
            Instruction::Add => {
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
            Instruction::Call(token) => {
                let old_pc = std::mem::replace(pc, Some((token.clone(), 0))).unwrap();
                return_stack.push((old_pc.0, old_pc.1 + 1));
                Ok(Status::None)
            }
            Instruction::Distance => {
                if stack.len() < 2 {
                    return Err("less that 2 items on stack".to_owned());
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
                Self::next(Status::None, pc)
            }
            Instruction::Div => {
                let (nos, tos) = Self::get_two_ints(stack)?;
                stack.push(StackItem::Int(nos / tos));
                Self::next(Status::None, pc)
            }
            Instruction::Drop => {
                if stack.is_empty() {
                    return Err("nothing on sack".into());
                };
                stack.pop();
                Self::next(Status::None, pc)
            }
            Instruction::Dup => {
                let Some(tos) = stack.last() else {
                    return Err("top of stack not a number".into());
                };
                stack.push(tos.clone());
                Self::next(Status::None, pc)
            }
            Instruction::Eq => {
                if stack.len() < 2 {
                    return Err("less that two items on the stack".to_owned());
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
            Instruction::FindInInventory => {
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
            }
            // ForthFindNearest should set up the CPU for runing the next instruction when it it ticked then pray for the answer to be put on the stack
            Instruction::FindNearest => {
                if stack.len() < 2 {
                    return Err("less that 2 items on stack".to_owned());
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
            }
            // ForthGetEnergy should set up the CPU for runing the next instruction when it it ticked then pray for the answer to be put on the stack
            Instruction::GetEnergy => {
                let Some(StackItem::EntityId(_)) = stack.last() else {
                    return Err("tos wasn't an EntityId".to_owned());
                };
                let Some(StackItem::EntityId(entity_id)) = stack.pop() else {
                    unreachable!()
                };
                return Self::next(Status::GetEnergy(entity_id.clone()), pc);
            }
            // ForthGetLocation should set up the CPU for runing the next instruction when it it ticked then pray for the answer to be put on the stack
            Instruction::GetLocation => {
                let Some(StackItem::EntityId(_)) = stack.last() else {
                    return Err("tos wasn't an EntityId".to_owned());
                };
                let Some(StackItem::EntityId(entity_id)) = stack.pop() else {
                    unreachable!()
                };
                Self::next(Status::GetLocation(entity_id.clone()), pc)
            }
            // ForthGetHP should set up the CPU for runing the next instruction when it it ticked then pray for the answer to be put on the stack
            Instruction::GetHP => {
                let Some(StackItem::EntityId(_)) = stack.last() else {
                    return Err("tos wasn't an EntityId".to_owned());
                };
                let Some(StackItem::EntityId(entity_id)) = stack.pop() else {
                    unreachable!()
                };
                Self::next(Status::GetHp(entity_id.clone()), pc)
            }
            Instruction::GE => {
                let (nos, tos) = Self::get_two_ints(stack)?;
                if nos >= tos {
                    stack.push(StackItem::True);
                } else {
                    stack.push(StackItem::False);
                }
                Self::next(Status::None, pc)
            }
            Instruction::GetBlackboard => {
                let Some(StackItem::String(_)) = stack.last() else {
                    return Err("tos wasn't a sting".to_owned());
                };
                let Some(StackItem::String(key)) = stack.pop() else {
                    unreachable!()
                };

                stack.push(match blackboard.get(&*key) {
                    Some(x) => StackItem::Option(Box::new(x.into())),
                    None => StackItem::none(),
                });
                Self::next(Status::None, pc)
            }
            Instruction::GT => {
                let (nos, tos) = Self::get_two_ints(stack)?;
                if nos > tos {
                    stack.push(StackItem::True);
                } else {
                    stack.push(StackItem::False);
                }
                Self::next(Status::None, pc)
            }
            Instruction::If(skip) => {
                let Some((_, idx)) = pc else {
                    return Err("unexptect end of program".to_owned());
                };
                *idx += 1;
                if Some(StackItem::True) != stack.pop() {
                    *idx += skip;
                }
                Ok(Status::None)
            }
            Instruction::InventoryGE => {
                if stack.len() < 2 {
                    return Err("less that 2 items on stack".to_owned());
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
                    pc,
                )
            }
            Instruction::IsInt => {
                let value = if let Some(StackItem::Int(_)) = stack.last() {
                    StackItem::True
                } else {
                    StackItem::False
                };
                stack.push(value);
                Self::next(Status::None, pc)
            }
            Instruction::Jump(token) => {
                *pc = Some((token.clone(), 0));
                Ok(Status::None)
            }
            Instruction::LE => {
                let (nos, tos) = Self::get_two_ints(stack)?;
                if nos <= tos {
                    stack.push(StackItem::True);
                } else {
                    stack.push(StackItem::False);
                }
                Self::next(Status::None, pc)
            }
            Instruction::LT => {
                let (nos, tos) = Self::get_two_ints(stack)?;
                if nos < tos {
                    stack.push(StackItem::True);
                } else {
                    stack.push(StackItem::False);
                }
                Self::next(Status::None, pc)
            }
            Instruction::Lit(value) => {
                stack.push(value.clone());
                Self::next(Status::None, pc)
            }
            Instruction::Mul => {
                let (nos, tos) = Self::get_two_ints(stack)?;
                stack.push(StackItem::Int(nos * tos));
                Self::next(Status::None, pc)
            }
            Instruction::NotTrue => {
                //stack.push(ifSome(StackItem::True) ==
                match stack.pop() {
                    Some(StackItem::True) => {
                        stack.push(StackItem::False);
                    }
                    Some(_) => {
                        stack.push(StackItem::True);
                    }
                    None => return Err("no top of stack".to_owned()),
                };
                Self::next(Status::None, pc)
            }
            Instruction::Or => {
                if stack.len() < 2 {
                    return Err("less that 2 items on stack".to_owned());
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

            Instruction::Rem => {
                let (nos, tos) = Self::get_two_ints(stack)?;
                stack.push(StackItem::Int(nos % tos));
                Self::next(Status::None, pc)
            }
            Instruction::Rot => {
                if stack.len() < 3 {
                    return Err("less that 3 items on stack".to_owned());
                };
                let x = stack.remove(stack.len() - 3);
                stack.push(x);
                Self::next(Status::None, pc)
            }
            // like "!"'s stack diagram is "( n adr -- ). This uses TOS of the key and stores NOS under it
            Instruction::SetBlackboard => {
                if stack.len() < 2 {
                    return Err("less that 2 items on stack".to_owned());
                };
                let Some(StackItem::String(_)) = stack.last() else {
                    return Err("tos not a string".to_owned());
                };
                let Some(StackItem::String(key)) = stack.pop() else {
                    unreachable!()
                };
                let Some(nos) = stack.pop() else {
                    unreachable!()
                };
                blackboard.insert(
                    (*key).clone(),
                    Variable::Chit(nos.into()),
                );
                Self::next(Status::None, pc)
            }
            Instruction::Stuff => {
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
            Instruction::SomeCoord => {
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
            Instruction::SomeEntityId => {
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
            Instruction::IsEmpty => {
                let Some(StackItem::Table(x)) = stack.last() else {
                    return Err("TOS wasn't a table".to_owned());
                };
                let map_empty_ka = x.map.is_empty();
                stack.push(if map_empty_ka {
                    StackItem::True
                } else {
                    StackItem::False
                });
                Self::next(Status::None, pc)
            }
            Instruction::PopLast => {
                let Some(StackItem::Table(x)) = stack.last_mut() else {
                    return Err("TOS wasn't a table".to_owned());
                };
                let last_maybe = Arc::make_mut(x).map.pop_last();

                if let Some((_, last)) = last_maybe {
                    stack.push(StackItem::some(last));
                } else {
                    stack.push(StackItem::False);
                };
                Self::next(Status::None, pc)
            }
            Instruction::SomeInt => {
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
            Instruction::Sub => {
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
            Instruction::Swap => {
                if stack.len() < 2 {
                    return Err("less that 2 items on stack".to_owned());
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
            Instruction::Return => Self::exit(Status::None, return_stack, pc),
            // ToDoGetEntities should set up the CPU for runing the next instruction when it it ticked then pray for the answer to be put on the stack
            Instruction::GetEntities => {
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
            }
            Instruction::RemoveEntitiesOfType => {
                let Some(StackItem::String(stack_string)) = stack.last() else {
                    return Err("top of stack not a number".into());
                };
                let stack_str: &str = stack_string;
                let Ok(item_type_from_stack) = stack_str.try_into() else {
                    return Err(format!("couldn't convert {stack_str:?} to type"));
                };
                stack.pop();
                Self::next(Status::RemoveEntitiesOfType(item_type_from_stack), pc)
            }
            Instruction::RetainEntitiesOfType => {
                let Some(StackItem::String(stack_string)) = stack.last() else {
                    return Err("top of stack not a number".into());
                };
                let stack_str: &str = stack_string;
                let Ok(item_type_from_stack) = stack_str.try_into() else {
                    return Err(format!("couldn't convert {stack_str:?} to type"));
                };
                stack.pop();
                Self::next(Status::RetainEntitiesOfType(item_type_from_stack), pc)

            }
        };
        println!("ending Stack is:");
        for c in &*stack {
            println!("    {c:?}");
        }
        ret
    }
}
