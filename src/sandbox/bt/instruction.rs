use std::collections::BTreeSet;

use crate::sandbox::{
    bt::{
        cpu::{tick_action, tick_selector, tick_sequence, ProgramCounter, ReturnStack, Stack},
        ExecutionToken, InpulseId, ItemId, Status,
    },
    ItemClass, World,
};

use super::{cpu::Prayer, StackItem, ThreadName, TreePool};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Instruction {
    Action(InpulseId),
    Combine(ItemId, ItemId),
    Eat(ItemId),
    InventoryGE(ItemClass, u8),
    Selector(Vec<ExecutionToken>),
    Sequence(Vec<ExecutionToken>),
    Use(ItemId, ItemId),
    //
    ForthGetHP,
    ForthLit(StackItem),
    ForthAdd,
    ForthSub,
    ForthMul,
    ForthDiv,
    ForthRem,
    ForthGT,
    ForthLT,
    ForthGE,
    ForthLE,
    ForthIf(usize),
    ForthCall(ThreadName, usize),
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
            | Instruction::ForthIf(_) => (),
        }
        missing
    }
    pub fn tick(
        &self,
        stack: &mut Stack,
        return_stack: &mut ReturnStack,
        pc: &mut ProgramCounter,
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
            Instruction::ForthGetHP => {
                todo!("replace todo_replace_this{:?}", {
                    let todo_replace_this = 0;
                    let Some(hp) = world.get_hp(&todo_replace_this) else {
                        stack.push(StackItem::False);
                        return Self::next(Status::None, pc);
                    };
                    stack.push(StackItem::Int(*hp as i32));
                    Self::next(Status::None, pc)
                })
            }
            Instruction::ForthLit(value) => {
                stack.push(value.clone());
                Self::next(Status::None, pc)
            }
            Instruction::ForthAdd => {
                let (nos, tos) = Self::get_two_ints(stack)?;
                stack.push(StackItem::Int(nos + tos));
                Self::next(Status::None, pc)
            }
            Instruction::ForthSub => {
                let (nos, tos) = Self::get_two_ints(stack)?;
                stack.push(StackItem::Int(nos - tos));
                Self::next(Status::None, pc)
            }
            Instruction::ForthMul => {
                let (nos, tos) = Self::get_two_ints(stack)?;
                stack.push(StackItem::Int(nos * tos));
                Self::next(Status::None, pc)
            }
            Instruction::ForthDiv => {
                let (nos, tos) = Self::get_two_ints(stack)?;
                stack.push(StackItem::Int(nos / tos));
                Self::next(Status::None, pc)
            }
            Instruction::ForthRem => {
                let (nos, tos) = Self::get_two_ints(stack)?;
                stack.push(StackItem::Int(nos % tos));
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
            Instruction::ForthLT => {
                let (nos, tos) = Self::get_two_ints(stack)?;
                if nos < tos {
                    stack.push(StackItem::True);
                } else {
                    stack.push(StackItem::False);
                }
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
            Instruction::ForthLE => {
                let (nos, tos) = Self::get_two_ints(stack)?;
                if nos <= tos {
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
            Instruction::ForthCall(token, idx) => {
                *pc = Some((token.clone(), *idx));
                Ok(Status::None)
            }
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
