type NodeID = usize;

#[derive(Debug)]
pub enum Behaviour {
    Sequence{program_counter: NodeID},
    Fallback,
    Add,
    Print,
}

#[derive(Debug)]
pub enum Item {
    Success,
    Falure,
    Int(i64),
}

#[derive(Debug)]
pub struct BehaviourTree {
    pub nodes: Vec<Behaviour>,
    pub running: Vec<NodeID>,
    pub stack: Vec<Item>,
    
}
impl BehaviourTree {
    pub fn fallback_tick(&mut self) {
    todo!()
    }
    pub fn sequence_tick(&mut self) {
        let Some(self_id) = self.running.last() else {
            panic!();
        };
        let Some(Behaviour::Sequence{program_counter}) = self.nodes.get_mut(*self_id) else {
            panic!();
          //  return;
        };
        match self.stack.pop() {
            Some(Item::Success) => {
                *program_counter += 1;
                self.running.push(program_counter.clone());
            },
            Some(Item::Falure) => {
                *program_counter = 0;
                self.stack.push(Item::Falure);
                self.running.pop();
            },
            _ => {
                *program_counter = 0;
                self.stack.push(Item::Falure);
                self.running.pop();
            }
        }
        
    }

    pub fn add_tick(&mut self) {
        self.running.pop();
        let Some(Item::Int(a)) = self.stack.pop() else {
            self.stack.push(Item::Falure);
            return;
        };
        let Some(Item::Int(b)) = self.stack.pop() else {
            self.stack.push(Item::Falure);
            return;
        };
        let x = a + b;
        self.stack.push(Item::Int(x));
        self.stack.push(Item::Success);
    }
    pub fn print_tick(&mut self) {
        self.running.pop();
        let Some(a) = self.stack.pop() else {
            println!("Nothing on stack to print!");
            return;
        };
        println!("{:?}", a);
        
    }
    pub fn tick(&mut self) -> bool {
        let Some(last) = self.running.last() else {
            return false;
        };
        let x = self.nodes.get(*last);
        //println!("doing: {x:?}");
        match x {
            Some(Behaviour::Sequence{..}) => {
                self.sequence_tick()
            },
            Some(Behaviour::Fallback) => {
                self.fallback_tick()
            },
            Some(Behaviour::Add) => {
                self.add_tick()
            },
            Some(Behaviour::Print) => {
                self.print_tick()
            },
            _ => {
                panic!();
                //return false;
            }
        };
        true
    }
}

pub fn main(){
    let mut bt = BehaviourTree {
        nodes: vec![Behaviour::Sequence{program_counter: 0}, Behaviour::Add, Behaviour::Add, Behaviour::Print],
        running: vec![0],
        stack: vec![Item::Int(3),Item::Int(2),Item::Int(1),Item::Success],
    };
    let mut c = 0;
    //while !bt.running.is_empty() {
    while c < 7 {
        println!("Loop:{c}\n{bt:?}");
        if ! bt.tick() {
            return
        };
        c += 1;
    }
}