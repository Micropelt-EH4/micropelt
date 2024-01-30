use std::io::{Error, ErrorKind, Result};

pub fn check_range(lower: f32, input: f32, upper: f32, resolution: f32) -> Result<()> {
    if input < lower - 0.5 * resolution {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            format!("Lower limit is {lower}, received {input}"),
        ));
    }
    if input > upper + 0.5 * resolution {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            format!("Upper limit is {upper}, received {input}"),
        ));
    }

    Ok(())
}

pub fn bin_to_ten(input: u8) -> u16 {
    input as u16 * 10
}

pub fn bin_to_float_point_five(input: u8) -> f32 {
    input as f32 * 0.5
}

pub fn bin_to_float_point_two_five(input: u8) -> f32 {
    input as f32 * 0.25
}

pub fn bin_to_float_point_zero_two(input: u8) -> f32 {
    input as f32 * 0.02
}

pub fn bin_to_bool(input: u8) -> Result<bool> {
    match input {
        0 => Ok(false),
        1 => Ok(true),
        _ => Err(Error::new(
            ErrorKind::InvalidData,
            format!("A bit should only be 0 or 1, got {input}"),
        )),
    }
}

pub fn bool_to_bin(input: bool) -> u8 {
    match input {
        true => 1,
        false => 0,
    }
}

pub fn percent_to_bin(input: u8) -> Result<u8> {
    check_range(0.0, input as f32, 100.0, 1.0)?;

    Ok(input)
}

pub fn float_point_five_to_bin(input: f32, upper: f32) -> Result<u8> {
    check_range(0.0, input, upper, 0.5)?;

    Ok((input * 2.0) as u8)
}