use crate::sandbox::ai::{
    parser::{balanced_parser::Tract, TreesUsed},
    Instruction, Thread, TaskPool,
};

pub(super) trait If {
    fn flatten(self) -> (Thread, TaskPool);
}
impl If for Vec<Tract<(Thread, TaskPool)>> {
    fn flatten(self) -> (Thread, TaskPool) {
        let mut thread = vec![Instruction::ForthIf(0)];
        let hash_map = TreesUsed::new();
        let mut used = hash_map;
        let mut count = 0;
        for x in self {
            match x {
                Tract::Item(mut item) => {
                    count += item.0.len();
                    thread.append(&mut item.0);
                    used.extend(item.1.into_iter());
                }
                Tract::Level(vec) => {
                    let mut item = vec.flatten();
                    count += item.0.len();
                    thread.append(&mut item.0);
                    used.extend(item.1.into_iter());
                }
            }
        }
        let Some(Instruction::ForthIf(x)) = thread.first_mut() else {
            panic!()
        };
        *x = count;

        (thread, used)
    }
}
