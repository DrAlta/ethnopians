//! for the test aI just have it plant vegtibles in a field and harvest them ehen they are mature then replant them
//! if out od seed find neared plant to collect seeds from 
//! i thing a veg can be split into 3 seeds
//! useing hands on a plant produces vegs and consumes the plant
//! use an knife of a veg produces 3 seeds and consumes the veg
//! 
//! use a stone on stone produces a knife and consomes one stone
//! 
//! useinga knife on stick or visvera produces a axe and consumes the knife and stick
//! 
//! knife has higher DPS than axe but shorter range


#[cfg(test)]
mod tests {
    use crate::sandbox::bt::{cpu::CPU, get_hermit_behavoir_tree, Status};


    #[test]
    pub fn foo() {
        let bt = get_hermit_behavoir_tree(); 
        let mut cpu = CPU::load("hermit".to_owned());
        loop {
            match cpu.step(&bt) {
                Ok(status) => {
                    match status {
                        Status::Success => todo!(),
                        Status::Failure => todo!(),
                        Status::Running(_inpulse_id) => todo!(),
                        Status::None => todo!(),
                    }
                },
                Err(_) => todo!(),
            }
        }
    }
}