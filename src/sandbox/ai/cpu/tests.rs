//use std::collections::HashMap;

use qol::pout;

use super::*;
use crate::sandbox::ai::{
    Blackboard, InpulseId, Instruction, Status, TreePool,
};

#[test]
fn step_test() {
    let mut blackboard = Blackboard::new();

    let mut task_db = TreePool::new();
    let action1 = "act1".to_owned();
    task_db.insert(
        action1.clone(),
        vec![
            Instruction::ForthDrop,
            Instruction::ForthAction(InpulseId::Act1),
            Instruction::ForthReturn,
        ],
    );

    let action2 = "act2".to_owned();
    task_db.insert(
        action2.clone(),
        vec![
            Instruction::ForthDrop,
            Instruction::ForthAction(InpulseId::Act2),
            Instruction::ForthReturn,
        ],
    );
    let action3 = "act3".to_owned();
    task_db.insert(
        action3.clone(),
        vec![
            Instruction::ForthDrop,
            Instruction::ForthAction(InpulseId::Act3),
            Instruction::ForthReturn,
        ],
    );

    let sequence = "seq".to_owned();
    task_db.insert(
        sequence.clone(),
        vec![Instruction::Sequence(vec![
            action1.clone(),
            action2.clone(),
        ])],
    );

    let selector = "sel".to_owned();
    task_db.insert(
        selector.clone(),
        vec![Instruction::Selector(vec![sequence.clone(), action3])],
    );

    /*
        crate::sandbox::ai::task_testing_harness(
            &selector,
            task_db,
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            vec![true],
            blackboard,
            HashMap::new(),
        );
    }
    */
    let mut cpu = CPU::load(selector.clone());
    //step 1 selectpr does its init and sets the cpu up to call its first child, sequence.
    pout!("ticking:{:?}", cpu.pc);
    assert_eq!(cpu.step(&task_db, &mut blackboard), Ok(Status::None));
    assert_eq!(&cpu.stack, &vec![StackItem::selector(1), StackItem::init()]);
    assert_eq!(&cpu.return_stack, &vec![(selector.clone(), 0)]);
    //step 2 sequence intalized and set the cpu up to call its first child, action1
    pout!("ticking:{:?}", cpu.pc);
    assert_eq!(cpu.step(&task_db, &mut blackboard), Ok(Status::None));
    assert_eq!(
        &cpu.stack,
        &vec![
            StackItem::selector(1),
            StackItem::sequence(1),
            StackItem::init()
        ]
    );
    assert_eq!(
        &cpu.return_stack,
        &vec![(selector.clone(), 0), (sequence.clone(), 0)]
    );
    //step 3
    pout!("ticking:{:?}", cpu.pc);
    assert_eq!(cpu.step(&task_db, &mut blackboard), Ok(Status::None));
    assert_eq!(
        &cpu.stack,
        &vec![StackItem::selector(1), StackItem::sequence(1),]
    );
    assert_eq!(
        &cpu.return_stack,
        &vec![(selector.clone(), 0), (sequence.clone(), 0)]
    );
    assert_eq!(&cpu.pc, &Some((action1.clone(), 1)));
    //step 4
    pout!("ticking:{:?}", cpu.pc);
    assert_eq!(
        cpu.step(&task_db, &mut blackboard),
        Ok(Status::Running(InpulseId::Act1))
    );
    assert_eq!(
        &cpu.stack,
        &vec![StackItem::selector(1), StackItem::sequence(1),]
    );
    assert_eq!(
        &cpu.return_stack,
        &vec![(selector.clone(), 0), (sequence.clone(), 0)]
    );
    assert_eq!(&cpu.pc, &Some((action1, 2)));
    cpu.stack.push(StackItem::success()); // push the asnswer to the prayer
                                          //step 5
    pout!("ticking:{:?}", cpu.pc);
    assert_eq!(cpu.step(&task_db, &mut blackboard), Ok(Status::None));
    assert_eq!(
        &cpu.stack,
        &vec![
            StackItem::selector(1),
            StackItem::sequence(1),
            StackItem::success(),
        ]
    );
    assert_eq!(&cpu.return_stack, &vec![(selector.clone(), 0)]);
    //step 6
    pout!("ticking:{:?}", cpu.pc);
    assert_eq!(cpu.step(&task_db, &mut blackboard), Ok(Status::None));
    assert_eq!(
        &cpu.stack,
        &vec![
            StackItem::selector(1),
            StackItem::sequence(2),
            StackItem::init()
        ]
    );
    assert_eq!(&cpu.pc, &Some((action2, 0)));
    assert_eq!(
        &cpu.return_stack,
        &vec![(selector.clone(), 0), (sequence.clone(), 0)]
    );
    //step 7
    pout!("ticking:{:?}", cpu.pc);
    assert_eq!(cpu.step(&task_db, &mut blackboard), Ok(Status::None));
    assert_eq!(
        &cpu.stack,
        &vec![StackItem::selector(1), StackItem::sequence(2),]
    );
    assert_eq!(
        &cpu.return_stack,
        &vec![(selector.clone(), 0), (sequence.clone(), 0)]
    );
    //step 8
    pout!("ticking:{:?}", cpu.pc);
    assert_eq!(
        cpu.step(&task_db, &mut blackboard),
        Ok(Status::Running(InpulseId::Act2))
    );
    assert_eq!(
        &cpu.stack,
        &vec![StackItem::selector(1), StackItem::sequence(2),]
    );
    assert_eq!(
        &cpu.return_stack,
        &vec![(selector.clone(), 0), (sequence, 0)]
    );
    cpu.stack.push(StackItem::failure()); // push answer to prayer onto stack
                                          //step 9
    pout!("ticking:{:?}", cpu.pc);
    assert_eq!(cpu.step(&task_db, &mut blackboard), Ok(Status::None));
    assert_eq!(
        &cpu.stack,
        &vec![
            StackItem::selector(1),
            StackItem::sequence(2),
            StackItem::failure()
        ]
    );
    assert_eq!(&cpu.return_stack, &vec![(selector.clone(), 0)]);
    //step 10
    pout!("ticking:{:?}", cpu.pc);
    assert_eq!(cpu.step(&task_db, &mut blackboard), Ok(Status::None));
    assert_eq!(
        &cpu.stack,
        &vec![StackItem::selector(1), StackItem::failure()]
    );
    assert_eq!(&cpu.return_stack, &ReturnStack::new());
    //step 11
    pout!("ticking:{:?}", cpu.pc);
    assert_eq!(cpu.step(&task_db, &mut blackboard), Ok(Status::None));
    assert_eq!(&cpu.stack, &vec![StackItem::selector(2), StackItem::init()]);
    assert_eq!(&cpu.return_stack, &vec![(selector.clone(), 0)]);
    //step 12
    pout!("ticking:{:?}", cpu.pc);
    assert_eq!(cpu.step(&task_db, &mut blackboard), Ok(Status::None));
    assert_eq!(&cpu.stack, &vec![StackItem::selector(2)]);
    assert_eq!(&cpu.return_stack, &vec![(selector.clone(), 0)]);
    //step 13
    pout!("ticking:{:?}", cpu.pc);
    assert_eq!(
        cpu.step(&task_db, &mut blackboard),
        Ok(Status::Running(InpulseId::Act3))
    );
    assert_eq!(&cpu.stack, &vec![StackItem::selector(2)]);
    assert_eq!(&cpu.return_stack, &vec![(selector, 0)]);
    cpu.stack.push(StackItem::success()); //answer the prayer
                                          //step 14
    pout!("ticking:{:?}", cpu.pc);
    assert_eq!(cpu.step(&task_db, &mut blackboard), Ok(Status::None));
    assert_eq!(
        &cpu.stack,
        &vec![StackItem::selector(2), StackItem::success()]
    );
    assert_eq!(&cpu.return_stack, &ReturnStack::new());
    //step 15
    pout!("ticking:{:?}", cpu.pc);
    assert_eq!(cpu.step(&task_db, &mut blackboard), Ok(Status::Success));
    assert_eq!(&cpu.stack, &vec![StackItem::success()]);
    assert_eq!(&cpu.return_stack, &ReturnStack::new());
    //step 16
    pout!("ticking:{:?}", cpu.pc);
    assert_eq!(
        cpu.step(&task_db, &mut blackboard),
        Err("program halted".into())
    );
    assert_eq!(&cpu.stack, &vec![StackItem::success()]);
    assert_eq!(&cpu.return_stack, &ReturnStack::new());
}

