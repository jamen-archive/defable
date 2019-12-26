use fable_base::nom::IResult;
use fable_base::nom::multi::{many0,many_till};

use crate::shared::decode::{decode_instr,decode_instr_tag};

use crate::tng::{
    TngThing,
    TngSection,
    Tng,
};

pub fn decode_tng_thing(input: &[u8]) -> IResult<&[u8], TngThing> {
    let (maybe_input, new_thing) = decode_instr_tag("NewThing")(input)?;
    let (maybe_input, (instrs, _end)) = many_till(decode_instr, decode_instr_tag("EndThing"))(maybe_input)?;

    Ok(
        (
            maybe_input,
            TngThing {
                new_thing: new_thing,
                instrs: instrs,
            }
        )
    )
}

pub fn decode_tng_section(input: &[u8]) -> IResult<&[u8], TngSection> {
    let (maybe_input, section_start) = decode_instr_tag("XXXSectionStart")(input)?;
    let (maybe_input, (things, _end)) = many_till(decode_tng_thing, decode_instr_tag("XXXSectionEnd"))(maybe_input)?;

    Ok(
        (
            maybe_input,
            TngSection {
                section_start: section_start,
                things: things,
            }
        )
    )
}

pub fn decode_tng(input: &[u8]) -> IResult<&[u8], Tng> {
    let (maybe_input, version) = decode_instr_tag("Version")(input)?;
    let (maybe_input, sections) = many0(decode_tng_section)(maybe_input)?;

    Ok(
        (
            maybe_input,
            Tng {
                version: version,
                sections: sections,
            }
        )
    )
}