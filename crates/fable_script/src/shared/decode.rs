use fable_base::nom::IResult;
use fable_base::nom::Err;
use fable_base::nom::error::ErrorKind;
use fable_base::nom::character::complete::{digit1,line_ending,one_of,space0};
use fable_base::nom::character::{is_digit,is_alphabetic};
use fable_base::nom::combinator::opt;
use fable_base::nom::bytes::complete::{tag,take_while1,escaped,is_not};
use fable_base::nom::branch::alt;
use fable_base::nom::multi::{many_till,many0,many1};
use fable_base::nom::sequence::{terminated,preceded};

use crate::shared::{
    Instr,
    InstrKey,
    InstrValue,
};

pub fn decode_instr_key(input: &[u8]) -> IResult<&[u8], InstrKey> {
    alt((
        decode_instr_key_property,
        decode_instr_key_index,
        decode_instr_key_name
    ))(input)
}

pub fn decode_instr_key_property(input: &[u8]) -> IResult<&[u8], InstrKey> {
    let (maybe_input, key_name) = decode_instr_key_name(input)?;
    let (maybe_input, mut parts) = many1(decode_instr_key_property_access)(maybe_input)?;

    parts.insert(0, key_name);

    Ok((maybe_input, InstrKey::Property(parts)))
}

pub fn decode_instr_key_property_access(input: &[u8]) -> IResult<&[u8], InstrKey> {
    let (maybe_input, accessor) = one_of(".[")(input)?;

    if accessor == '[' {
        terminated(decode_instr_key, tag("]"))(maybe_input)
    } else if accessor == '.' {
        decode_instr_key_name(maybe_input)
    } else {
        Err(Err::Error((input, ErrorKind::ParseTo)))
    }
}

pub fn decode_instr_key_index(input: &[u8]) -> IResult<&[u8], InstrKey> {
    let (maybe_input, index) = decode_instr_value_number(input)?;

    let index = match index {
        InstrValue::Number(index) => index,
        _ => return Err(Err::Error((input, ErrorKind::ParseTo))),
    };

    Ok((maybe_input, InstrKey::Index(index as u32)))
}

pub fn decode_instr_key_name(input: &[u8]) -> IResult<&[u8], InstrKey> {
    let (maybe_input, key) = take_while1(|x| is_alphabetic(x) || is_digit(x) || x == 0x5f)(input)?;

    let key = match String::from_utf8(key.to_vec()) {
        Ok(key) => key,
        Err(_error) => return Err(Err::Error((input, ErrorKind::ParseTo))),
    };

    Ok((maybe_input, InstrKey::Name(key)))
}

pub fn decode_instr_value(input: &[u8]) -> IResult<&[u8], InstrValue> {
    alt((
        decode_instr_value_none,
        decode_instr_value_bool,
        decode_instr_value_float,
        decode_instr_value_number,
        decode_instr_value_big_number,
        decode_instr_value_string,
        decode_instr_value_call,
        decode_instr_value_name,
    ))(input)
}

pub fn decode_instr_value_bool(input: &[u8]) -> IResult<&[u8], InstrValue> {
    let (maybe_input, value) = alt((tag("TRUE"), tag("FALSE")))(input)?;
    let value = match value {
        b"TRUE" => true,
        b"FALSE" => false,
        _ => return Err(Err::Error((input, ErrorKind::ParseTo)))
    };
    Ok((maybe_input, InstrValue::Bool(value)))
}

pub fn decode_instr_value_none(input: &[u8]) -> IResult<&[u8], InstrValue> {
    match alt((tag(";"), line_ending))(input) {
        Ok((_input, _tag)) => Ok((input, InstrValue::None)),
        Err(error) => Err(error)
    }
}

pub fn decode_instr_value_float(input: &[u8]) -> IResult<&[u8], InstrValue> {
    let (input, negative) = opt(tag("-"))(input)?;
    let (maybe_input, value) = take_while1(|x| is_digit(x) || x == 0x2e)(input)?;

    let value = match String::from_utf8(value.to_vec()) {
        Ok(value) => value,
        Err(_error) => return Err(Err::Error((input, ErrorKind::ParseTo))),
    };

    let value = match value.parse::<f32>() {
        Ok(value) => value,
        Err(_error) => return Err(Err::Error((input, ErrorKind::Digit))),
    };

    let value = if negative.is_none() { value } else { -value };

    Ok((maybe_input, InstrValue::Float(value)))
}

pub fn decode_instr_value_number(input: &[u8]) -> IResult<&[u8], InstrValue> {
    let (maybe_input, negative) = opt(tag("-"))(input)?;
    let (maybe_input, value) = digit1(maybe_input)?;

    let value = match String::from_utf8(value.to_vec()) {
        Ok(value) => value,
        Err(_error) => return Err(Err::Error((input, ErrorKind::ParseTo))),
    };

    let value = match value.parse::<i32>() {
        Ok(value) => value,
        Err(_error) => return Err(Err::Error((input, ErrorKind::Digit))),
    };

    let value = if negative.is_none() { value } else { -value };

   Ok((maybe_input, InstrValue::Number(value)))
}

