use std::io::Result;

use crate::utils::check_range;

pub(super) fn bin_to_float_point_two_five_complement_of_two(input: u8) -> f32 {
    if input >= 128 {
        (input as f32 - 256.0) * 0.25
    } else {
        (input as f32) * 0.25
    }
}

pub(super) fn float_point_two_five_to_bin_complement_of_two(input: f32) -> Result<u8> {
    check_range(-32.0, input, 31.75, 0.25)?;

    let input_x4 = input * 4.0;

    if input_x4 < 0.0 {
        Ok((input_x4 + 256.0) as u8)
    } else {
        Ok(input_x4 as u8)
    }
}

#[cfg(test)]
#[path = "./test_common.rs"]
mod test_common;
