use super::*;
use crate::sandbox::bt::{InpulseId, Instruction};

#[test]
fn step_test() {
    let mut bt = HashMap::<ExecutionToken, Thread>::new();
    let action1 = "act1".to_owned();
    bt.insert(action1.clone(), Instruction::Action(InpulseId::Act1));

    let action2 = "act2".to_owned();
    bt.insert(action2.clone(), Instruction::Action(InpulseId::Act2));
    let action3 = "act3".to_owned();
    bt.insert(action3.clone(), Instruction::Action(InpulseId::Act3));

    let sequence = "seq".to_owned();
    bt.insert(
        sequence.clone(),
        Instruction::Sequence(vec![action1.clone(), action2.clone()]),
    );

    let selector = "sel".to_owned();
    bt.insert(
        selector.clone(),
        Instruction::Selector(vec![sequence.clone(), action3]),
    );

    let mut cpu = CPU::load(selector.clone());
    //step 1 selectpr does its init and sets the cpu up to call its first child, sequence.
    assert_eq!(cpu.step(&bt), Ok(Status::None));
    assert_eq!(&cpu.stack, &vec![StackItem::Selector(1), StackItem::Init]);
    assert_eq!(&cpu.return_stack, &vec![selector.clone()]);
    //step 2 sequence intalized and set the cpu up to call its first child, action1
    assert_eq!(cpu.step(&bt), Ok(Status::None));
    assert_eq!(
        &cpu.stack,
        &vec![
            StackItem::Selector(1),
            StackItem::Sequence(1),
            StackItem::Init
        ]
    );
    assert_eq!(&cpu.return_stack, &vec![selector.clone(), sequence.clone()]);
    //step 3 action1 puts it's state on the stack,Success, and prays Running(1)
    assert_eq!(
        cpu.step(&bt),
        Ok(Status::Running(InpulseId::Act1))
    );
    assert_eq!(
        &cpu.stack,
        &vec![
            StackItem::Selector(1),
            StackItem::Sequence(1),
            StackItem::Success
        ]
    );
    assert_eq!(&cpu.return_stack, &vec![selector.clone(), sequence.clone()]);
    assert_eq!(&cpu.pc, &Some(action1));
    //step 4 action1 sees it's status on the stack and returns it seting the cpu to return to the calling thread
    assert_eq!(cpu.step(&bt), Ok(Status::None));
    assert_eq!(
        &cpu.stack,
        &vec![
            StackItem::Selector(1),
            StackItem::Sequence(1),
            StackItem::Success
        ]
    );
    assert_eq!(&cpu.return_stack, &vec![selector.clone()]);
    //step 5  sequence sets the cpu up to run it's second child, action2
    assert_eq!(cpu.step(&bt), Ok(Status::None));
    assert_eq!(
        &cpu.stack,
        &vec![
            StackItem::Selector(1),
            StackItem::Sequence(2),
            StackItem::Init
        ]
    );
    assert_eq!(&cpu.pc, &Some(action2));
    assert_eq!(&cpu.return_stack, &vec![selector.clone(), sequence.clone()]);
    //step 6 action2 intalizes and prays Running(2)
    assert_eq!(
        cpu.step(&bt),
        Ok(Status::Running(InpulseId::Act2))
    );
    assert_eq!(
        &cpu.stack,
        &vec![
            StackItem::Selector(1),
            StackItem::Sequence(2),
            StackItem::Failure
        ]
    );
    assert_eq!(&cpu.return_stack, &vec![selector.clone(), sequence]);
    //step 7 action2 returns to sequence
    assert_eq!(cpu.step(&bt), Ok(Status::None));
    assert_eq!(
        &cpu.stack,
        &vec![
            StackItem::Selector(1),
            StackItem::Sequence(2),
            StackItem::Failure
        ]
    );
    assert_eq!(&cpu.return_stack, &vec![selector.clone()]);
    //step 8 sequence sees it's last child has returned failure to returns failure to select
    assert_eq!(cpu.step(&bt), Ok(Status::None));
    assert_eq!(&cpu.stack, &vec![StackItem::Selector(1), StackItem::Failure]);
    assert_eq!(&cpu.return_stack, &Vec::<ExecutionToken>::new());
    //step 9 selector sees it's first child has returned failure so puts it's new state and then init on the stack and set up the cpu to run it's second child, action3
    assert_eq!(cpu.step(&bt), Ok(Status::None));
    assert_eq!(&cpu.stack, &vec![StackItem::Selector(2), StackItem::Init]);
    assert_eq!(&cpu.return_stack, &vec![selector.clone()]);
    //step 10 action3 sees init and puts Success on tha stack and prays Running(3)
    assert_eq!(
        cpu.step(&bt),
        Ok(Status::Running(InpulseId::Act3))
    );
    assert_eq!(&cpu.stack, &vec![StackItem::Selector(2), StackItem::Success]);
    assert_eq!(&cpu.return_stack, &vec![selector]);
    //step 11 action3 sees the success on the stack and returns it to the calling function
    assert_eq!(cpu.step(&bt), Ok(Status::None));
    assert_eq!(&cpu.stack, &vec![StackItem::Selector(2), StackItem::Success]);
    assert_eq!(&cpu.return_stack, &Vec::<ExecutionToken>::new());
    //step 12 selection sees its it's child return success then seens its' has no calling function so holts execution and prays Success
    assert_eq!(cpu.step(&bt), Ok(Status::Success));
    assert_eq!(&cpu.stack, &vec![StackItem::Success]);
    assert_eq!(&cpu.return_stack, &Vec::<ExecutionToken>::new());
    //step 13 the program has is holted
    assert_eq!(
        cpu.step(&bt),
        Err("program halted".into())
    );
    assert_eq!(&cpu.stack, &vec![StackItem::Success]);
    assert_eq!(&cpu.return_stack, &Vec::<ExecutionToken>::new());
}

#[test]
fn test() {
    let mut bt = HashMap::<ExecutionToken, Thread>::new();
    let action1 = "a1".to_owned();
    bt.insert(
        action1.clone(),
        Thread::Action(InpulseId::Act1)
    );

    let action2 = "a2".to_owned() ;
    bt.insert(
        action2.clone(),
        Thread::Action(InpulseId::Act2)
    );
    let action3 ="a3".to_owned();
    bt.insert(
        action3.clone(),
        Thread::Action(InpulseId::Act3)
    );

    let sequence = "seq".to_owned();
    bt.insert(sequence.clone(), Thread::Sequence(vec![action1.clone(), action2.clone()]));

    let selector = "sel".to_owned();
    bt.insert(selector.clone(), Thread::Selector(vec![sequence.clone(), action3.clone()]));

    let mut cpu = CPU::load(selector.clone());

    for _ in 0..13{
//            println!("----\nStack:{stack:?}\nreturn_stack:{rs:?}");
        match cpu.step(&bt) {
            Ok(ok) => {
                println!("{ok:?}");
            },
            Err(err) => {
                println!("Err:{err:?}");
                break;
            }
        };
    }
}