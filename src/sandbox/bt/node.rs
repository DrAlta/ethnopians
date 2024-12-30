enum Node {
    Sequence(Vec<ReturnPointer>),
    Selector(Vec<ReturnPointer>),
    Action(fn(&mut World) -> Status),
}
impl Node {
    fn tick_selector(
        children: &Vec<ReturnPointer>, 
        stack: &mut Vec::<NodeState>, 
        return_stack: &mut Vec::<ReturnPointer>, 
        bt: &HashMap::<ReturnPointer, Node>, 
        world: &mut World,
    ) -> Status {
        let Some(NodeState::Selector(idx)) = stack.pop() else {
            return Status::Failure
        };
        
        for idx2 in idx..children.len() {
//            let pre: &str = if let Some(thing) = return_stack.last() { thing } else {&""};
            let Some(child) = bt.get(children.get(idx2).unwrap()) else {
                return Status::Failure
            };

            return_stack.push(idx2);//format!("{pre}:{idx}"));
            child.init(stack);
            match child.tick(stack, return_stack, bt, world) {
                Status::Success => {
                    return_stack.pop();
                    return Status::Success
                },
                Status::Failure => {
                    return_stack.pop();
                    continue
                },
                Status::Running(x) => {
                    //return_stack.pop();
                    stack.push(NodeState::Selector(idx2));
                    return Status::Running(x)
                },
            }
        }
        Status::Failure
    }
    fn tick_sequence(
        children: &Vec<ReturnPointer>, 
        stack: &mut Vec::<NodeState>, 
        return_stack: &mut Vec::<ReturnPointer>, 
        pc: &mut ReturnPointer,
        bt: HashMap<ReturnPointer, Node>,
        world: &mut World) -> Status {
        let Some(NodeState::Sequence(idx)) = stack.pop() else {
            return Status::Failure
        };
        
        let Some(child) = children.get(idx) else {
            return Status::Failure
        };
        return_stack.push(pc);
        child.init(stack, return_stack);
        //set the program counter so the child is executed next
        *rp = child;

            match child.tick(stack, return_stack, bt, world) {
                Status::Success => {
                    return_stack.pop();
                    continue
                },
                Status::Failure => {
                    return_stack.pop();
                    return Status::Failure
                },
                Status::Running(x) => {
                    //return_stack.pop();
                    stack.push(NodeState::Sequence(idx2));
                    return Status::Running(x)
                },
            }
        }
        Status::Success
    }
    pub fn resume(
        &self, 
        depth: usize, 
        stack: &mut Vec::<NodeState>,  
        return_stack: &mut Vec::<ReturnPointer>, 
        bt: &HashMap::<ReturnPointer, Node>,
        world: &mut World,
    ) -> Option<Status> {
        println!("resume depth:{depth} rs:{return_stack:?}");
        let child_idx = depth;
        let child;
        match self {
            Node::Selector(children) => {
                println!(
                    "Geting Selector's {}th child among {}",
                    if let Some(x) = return_stack.get(child_idx) {x} else{println!("depth to far");return None},
                    return_stack.len()
                );
                child = bt.get(children.get(return_stack[child_idx]).unwrap())?;
                println!("do you see this?");
            },
            Node::Sequence(children) => {
                println!(
                    "Geting Sequence's {}th child among {}",
                    return_stack.get(child_idx)?,
                    return_stack.len()
                );
                child = bt.get(
                    children.get(
                        return_stack[child_idx]
                    ).unwrap()
                )?;
            },
            Node::Action(_func) => {
            println!("got action");
            return None},
        }
        println!("rs exit:{return_stack:?}");
        if depth < return_stack.len() - 1 {
            child.resume(depth + 1, stack, return_stack, bt, world)
        } else {
            Some(child.tick(stack, return_stack, bt, world))
        }
    }

    pub fn tick(
        &self, 
        stack: &mut Vec::<NodeState>, 
        return_stack: &mut Vec::<ReturnPointer>, 
        bt: &HashMap::<ReturnPointer, Node>,
        world: &mut World
    ) -> Status {
        println!("tick rs:{return_stack:?}");
        match self {
            Node::Selector(children) => {
                Self::tick_selector(children, stack, return_stack, bt, world)
            },
            Node::Sequence(children) => {
                Self::tick_sequence(children, stack, return_stack, bt, world)
            },
            Node::Action(func) => func(world),
        }
    }
    pub fn init(&self, stack: &mut Vec::<NodeState>,) {
        match self {
            Node::Selector(_children) => {
                stack.push(NodeState::Selector(0))
            },
            Node::Sequence(_children) => {
                stack.push(NodeState::Sequence(0))
            },
            _ => (),
        }
    }
    pub fn run(
        &self, 
        bt: &HashMap::<ReturnPointer, Node>,
        world: &mut (bool, bool),
    ) -> (
        Status,
        Vec::<NodeState>, 
        Vec::<ReturnPointer>,
    ){
        let mut stack = Vec::new();
        let mut rs = Vec::new();

        self.init(&mut stack);
        let x = self.tick(&mut stack, &mut rs, bt, world);

        (x, stack, rs)

    }
}

#[test]
fn test() {
    let mut bt = HashMap::<ReturnPointer, Node>::new();
    let action1 = 0;
    bt.insert(
        action1,
        Node::Action(|world| {
            println!("Action 1");
            if world.0 {
                world.0 = false;
                Status::Running(69)
            } else {
                Status::Success
            }
        })
    );

    let action2 = 1 ;
    bt.insert(
        action2, 
        Node::Action(|_| {
            println!("Action 2");
            Status::Failure
        })
    );
    let action3 =3;
    bt.insert(
        action3,
        Node::Action(|world| {
            println!("Action 3");
            if world.1 {
                world.1 = false;
                Status::Running(42)
            } else {
                return Status::Success
            }
        })
    );

    let sequence = Node::Sequence(vec![action1, action2]);
    
    let selector = Node::Selector(vec![sequence, action3]);
    
    let mut world = (true, true);
    let mut depth = 0;
    let (a, mut stack, mut rs) = selector.run(& bt, &mut world);
    
    match a {
        Status::Success => println!("Sequence succeeded"),
        Status::Failure => println!("Sequence failed"),
        Status::Running(x) => {
            println!("Sequence running {x}");
        },
    };
    for _ in 0..3{
        println!("----\n{rs:?}\n----{depth}");
        match selector.resume(depth, &mut stack, &mut rs, & bt, &mut world) {
            Some(Status::Success) => {
                depth = 0;
                println!("Sequence succeeded")
            },
            Some(Status::Failure) => {
                depth = 0;
                println!("Sequence failed")
            },
            Some(Status::Running(x)) => {
                depth = rs.len() - 1;
                println!("Sequence running {x}:{rs:?}");
            },
            _ => {
                println!("oops");
                depth = 0;
            },
        };
        
    }
    panic!("success");
}