pub fn decode_instr_value_big_number(input: &[u8]) -> IResult<&[u8], InstrValue> {
    let (maybe_input, value) = digit1(input)?;

    let value = match String::from_utf8(value.to_vec()) {
        Ok(value) => value,
        Err(_error) => return Err(Err::Error((input, ErrorKind::ParseTo))),
    };

    let value = match value.parse::<u64>() {
        Ok(value) => value,
        Err(_error) => return Err(Err::Error((input, ErrorKind::Digit))),
    };

   Ok((maybe_input, InstrValue::BigNumber(value)))
}

pub fn decode_instr_value_string(input: &[u8]) -> IResult<&[u8], InstrValue> {
    let (maybe_input, _opener) = tag("\"")(input)?;
    let (maybe_input, value) = opt(escaped(is_not("\""), '\\', one_of("\"\\")))(maybe_input)?;
    let (maybe_input, _closer) = tag("\"")(maybe_input)?;

    let value = match value {
        Some(value) =>
            match String::from_utf8(value.to_vec()) {
                Ok(value) => value,
                Err(_error) => return Err(Err::Error((input, ErrorKind::ParseTo))),
            },
        None => "".to_string(),
    };

    Ok((maybe_input, InstrValue::String(value)))
}

// TODO: Add leniency on space between parameters.

pub fn decode_instr_value_call(input: &[u8]) -> IResult<&[u8], InstrValue> {
    let (maybe_input, name) = decode_instr_value_name(input)?;

    let name = match name {
        InstrValue::Name(value) => value,
        _ => return Err(Err::Error((input, ErrorKind::ParseTo))),
    };

    let (maybe_input, _start) = tag("(")(maybe_input)?;
    let (maybe_input, (mut values, last)) = many_till(
        preceded(space0, terminated(terminated(decode_instr_value, space0), tag(","))),
        preceded(space0, terminated(terminated(decode_instr_value, space0), tag(")")))
    )(maybe_input)?;

    values.push(last);

    Ok((maybe_input, InstrValue::Call((name, values))))
}

// pub fn decode_instr_value_call_tag(name: &'static str) -> impl Fn(&[u8]) -> IResult<&[u8], InstrValue> {
//     move |input: &[u8]| {
//         let (maybe_input, func) = decode_instr_value_call(input)?;

//         let (key, values) = match func {
//             InstrValue::Call((key, values)) => (key, values),
//             _ => return Err(Err::Error((input, ErrorKind::ParseTo))),
//         };

//         if key != name.clone() {
//             return Err(Err::Error((input, ErrorKind::ParseTo)));
//         }

//         Ok((maybe_input, InstrValue::Call((key, values))))
//     }
// }

pub fn decode_instr_value_name(input: &[u8]) -> IResult<&[u8], InstrValue> {
    let (maybe_input, name) = take_while1(|x| (is_alphabetic(x) || is_digit(x) || x == 0x5f || x == 0x20))(input)?;

    let name = match String::from_utf8(name.to_vec()) {
        Ok(value) => value,
        Err(_error) => return Err(Err::Error((input, ErrorKind::ParseTo))),
    };

    Ok((maybe_input, InstrValue::Name(name)))
}

pub fn decode_instr(input: &[u8]) -> IResult<&[u8], Instr> {
    let (maybe_input, _line_ending) = many0(line_ending)(input)?;
    let (maybe_input, key) = decode_instr_key(maybe_input)?;
    let (maybe_input, _space) = opt(tag(" "))(maybe_input)?;
    let (maybe_input, value) = decode_instr_value(maybe_input)?;
    let (maybe_input, _semicolon) = tag(";")(maybe_input)?;
    let (maybe_input, _line_ending) = many1(line_ending)(maybe_input)?;

    Ok((maybe_input, (key, value)))
}

pub fn decode_instr_tag(name: &'static str) -> impl Fn(&[u8]) -> IResult<&[u8], Instr> {
    move |input: &[u8]| {
        let (maybe_input, (key, value)) = decode_instr(input)?;

        let key = match key {
            InstrKey::Name(x) => x,
            InstrKey::Index(_) => return Err(Err::Error((input, ErrorKind::ParseTo))),
            InstrKey::Property(_) => return Err(Err::Error((input, ErrorKind::ParseTo))),
        };

        // println!("{:?} == {:?}", name, key);

        if key != name {
            return Err(Err::Error((input, ErrorKind::ParseTo)));
        }

        Ok((maybe_input, (InstrKey::Name(key), value)))
    }
}