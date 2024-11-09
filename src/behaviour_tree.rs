type NodeID = usize;

#[derive(Debug)]
pub enum Behaviour {
    Sequence,
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
    pub program_counter: usize,
    
}
impl BehaviourTree {
    pub fn fallback_tick(&mut self) {
    todo!()
    }
    pub fn sequence_tick(&mut self) {
        match self.stack.pop() {
            Some(Item::Success) => {
                self.running.push(self.program_counter.clone());
                self.program_counter += 1;
            },
            Some(Item::Falure) => {
                self.stack.push(Item::Falure);
                self.running.pop();
            },
            _ => {
                self.stack.push(Item::Falure);
                self.running.pop();
            }
        }
        
    }

    pub fn add_tick(&mut self) {
        self.running.pop();
        self.program_counter += 1;
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
        self.program_counter += 1;
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
            Some(Behaviour::Sequence) => {
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
                return false;
            }
        };
        true
    }
}

fn main(){
    let mut bt = BehaviourTree {
        nodes: vec![Behaviour::Sequence, Behaviour::Add, Behaviour::Add, Behaviour::Print],
        running: vec![0],
        stack: vec![Item::Int(3),Item::Int(2),Item::Int(1),Item::Success],
        program_counter: 1,
    };
    let mut c = 0;
    while !bt.running.is_empty() {
    //println!("Loop:{c}\n{bt:?}");
        bt.tick();
        c += 1;
    }
}