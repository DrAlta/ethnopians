/// this is basd on the angry guest in RCT
///
/// anget with ene after level ticks but if that bumped then it goes back to LEVELs - number_of_times_they_have_benn_bumped_
///
/// with LEVELS = 16 then it'll start at 15 then if bumped with return to 14 then 13 and so on.
///
/// ToDO:
/// seperate number of levels and number of stages

pub fn main() {
    let mut a = Anger::new();
    println!("starting:{:?}", a.get());
    for _ in 0..10 {
        println!("{:?}", a.tick())
    }
    println!("after 20 ticks:{:?}", a.get());
    println!("stage:{:?}", a.stage());
    println!("bump:{:?}", a.bump());
    println!("stage:{:?}", a.stage());

    let mut idx = 0;
    loop {
        if a.tick() == None {
            println!("{idx}");
            return;
        };
        idx += 1;
    }
}

struct Anger(u16);
const LEVELS: u16 = 16;
type Int = u16;
impl Anger {
    pub fn new() -> Self {
        let stage = LEVELS - 1;
        Anger((stage * LEVELS) - 1)
    }
    pub fn get(&self) -> Option<Int> {
        if (self.0 % LEVELS) == 0 {
            return None;
        };
        Some(self.0 % LEVELS)
    }
    pub fn tick(&mut self) -> Option<Int> {
        if (self.0 % LEVELS) == 0 {
            return None;
        };
        self.0 -= 1;
        Some(self.0 % LEVELS)
    }
    pub fn stage(&self) -> Int {
        self.0 / LEVELS
    }
    pub fn bump(&mut self) -> Option<Int> {
        if (self.0 % LEVELS) == 0 {
            return None;
        };
        let stage = self.0 / LEVELS;
        println!("s:{stage}, a:{}", self.0 % LEVELS);
        self.0 = ((stage - 1) * LEVELS) + stage;
        Some(self.0 % LEVELS)
    }
}
