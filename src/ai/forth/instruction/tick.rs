use std::sync::Arc;

use qol::logy;

use crate::sandbox::new_ai::{
    forth::{Instruction, ProgramCounter, ReturnStack, Stack, StackItem},
    Blackboard, BlackboardKey, BlackboardValue, Prayer, Variable,
};

impl Instruction {
    pub fn tick(
        &self,
        stack: &mut Stack,
        return_stack: &mut ReturnStack,
        pc: &mut ProgramCounter,
        blackboard: &mut Blackboard<BlackboardKey, BlackboardValue>,
    ) -> Result<Option<Prayer>, String> {
        logy!("debug", "ticking:{pc:?}:{self:?}");
        println!("Stack is:");
        for c in &*stack {
            println!("    {c:?}");
        }

        let ret = match self {
            Instruction::Action(action_id) => {
                //*pc = return_stack.pop();

                return Self::next(Prayer::Inpulse(action_id.clone()).into(), pc);
            }
            Instruction::Debug(_x) => {
                logy!("debug", "{_x}");
                Self::next(None, pc)
            }
            Instruction::Add => {
                if let Ok((nos, tos)) = Self::get_two_ints(stack) {
                    stack.push(StackItem::Int(nos + tos));
                    Self::next(None, pc)
                } else {
                    let (nos, tos) = Self::get_two_coords(stack)?;
                    stack.push(StackItem::Coord {
                        x: nos.0 + tos.0,
                        y: nos.1 + tos.1,
                    });
                    Self::next(None, pc)
                }
            }
            Instruction::Call(token) => {
                let old_pc = std::mem::replace(pc, Some((token.clone(), 0))).unwrap();
                return_stack.push((old_pc.0, old_pc.1 + 1));
                Ok(None)
            }
            Instruction::Distance => {
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
                Self::next(None, pc)
            }
            Instruction::Div => {
                let (nos, tos) = Self::get_two_ints(stack)?;
                stack.push(StackItem::Int(nos / tos));
                Self::next(None, pc)
            }
            Instruction::Drop => {
                if stack.is_empty() {
                    return Err(format!("{}:{}:nothing on sack", file!(), line!()));
                };
                stack.pop();
                Self::next(None, pc)
            }
            Instruction::Dup => {
                let Some(tos) = stack.last() else {
                    return Err(format!("{}:{}:top of stack not a number", file!(), line!()));
                };
                stack.push(tos.clone());
                Self::next(None, pc)
            }
            Instruction::Eq => {
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
                Self::next(None, pc)
            }
            Instruction::FindInInventory => {
                let Some(StackItem::String(_)) = stack.last() else {
                    return Err(format!("{}:{}:tos wasn't a sting", file!(), line!()));
                };
                let Some(StackItem::String(item_class_string)) = stack.pop() else {
                    unreachable!()
                };
                let Ok(item_class) = item_class_string.try_into() else {
                    return Err(format!("{}:{}:item class was not valid", file!(), line!()));
                };
                Self::next(Prayer::FindInInventory { item_class }.into(), pc)
            }
            // ForthFindNearest should set up the CPU for runing the next instruction when it it ticked then pray for the answer to be put on the stack
            Instruction::FindNearest => {
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
                Self::next(Prayer::FindNearest { x, y, item_class }.into(), pc)
            }
            // ForthGetEnergy should set up the CPU for runing the next instruction when it it ticked then pray for the answer to be put on the stack
            Instruction::GetEnergy => {
                let Some(StackItem::EntityId(_)) = stack.last() else {
                    return Err(format!("{}:{}:tos wasn't an EntityId", file!(), line!()));
                };
                let Some(StackItem::EntityId(entity_id)) = stack.pop() else {
                    unreachable!()
                };
                return Self::next(Prayer::GetEnergy(entity_id.clone()).into(), pc);
            }
            // ForthGetLocation should set up the CPU for runing the next instruction when it it ticked then pray for the answer to be put on the stack
            Instruction::GetLocation => {
                let Some(StackItem::EntityId(_)) = stack.last() else {
                    return Err(format!("{}:{}:tos wasn't an EntityId", file!(), line!()));
                };
                let Some(StackItem::EntityId(entity_id)) = stack.pop() else {
                    unreachable!()
                };
                Self::next(Prayer::GetLocation(entity_id.clone()).into(), pc)
            }
            // ForthGetHP should set up the CPU for runing the next instruction when it it ticked then pray for the answer to be put on the stack
            Instruction::GetHP => {
                let Some(StackItem::EntityId(_)) = stack.last() else {
                    return Err(format!("{}:{}:tos wasn't an EntityId", file!(), line!()));
                };
                let Some(StackItem::EntityId(entity_id)) = stack.pop() else {
                    unreachable!()
                };
                Self::next(Prayer::GetHp(entity_id.clone()).into(), pc)
            }
            Instruction::GE => {
                let (nos, tos) = Self::get_two_ints(stack)?;
                if nos >= tos {
                    stack.push(StackItem::True);
                } else {
                    stack.push(StackItem::False);
                }
                Self::next(None, pc)
            }
            Instruction::GetBlackboard => {
                let Some(StackItem::String(_)) = stack.last() else {
                    return Err(format!("{}:{}:tos wasn't a sting", file!(), line!()));
                };
                let Some(StackItem::String(key)) = stack.pop() else {
                    unreachable!()
                };

                stack.push(match blackboard.get(&*key) {
                    Some(x) => StackItem::Option(Box::new(x.into())),
                    None => StackItem::none(),
                });
                Self::next(None, pc)
            }
            Instruction::GT => {
                let (nos, tos) = Self::get_two_ints(stack)?;
                if nos > tos {
                    stack.push(StackItem::True);
                } else {
                    stack.push(StackItem::False);
                }
                Self::next(None, pc)
            }
            Instruction::If(skip) => {
                let Some((_, idx)) = pc else {
                    return Err(format!("{}:{}:unexptect end of program", file!(), line!()));
                };
                *idx += 1;
                if Some(StackItem::True) != stack.pop() {
                    *idx += skip;
                }
                Ok(None)
            }
            Instruction::InventoryGE => {
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
                    Prayer::GetIsInventoryGE {
                        agent,
                        item_class,
                        amount,
                    }.into(),
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
                Self::next(None, pc)
            }
            Instruction::Jump(token) => {
                *pc = Some((token.clone(), 0));
                Ok(None)
            }
            Instruction::LE => {
                let (nos, tos) = Self::get_two_ints(stack)?;
                if nos <= tos {
                    stack.push(StackItem::True);
                } else {
                    stack.push(StackItem::False);
                }
                Self::next(None, pc)
            }
            Instruction::LT => {
                let (nos, tos) = Self::get_two_ints(stack)?;
                if nos < tos {
                    stack.push(StackItem::True);
                } else {
                    stack.push(StackItem::False);
                }
                Self::next(None, pc)
            }
            Instruction::Lit(value) => {
                stack.push(value.clone());
                Self::next(None, pc)
            }
            Instruction::Mul => {
                let (nos, tos) = Self::get_two_ints(stack)?;
                stack.push(StackItem::Int(nos * tos));
                Self::next(None, pc)
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
                    None => return Err(format!("{}:{}:no top of stack", file!(), line!())),
                };
                Self::next(None, pc)
            }
            Instruction::Or => {
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
                Self::next(None, pc)
            }

            Instruction::Rem => {
                let (nos, tos) = Self::get_two_ints(stack)?;
                stack.push(StackItem::Int(nos % tos));
                Self::next(None, pc)
            }
            Instruction::Rot => {
                if stack.len() < 3 {
                    return Err(format!("{}:{}:less that 3 items on stack", file!(), line!()));
                };
                let x = stack.remove(stack.len() - 3);
                stack.push(x);
                Self::next(None, pc)
            }
            // like "!"'s stack diagram is "( n adr -- ). This uses TOS of the key and stores NOS under it
            Instruction::SetBlackboard => {
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
                blackboard.insert((*key).clone(), Variable::Chit(nos.into()));
                Self::next(None, pc)
            }
            Instruction::Stuff => {
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
                Self::next(None, pc)
            }
            Instruction::SomeCoord => {
                let Some(StackItem::Option(x)) = stack.last() else {
                    stack.push(StackItem::False);
                    return Self::next(None, pc);
                };
                match x.as_ref() {
                    StackItem::Coord { .. } => (),
                    _ => {
                        stack.push(StackItem::False);
                        return Self::next(None, pc);
                    }
                }
                let Some(StackItem::Option(y)) = stack.pop() else {
                    unreachable!()
                };
                stack.push(Box::into_inner(y));
                stack.push(StackItem::True);
                Self::next(None, pc)
            }
            Instruction::SomeEntityId => {
                let Some(StackItem::Option(x)) = stack.last() else {
                    stack.push(StackItem::False);
                    return Self::next(None, pc);
                };
                match x.as_ref() {
                    StackItem::EntityId(_) => (),
                    _ => {
                        stack.push(StackItem::False);
                        return Self::next(None, pc);
                    }
                }
                let Some(StackItem::Option(y)) = stack.pop() else {
                    unreachable!()
                };
                stack.push(Box::into_inner(y));
                stack.push(StackItem::True);
                Self::next(None, pc)
            }
            Instruction::IsEmpty => {
                let Some(StackItem::Table(x)) = stack.last() else {
                    return Err(format!("{}:{}:TOS wasn't a table", file!(), line!()));
                };
                let map_empty_ka = x.map.is_empty();
                stack.push(if map_empty_ka {
                    StackItem::True
                } else {
                    StackItem::False
                });
                Self::next(None, pc)
            }
            Instruction::PopLast => {
                let Some(StackItem::Table(x)) = stack.last_mut() else {
                    return Err(format!("{}:{}:TOS wasn't a table", file!(), line!()));
                };
                let last_maybe = Arc::make_mut(x).map.pop_last();

                if let Some((_, last)) = last_maybe {
                    stack.push(StackItem::some(last));
                } else {
                    stack.push(StackItem::False);
                };
                Self::next(None, pc)
            }
            Instruction::SomeInt => {
                let Some(StackItem::Option(x)) = stack.last() else {
                    stack.push(StackItem::False);
                    return Self::next(None, pc);
                };
                match x.as_ref() {
                    StackItem::Int(_) => (),
                    _ => {
                        stack.push(StackItem::False);
                        return Self::next(None, pc);
                    }
                }
                let Some(StackItem::Option(y)) = stack.pop() else {
                    unreachable!()
                };
                stack.push(Box::into_inner(y));
                stack.push(StackItem::True);
                Self::next(None, pc)
            }
            Instruction::Sub => {
                if let Ok((nos, tos)) = Self::get_two_ints(stack) {
                    stack.push(StackItem::Int(nos - tos));
                    Self::next(None, pc)
                } else {
                    let (nos, tos) = Self::get_two_coords(stack)?;
                    stack.push(StackItem::Coord {
                        x: nos.0 - tos.0,
                        y: nos.1 - tos.1,
                    });
                    Self::next(None, pc)
                }
            }
            Instruction::Swap => {
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
                Self::next(None, pc)
            }
            Instruction::Return => Self::exit(None, return_stack, pc),
            // ToDoGetEntities should set up the CPU for runing the next instruction when it it ticked then pray for the answer to be put on the stack
            Instruction::GetEntities => {
                let ((min_x, min_y), (max_x, max_y)) = Self::get_two_coords(stack)?;
                Self::next(
                    Prayer::GetEntities {
                        min_x,
                        min_y,
                        max_x,
                        max_y,
                    }.into(),
                    pc,
                )
            }
            Instruction::RemoveEntitiesOfType => {
                let Some(StackItem::String(stack_string)) = stack.last() else {
                    return Err(format!("{}:{}:top of stack not a number", file!(), line!()));
                };
                let stack_str: &str = stack_string;
                let Ok(item_type_from_stack) = stack_str.try_into() else {
                    return Err(format!("{}:{}:couldn't convert {stack_str:?} to type", file!(), line!()));
                };
                stack.pop();
                Self::next(Some(Prayer::RemoveEntitiesOfType(item_type_from_stack)), pc)
            }
            Instruction::RetainEntitiesOfType => {
                let Some(StackItem::String(stack_string)) = stack.last() else {
                    return Err(format!("{}:{}:top of stack not a number", file!(), line!()));
                };
                let stack_str: &str = stack_string;
                let Ok(item_type_from_stack) = stack_str.try_into() else {
                    return Err(format!("{}:{}:couldn't convert {stack_str:?} to type", file!(), line!()));
                };
                stack.pop();
                Self::next(Prayer::RetainEntitiesOfType(item_type_from_stack).into(), pc)
            }
        };
        println!("ending Stack is:");
        for c in &*stack {
            println!("    {c:?}");
        }
        ret
    }
}