#[test]
fn test() {
    let mut blackboard = Blackboard::new();

    let mut task_db = TreePool::new();
    let action1 = "a1".to_owned();
    task_db.insert(
        action1.clone(),
        vec![Instruction::ForthAction(InpulseId::Act1)],
    );

    let action2 = "a2".to_owned();
    task_db.insert(
        action2.clone(),
        vec![Instruction::ForthAction(InpulseId::Act2)],
    );
    let action3 = "a3".to_owned();
    task_db.insert(
        action3.clone(),
        vec![Instruction::ForthAction(InpulseId::Act3)],
    );

    let sequence = "seq".to_owned();
    task_db.insert(
        sequence.clone(),
        vec![Instruction::Sequence(vec![
            action1.clone(),
            action2.clone(),
        ])],
    );

    let selector = "sel".to_owned();
    task_db.insert(
        selector.clone(),
        vec![Instruction::Selector(vec![
            sequence.clone(),
            action3.clone(),
        ])],
    );

    let mut cpu = CPU::load(selector.clone());

    for _ in 0..13 {
        //            println!("----\nStack:{stack:?}\nreturn_stack:{rs:?}");
        match cpu.step(&task_db, &mut blackboard) {
            Ok(ok) => {
                println!("{ok:?}");
            }
            Err(err) => {
                println!("Err:{err:?}");
                break;
            }
        };
    }
}
