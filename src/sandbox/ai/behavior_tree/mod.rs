//! when a child returns running it shouldn't move on to the next child but retry the same child to see if it will return running or if it now succeess/failure and then move on

/*
/// ActiveSeq keeps track of the last child that returned success, each time a child returns success it sets that as it's highest succes then restarts at it's first child. of a child if after the highest success fails the actice selector fails, other wares it keep evaluating it's chilren like a selector
stuct ActiveSeq{
    children: Vec<node>,
    currect: usize,
    last_success: usize,
}
impl ActiveSeq {
    fn tick(&mut self) -> Status{
    let x = self.children[self.current].tick();
    match (x, self.currect <= self.last_success) {
        (Success, _) => {
            self.last_success = self.current;
            self.current + 1;
            if self.current == self.children.len() {
                return Success
            } else {
                return Running
            }
        },
        (Failure, false) => {
            self.current = 0;
            self.last_succes = 0;
            return Failure
        },
        (Failure, true) => {
            self.current += 1;
            return Running
        },
    }
}
*/

mod correct;
pub use correct::Corrent;
