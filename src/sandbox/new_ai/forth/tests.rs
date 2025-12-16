use qol::{assert_specimen, pout};

use crate::sandbox::new_ai::{
    Blackboard, Prayer, forth::{Instruction, ThreadPool, cpu::CPU}
};

#[test]
fn step_test() {
    let mut blackboard = Blackboard::new();

    let mut task_db = ThreadPool::new();
    let action1 = "act1".to_owned();
    task_db.insert(
        action1.clone(),
        vec![Instruction::Action(1), Instruction::Return],
    );
    let action3 = "act3".to_owned();
    task_db.insert(
        action3.clone(),
        vec![
            Instruction::Drop,
            Instruction::Call(action1.clone()),
            Instruction::Return,
        ],
    );
    let mut cpu = CPU::load(action3.clone());
    //step 1 selectpr does its init and sets the cpu up to call its first child, sequence.
    pout!("ticking:{:?}", cpu.pc);
    assert_specimen!(cpu.step(&task_db, &mut blackboard), Ok(None));
    pout!("ticking:{:#?}", cpu);

    assert_specimen!(&cpu.stack, &vec![]);
    assert_specimen!(&cpu.pc, &Some((action3.clone(), 1)));
    // step 2
    pout!("ticking:{:?}", cpu.pc);
    assert_specimen!(cpu.step(&task_db, &mut blackboard), Ok(None));
    pout!("ticking:{:#?}", cpu);

    assert_specimen!(&cpu.stack, &vec![]);
    assert_specimen!(&cpu.return_stack, &vec![(action3.clone(), 2)]);

    assert_specimen!(&cpu.pc, &Some((action1.clone(), 0)));
    // step 3
    pout!("ticking:{:?}", cpu.pc);
    assert_specimen!(cpu.step(&task_db, &mut blackboard), Ok(Prayer::Inpulse(1).into()));
    pout!("ticking:{:#?}", cpu);

    assert_specimen!(&cpu.stack, &vec![]);
    assert_specimen!(&cpu.return_stack, &vec![(action3.clone(), 2)]);

    assert_specimen!(&cpu.pc, &Some((action1.clone(), 1)));
    // step 4
    pout!("ticking:{:?}", cpu.pc);
    assert_specimen!(cpu.step(&task_db, &mut blackboard), Ok(None));
    pout!("ticking:{:#?}", cpu);

    assert_specimen!(&cpu.stack, &vec![]);
    assert_specimen!(&cpu.return_stack, &vec![]);

    assert_specimen!(&cpu.pc, &Some((action3.clone(), 2)));

    // step 5
    pout!("ticking:{:?}", cpu.pc);
    assert_specimen!(cpu.step(&task_db, &mut blackboard), Ok(None));
    pout!("ticking:{:#?}", cpu);

    assert_specimen!(&cpu.stack, &vec![]);
    assert_specimen!(&cpu.return_stack, &vec![]);

    assert_specimen!(&cpu.pc, &None);
}
