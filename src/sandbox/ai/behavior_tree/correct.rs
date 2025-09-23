use crate::sandbox::ai::TaskPool;

pub trait Corrent {
    fn correct(&mut self, prefix: &str);
}
impl Corrent for TaskPool {
    fn correct(&mut self, prefix: &str) {
        let original = std::mem::replace(self, TaskPool::new());
        for (k, mut v) in original.into_iter() {
            v.iter_mut().for_each(|x| x.correct(prefix));
            assert_eq!(self.insert(format!("{prefix}{k}"), v), None,);
        }
    }
}
