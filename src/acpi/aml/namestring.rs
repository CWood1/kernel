use collections::vec::Vec;
use collections::string::String;

use super::AmlError;

use super::dataobj::{parse_arg_obj, parse_local_obj};

pub fn parse_name_string(data: &[u8]) -> Result<(String, usize), AmlError> {
    let mut characters: Vec<u8> = vec!();
    let mut starting_index: usize = 0;

    let mut control_bytes: usize = 0;

    if data[0] == 0x5C {
        characters.push(data[0]);
        starting_index = 1;
    } else if data[0] == 0x5E {
        while data[starting_index] == 0x5E {
            characters.push(data[starting_index]);
            starting_index += 1;
        }
    }

    // TODO: Ew. Clean this shit up
    match parse_name_seg(&data[starting_index..]) {
        Ok(mut v) => characters.append(&mut v),
        Err(AmlError::AmlParseError) => 
            match parse_dual_name_path(&data[starting_index..]) {
                Ok(mut v) => {
                    characters.append(&mut v);
                    control_bytes = 1;
                },
                Err(AmlError::AmlParseError) => 
                    match parse_multi_name_path(&data[starting_index..]) {
                        Ok(mut v) => {
                            characters.append(&mut v);
                            control_bytes = 2;
                        },
                        Err(AmlError::AmlParseError) => 
                            match data[starting_index] {
                                0x00 => control_bytes = 1,
                                _ => return Err(AmlError::AmlParseError)
                            }
                    }
            }
    }

    let name_string = String::from_utf8(characters);

    match name_string {
        Ok(s) => Ok((s.clone(), s.clone().len() + control_bytes)),
        Err(_) => Err(AmlError::AmlParseError)
    }
}

pub fn parse_name_seg(data: &[u8]) -> Result<Vec<u8>, AmlError> {
    match data[0] {
        0x41 ... 0x5A | 0x5F => (),
        _ => return Err(AmlError::AmlParseError)
    }

    match data[1] {
        0x30 ... 0x39 | 0x41 ... 0x5A | 0x5F => (),
        _ => return Err(AmlError::AmlParseError)
    }

    match data[2] {
        0x30 ... 0x39 | 0x41 ... 0x5A | 0x5F => (),
        _ => return Err(AmlError::AmlParseError)
    }

    match data[3] {
        0x30 ... 0x39 | 0x41 ... 0x5A | 0x5F => (),
        _ => return Err(AmlError::AmlParseError)
    }

    Ok(vec!(data[0], data[1], data[2], data[3]))
}

fn parse_dual_name_path(data: &[u8]) -> Result<Vec<u8>, AmlError> {
    if data[0] != 0x2E {
        return Err(AmlError::AmlParseError);
    }

    let mut characters: Vec<u8> = vec!();

    match parse_name_seg(&data[1..5]) {
        Ok(mut v) => characters.append(&mut v),
        Err(AmlError::AmlParseError) => return Err(AmlError::AmlParseError)
    }

    match parse_name_seg(&data[5..9]) {
        Ok(mut v) => characters.append(&mut v),
        Err(AmlError::AmlParseError) => return Err(AmlError::AmlParseError)
    }

    Ok(characters)
}

fn parse_multi_name_path(data: &[u8]) -> Result<Vec<u8>, AmlError> {
    if data[0] != 0x2F {
        return Err(AmlError::AmlParseError);
    }

    let seg_count = data[1];
    if seg_count == 0x00 {
        return Err(AmlError::AmlParseError);
    }

    let mut current_seg = 0;
    let mut characters: Vec<u8> = vec!();
    
    while current_seg < seg_count {
        match parse_name_seg(&data[(current_seg as usize * 4) + 2 .. ((current_seg as usize + 1) * 4) + 2]) {
            Ok(mut v) => characters.append(&mut v),
            Err(AmlError::AmlParseError) => return Err(AmlError::AmlParseError)
        }

        current_seg += 1;
    }

    Ok(characters)
}

pub fn parse_super_name(data: &[u8]) -> Result<(u8, usize), AmlError> {
    match parse_simple_name(data) {
        Ok(res) => return Ok(res),
        Err(AmlError::AmlParseError) => ()
    }

    Err(AmlError::AmlParseError)
}

fn parse_simple_name(data: &[u8]) -> Result<(u8, usize), AmlError> {
    match parse_name_string(data) {
        Ok((name, name_len)) => {
            println!("{}", name);
            return Ok((2, name_len))
        },
        Err(AmlError::AmlParseError) => ()
    }

    match parse_arg_obj(data) {
        Ok(res) => return Ok(res),
        Err(AmlError::AmlParseError) => ()
    }

    parse_local_obj(data)
}
