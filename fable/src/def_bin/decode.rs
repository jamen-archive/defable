use nom::IResult;

use crate::Error;

use super::{
    DefBin,
    DefBinHeader,
    DefBinNameLookup,
    DefSecondTableHeader,
    DefSecondTableRow,
    DefSecondTableRowDecompressed,
};

// impl DefBin {
//     fn decode_def_bin(input: &[u8]) -> IResult<&[u8], DefBin, Error> {
//     }
// }