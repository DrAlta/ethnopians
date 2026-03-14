use crate::stand_alone_complex::latadog::Relation;

#[derive(Debug)]
pub struct Variable<const INDEX: usize, Key, Tuple> {
    /// A list of already processed tuples.
    pub stable: Vec<Relation<INDEX, Key, Tuple>>,
    /// A list of recently added but unprocessed tuples.
    pub recent: Relation<INDEX, Key, Tuple>,
    /// A list of tuples yet to be introduced.
    pub to_add: Vec<Relation<INDEX, Key, Tuple>>,
}
impl<'a, const INDEX: usize, Key, Tuple> Variable<INDEX, Key, Tuple> {
    pub fn insert(&mut self, relation: Relation<INDEX, Key, Tuple>){
        self.to_add.push(relation)
    }
}
