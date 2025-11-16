mod behavior;
pub use behavior::Behavior;
mod item;
pub mod layout;
pub use item::Item;
mod treeturn;
pub use treeturn::Treeturn;

type NodeId = String;

#[cfg(test)]
mod tests {
    use qol::{logy, pout};

    use super::*;

    #[test]
    pub fn sequence_test() {
        logy!("trace-behavior-tree", "trace-behavior-tree enabled");
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
        let mut running = vec![String::new()];
        let mut stack = vec![Item::Int(9), Item::Int(3), Item::Int(2), Item::Int(1)];
        let mut c = 0;
        //while !bt.running.is_empty() {
        while c < 15 {
            pout!("\n\n\nLoop:{c}");
            match bt.tick(&mut stack, &mut running) {
                Ok(Treeturn::Success) | Ok(Treeturn::Failure) => break,
                Ok(Treeturn::Running(_)) => (),
                Err(err) => {
                    panic!("Oppsy! Got error: {err}")
                }
            }
            c += 1;
        }
    }
}
