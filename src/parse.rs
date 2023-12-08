use nom::{bytes::complete::take_while1, combinator::fail, error::context, IResult};
use num_traits::PrimInt;

pub fn take_number<N: PrimInt>(input: &str) -> IResult<&str, N> {
    let (input, number_str) = take_while1(|c: char| c.is_ascii_digit())(input)?;

    let Ok(number) = N::from_str_radix(number_str, 10) else {
        return context("could not parse the number", fail)(input);
    };

    Ok((input, number))
}
