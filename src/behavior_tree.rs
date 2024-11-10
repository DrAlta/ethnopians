type NodeID = String;

pub enum Treeturn {
    Success,
    Falure,
    Running(Vec<String>),
}

mod behavior;
pub use behavior::Behavior;
mod item;
pub use item::Item;
#[allow(unused_imports)]
use qol::logy;
/*
#[derive(Debug)]
pub struct BehaviorTree {
    pub nodes: Vec<Behavior>,
    pub running: Vec<NodeID>,
    pub stack: Vec<Item>,

}
impl BehaviorTree {
    pub fn tick(&mut self) -> Result<(), String> {
        let Some(last) = self.running.last() else {
            return Err("Nothing on `running`".into());
        };
        let Some(behavior) = self.nodes.get_mut(*last) else {
            return Err(format!("No Behavior {}", last));
        };
        logy!("trace-behavior-tree", "doing: {behavior:?}");
        match behavior.tick(&mut self.stack, &mut self.running) {
            Ok(true) => {
                let Some(last2) = self.running.last() else {
                    return Err("Nothing on `running`".into());
                };
                let Some(next) = self.nodes.get_mut(*last2) else {
                    return Err(format!("No Behavior {}", last2));
                };
                next.start(&mut self.stack, &mut self.running)?;
                Ok(())

            },
            Ok(false) => Ok(()),
            Err(err) => Err(err),
        }
    }
}
*/
#[test]
pub fn main() {
    println!("output!");
    #[cfg(feature = "trace-behavior-tree")]
    println!("trace-behavior-tree enabled");
    let bt = Behavior::Sequence {
        behaviors: vec![
            Behavior::Sequence {
                behaviors: vec![Behavior::Add, Behavior::Add],
            },
            Behavior::Print,
            Behavior::Lit(60),
            Behavior::Add,
            Behavior::Print,
        ],
    };
    let mut running = vec!["".to_owned()];
    let mut stack = vec![Item::Int(9), Item::Int(3), Item::Int(2), Item::Int(1)];
    let mut c = 0;
    //while !bt.running.is_empty() {
    while c < 15 {
        logy!("trace-behavior-tree", "\n\n\nLoop:{c}");
        match bt.tick(&mut stack, &mut running) {
            Ok(Treeturn::Success) => break,
            Ok(_) => (),
            Err(err) => {
                panic!("{}", err)
            }
        }
        c += 1;
    }
    panic!("success")
}
