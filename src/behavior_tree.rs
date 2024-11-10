mod behavior;
pub use behavior::Behavior;
mod item;
pub use item::Item;
mod treeturn;
pub use treeturn::Treeturn;

type NodeID = String;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn sequence_test() {
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
            println!("\n\n\nLoop:{c}");
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
