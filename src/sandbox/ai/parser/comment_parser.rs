use nom::{
    bytes::complete::{tag, take_until}, error::ParseError, sequence::tuple, IResult
  };
  
  pub fn comment_parser<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, (), E> {
    let (tail, _) = tuple((
        tag("/*"),
        take_until("*/"),
        tag("*/")
      )
    )(i)?;
    Ok((tail, ()))
}