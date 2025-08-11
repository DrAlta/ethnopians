use std::collections::HashMap;
#[derive(Debug)]
pub struct Relation<const N: usize> {
    pub titles: [String;N],
    pub tuples: Vec<[i8;N]>,
}

pub fn join(s:&HashMap<String, Relation<2>>, terms:Vec<(String,String)>) -> Vec<&[i8;2]>{
    let mut iter = terms.into_iter();
    let Some((relation_id, field_id)) = iter.next() else {
        return Vec::new();
    };
    let Some(first_relation) = s.get(&relation_id) else {
        return Vec::new();
    };
    let Some(field_idx) = first_relation.titles.iter().position(|x|x==&field_id) else{
        return Vec::new();
    };
    let mut temp: HashMap<usize, i8> = first_relation.tuples.iter().enumerate().map(|(idx, tuple)| (idx, tuple[field_idx])).collect();
    println!("{temp:?}");
    for (relation_id, field_id) in iter {
        let Some(relation) = s.get(&relation_id) else {
            return Vec::new();
        };
        let Some(field_idx) = relation.titles.iter().position(|x|x==&field_id) else{
            return Vec::new();
        };
        let test: Vec<&i8> = relation.tuples.iter().map(|t| &t[field_idx]).collect();
        temp.retain(|_,v| test.contains(&&*v));
    }
    let x: Vec<usize>= temp.into_iter().map(|(k,_)| k).collect();
    let ret = x.into_iter().map(|i| &first_relation.tuples[i]).collect();
    ret

    
} 

