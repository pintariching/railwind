use nom::branch::alt;
use nom::bytes::complete::{tag, take_while_m_n};
use nom::combinator::{map, map_res};
use nom::sequence::{preceded, tuple};
use nom::IResult;
use std::num::ParseIntError;

fn from_hex(input: &str) -> Result<u8, ParseIntError> {
    u8::from_str_radix(input, 16)
}

fn is_hex_digit(c: char) -> bool {
    c.is_digit(16)
}

fn hex_primary(input: &str) -> IResult<&str, u8> {
    map_res(take_while_m_n(2, 2, is_hex_digit), from_hex)(input)
}

fn hex_single(input: &str) -> IResult<&str, u8> {
    map_res(take_while_m_n(1, 1, is_hex_digit), |i| {
        from_hex(&format!("{}{}", i, i))
    })(input)
}

pub fn hex_color(input: &str) -> IResult<&str, String> {
    preceded(
        tag("#"),
        alt((
            map(
                tuple((hex_primary, hex_primary, hex_primary)),
                |(red, green, blue)| format!("{red} {green} {blue}"),
            ),
            map(
                tuple((hex_single, hex_single, hex_single)),
                |(red, green, blue)| format!("{red} {green} {blue}"),
            ),
        )),
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_color_normal() {
        assert_eq!(hex_color("#26ad4a"), Ok(("", "38 173 74".to_string())));
    }

    #[test]
    fn test_hex_color_single() {
        assert_eq!(hex_color("#234"), Ok(("", "34 51 68".to_string())))
    }
}
