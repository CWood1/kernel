use collections::vec::Vec;

use super::AmlError;

use super::pkglength::parse_pkg_length;
use super::termlist::{parse_term_arg, parse_term_list, TermObj, TermArg};

pub enum Type1OpCode {
    DefIfElse {
        predicate: TermArg,
        if_block: Vec<TermObj>,
        else_block: Vec<TermObj>
    },
    DefWhile {
        predicate: TermArg,
        block: Vec<TermObj>
    }
}

pub fn parse_type1_opcode(data: &[u8]) -> Result<(Type1OpCode, usize), AmlError> {
    match parse_def_if_else(data) {
        Ok(res) => return Ok(res),
        Err(AmlError::AmlParseError) => ()
    }
    
    parse_def_while(data)
}

fn parse_def_if_else(data: &[u8]) -> Result<(Type1OpCode, usize), AmlError> {
    if data[0] != 0xA0 {
        return Err(AmlError::AmlParseError);
    }

    let (pkg_length, pkg_length_len) = parse_pkg_length(&data[1..])?;
    let (predicate, predicate_len) = parse_term_arg(&data[1 + pkg_length_len..])?;
    let if_block = parse_term_list(&data[1 + pkg_length_len + predicate_len .. 1 + pkg_length])?;
    let (else_block, else_block_len) = parse_def_else(&data[1 + pkg_length..])?;

    Ok((Type1OpCode::DefIfElse {predicate, if_block, else_block}, pkg_length + else_block_len + 1))
}

fn parse_def_else(data: &[u8]) -> Result<(Vec<TermObj>, usize), AmlError> {
    if data[0] != 0xA1 {
        return Ok((vec!(), 0));
    }

    let (pkg_length, pkg_length_len) = parse_pkg_length(&data[1..])?;
    let term_list = parse_term_list(&data[1 + pkg_length_len .. 1 + pkg_length])?;

    Ok((term_list, pkg_length + 1))
}

fn parse_def_while(data: &[u8]) -> Result<(Type1OpCode, usize), AmlError> {
    if data[0] != 0xA2 {
        return Err(AmlError::AmlParseError);
    }

    let (pkg_length, pkg_length_len) = parse_pkg_length(&data[1..])?;
    let (predicate, predicate_len) = parse_term_arg(&data[1 + pkg_length_len..])?;
    let block = parse_term_list(&data[1 + pkg_length_len + predicate_len .. 1 + pkg_length])?;

    Ok((Type1OpCode::DefWhile {predicate, block}, pkg_length + 1))
}
