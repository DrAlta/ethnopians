use qol::{logy, pout};

use super::{Item, NodeId, Treeturn};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Behavior {
    Sequence { behaviors: Vec<Behavior> },
    Fallback { behaviors: Vec<Behavior> },
    Add,
    Print,
    Lit(i64),
}

impl Behavior {
    pub fn tick(
        &self,
        stack: &mut Vec<Item>,
        running: &mut Vec<NodeId>,
    ) -> Result<Treeturn, String> {
        logy!(
            "trace-behavior-tree",
            "----\nDoing {self:?}\nPC:{:?}\nstack:{:?}",
            running.last(),
            stack
        );

        match self {
            Behavior::Sequence { behaviors } => Self::sequence_tick(behaviors, stack, running),
            Behavior::Fallback { behaviors } => Self::fallback_tick(behaviors, stack, running),
            Behavior::Add => Self::add_tick(stack, running),
            Behavior::Print => Self::print_tick(stack, running),
            Behavior::Lit(lit) => Self::lit_tick(*lit, stack, running),
        }
    }

    pub fn fallback_tick(
        _behaviors: &Vec<Behavior>,
        _stack: &mut Vec<Item>,
        _running: &mut Vec<NodeId>,
    ) -> Result<Treeturn, String> {
        todo!()
    }

    pub fn sequence_tick(
        behaviors: &Vec<Behavior>,
        stack: &mut Vec<Item>,
        running: &mut Vec<NodeId>,
    ) -> Result<Treeturn, String> {
        for (program_counter, behavoir) in behaviors.iter().enumerate() {
            if let Some(thing) = running.last() {
                running.push(format!("{thing}:{program_counter}"));
            } else {
                running.push(format!("{program_counter}"));
            }

            match behavoir.tick(stack, running)? {
                Treeturn::Success => {
                    logy!("trace-behavior-tree", "seq last behavior successed");
                    continue;
                }
                Treeturn::Failure => {
                    running.pop();
                    return Ok(Treeturn::Failure);
                }
                Treeturn::Running(mut vec) => {
                    running.pop();
                    return Ok(Treeturn::Running({
                        vec.push(format!("{}", line!()));
                        vec
                    }));
                }
            }
        }

        logy!("trace-behavior-tree", "seq successed");
        running.pop();
        return Ok(Treeturn::Success);
    }

    pub fn add_tick(stack: &mut Vec<Item>, running: &mut Vec<NodeId>) -> Result<Treeturn, String> {
        running.pop();
        let Some(Item::Int(a)) = stack.pop() else {
            return Err("top of stack wasn't an int".into());
        };
        let Some(Item::Int(b)) = stack.pop() else {
            return Err("second of stack wasn't an int".into());
        };
        let x = a + b;
        stack.push(Item::Int(x));
        return Ok(Treeturn::Success);
    }

    pub fn lit_tick(
        lit: i64,
        stack: &mut Vec<Item>,
        running: &mut Vec<NodeId>,
    ) -> Result<Treeturn, String> {
        running.pop();
        stack.push(Item::Int(lit));
        return Ok(Treeturn::Success);
    }

    pub fn print_tick(
        stack: &mut Vec<Item>,
        running: &mut Vec<NodeId>,
    ) -> Result<Treeturn, String> {
        running.pop();

        let Some(a) = stack.pop() else {
            return Err("Nothing on stack to print!".into());
        };
        pout!("{:?}", a);
        return Ok(Treeturn::Success);
    }
}
