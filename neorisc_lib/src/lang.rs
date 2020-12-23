use crate::error::LangParseError;

#[derive(Debug, Eq, PartialEq)]
pub enum LangCommand {
    ADD,
    JMP,
}

impl LangCommand {
    pub fn from_string(ss: &str) -> Option<LangCommand> {
        match ss {
            "ADD" => Some(LangCommand::ADD),
            "JMP" => Some(LangCommand::JMP),
            _ => None,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum LangLiteral {
    Register(u8),
    Constant(u8),
    Address(u8),
    Char(char),
    String(String),
}

impl LangLiteral {
    fn parse_prefixed_u8(
        source: &str,
        bounds: (i32, i32),
        prefix_count: usize,
        parse_err: LangParseError,
        oob_error: LangParseError,
    ) -> Result<u8, LangParseError> {
        let number_part = &source[prefix_count..];
        let number_part = number_part.parse::<i32>().map_err(|_| parse_err)?;

        if number_part >= bounds.0 && number_part <= bounds.1 {
            Ok(number_part as u8)
        } else {
            Err(oob_error)
        }
    }

    fn parse_potential_register(ss: &str) -> Result<LangLiteral, LangParseError> {
        let byte = Self::parse_prefixed_u8(
            ss,
            (0, 15),
            1,
            LangParseError::InvalidRegisterArgument,
            LangParseError::RegisterIndexOutOfBounds,
        )?;
        Ok(LangLiteral::Register(byte))
    }

    fn parse_potential_constant(ss: &str) -> Result<LangLiteral, LangParseError> {
        let byte = Self::parse_prefixed_u8(
            ss,
            (0, 255),
            1,
            LangParseError::InvalidConstantArgument,
            LangParseError::ConstantOutOfBounds,
        )?;
        Ok(LangLiteral::Constant(byte))
    }

    fn parse_potential_address(ss: &str) -> Result<LangLiteral, LangParseError> {
        let byte = Self::parse_prefixed_u8(
            ss,
            (0, 255),
            0,
            LangParseError::InvalidConstantArgument,
            LangParseError::ConstantOutOfBounds,
        )?;
        Ok(LangLiteral::Address(byte))
    }

    fn parse_potential_char(ss: &str) -> Result<LangLiteral, LangParseError> {
        if ss.len() != 3 && ss.ends_with('\'') {
            let char = ss
                .chars()
                .next()
                .ok_or(LangParseError::InvalidCharArgument)?;
            if char.len_utf8() != 1 {
                Err(LangParseError::CharOutOfBounds)
            } else {
                Ok(LangLiteral::Char(char))
            }
        } else {
            Err(LangParseError::InvalidCharArgument)
        }
    }

    pub fn from_string(ss: &str) -> Result<LangLiteral, LangParseError> {
        if ss.starts_with('r') {
            Self::parse_potential_register(ss)
        } else if ss.starts_with('#') {
            Self::parse_potential_constant(ss)
        } else if ss.starts_with('\"') {
            Ok(LangLiteral::String(ss.to_string()))
        } else if ss.starts_with('\'') {
            Self::parse_potential_char(ss)
        } else {
            Self::parse_potential_address(ss)
        }
    }
}
