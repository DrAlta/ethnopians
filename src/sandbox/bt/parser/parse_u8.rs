use nom::{
    character::complete::{char, one_of}, combinator::{map_res, recognize}, error::ErrorKind, multi::{many0, many1}, sequence::terminated, IResult
  };
  
pub fn parse_u8(input: &str) -> IResult<&str, u8, (&str, ErrorKind)> {
    map_res(
        recognize(
            many1(
                terminated(one_of("0123456789"), many0(char('_')))
            )
        ),
        |out| {
            u8::from_str_radix(
                &str::replace(
                    out, 
                    "_", 
                    ""
                ), 
                10
            )
        }
        )(input)
  }