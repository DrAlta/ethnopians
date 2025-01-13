use crate::sandbox::bt::{
    parser::{forth_parser::balanced::Tract, TreesUsed},
    Instruction, Thread, TreePool,
};

pub(super) trait If {
    fn flatten(self) -> (Thread, TreePool);
}
impl If for Vec<Tract<(Thread, TreePool)>> {
    fn flatten(self) -> (Thread, TreePool) {
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
