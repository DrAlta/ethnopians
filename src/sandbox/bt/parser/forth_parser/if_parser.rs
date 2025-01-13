use nom::{
    bytes::complete::tag, character::streaming::multispace0, combinator::{map_res, recognize}, error::ErrorKind, sequence::tuple, IResult
};

use crate::sandbox::bt::{
    parser::TreesUsed,
    Instruction, Thread, TreePool,
};

use super::{balanced::{balanced, Tract}, forth_threadette_parser};


trait If{
    fn flatten(self) -> (Thread, TreePool);
}
impl If for Vec<Tract<(Thread, TreePool)>> {
    fn flatten(self) -> (Thread, TreePool) {
        let mut thread = vec![Instruction::ForthIf(0)];
        let mut used = TreesUsed::new();
        let mut count = 0;
        for x in self{
            match x {
                Tract::Item(mut item) => {
                    count += item.0.len();
                    thread.append(&mut item.0);
                    used.extend(item.1.into_iter());
                },
                Tract::Level(vec) => {
                    let mut item = vec.flatten();
                    count += item.0.len();
                    thread.append(&mut item.0);
                    used.extend(item.1.into_iter());
                },
            }
        };
        let Some(Instruction::ForthIf(x)) = thread.first_mut() else {
            panic!()
        };
        *x = count;

        (thread, used)
    }
}


pub fn if_parser<'a>(input: &'a str) -> IResult<&'a str, (Thread, TreePool), (&'a str, ErrorKind)> {
    let (tail, body) = balanced(
        map_res(tuple((
                multispace0,
                forth_threadette_parser,
            )),
            |(_,x)| Result::<(Thread, TreePool), (&'a str, ErrorKind)>::Ok(x)
        ),
        recognize(tuple((
            multispace0,
            tag("if"),
        ))),
        recognize(tuple((
            multispace0,
            tag("then")
        )))
    )(input)?;
    //let mut vec = Vec::new();
    let x = body.flatten();
    Ok((tail, x))
    /*
    vec.push(Instruction::ForthIf(thread.len()));
    vec.extend(thread.into_iter());
    Ok((tail, (vec, used)))*/
}

#[cfg(test)]
mod tests {
    use crate::sandbox::bt::StackItem;

    use super::*;

    #[test]
    fn if_parser_test() {
        let input = "if
            lit(Success)
            return
        then";
        let (tail, (body, used)) = if_parser(input).unwrap();
        assert_eq!(tail, "");
        assert_eq!(used, TreePool::new());
        assert_eq!(
            body,
            vec![
                Instruction::ForthIf(2),
                Instruction::ForthLit(StackItem::Success),
                Instruction::ForthReturn,
            ]
        )
    }
    #[test]
    fn nested_if_parser_test() {
        let input = "if
            lit(1)
            if
                return
            then
            lit(2)
        then";
        let (tail, (body, used)) = if_parser(input).unwrap();
        assert_eq!(tail, "");
        assert_eq!(used, TreePool::new());
        assert_eq!(
            body,
            vec![
                Instruction::ForthIf(4),
                Instruction::ForthLit(StackItem::Int(1)),
                Instruction::ForthIf(1),
                Instruction::ForthReturn,
                Instruction::ForthLit(StackItem::Int(2)),
            ]
        )
    }
}
