use crate::error::LangParseError;
use regex::Regex;

lazy_static! {
    static ref LABEL_REGEX: Regex = Regex::new(r"^[A-Za-z_0-9]+$").unwrap();
}

#[derive(Debug, Eq, PartialEq)]
pub enum LangCommand {
    DEF,
    CODE,
    DATA,
    DB,
    ORG,
    ADD,
    JMP,
    MOV,
}

impl LangCommand {
    pub fn from_string(ss: &str) -> Option<LangCommand> {
        match ss {
            "DEF" => Some(LangCommand::DEF),
            "CODE" => Some(LangCommand::CODE),
            "DATA" => Some(LangCommand::DATA),
            "DB" => Some(LangCommand::DB),
            "ORG" => Some(LangCommand::ORG),
            "ADD" => Some(LangCommand::ADD),
            "JMP" => Some(LangCommand::JMP),
            "MOV" => Some(LangCommand::MOV),
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
    Symbol(String),
}

impl LangLiteral {
    pub fn validate_symbol_name(string: &str) -> Result<&str, LangParseError> {
        if string == "" {
            Err(LangParseError::LabelNameExpected)
        } else if !LABEL_REGEX.is_match(string) {
            Err(LangParseError::InvalidSymbolName)
        } else if LangCommand::from_string(string).is_some() {
            Err(LangParseError::InvalidSymbolName)
        } else {
            Ok(string)
        }
    }

    fn parse_radix_to_try_from_str(ss: &str) -> u32 {
        if ss.starts_with("0x") {
            16
        } else if ss.starts_with("0b") {
            2
        } else {
            10
        }
    }

    fn parse_prefixed_u8(
        source: &str,
        bounds: (i32, i32),
        prefix_count: usize,
        allow_non_decimals: bool,
        parse_err: LangParseError,
        oob_error: LangParseError,
    ) -> Result<u8, LangParseError> {
        let mut number_part = &source[prefix_count..];
        let radix = Self::parse_radix_to_try_from_str(number_part);
        if radix != 10 && !allow_non_decimals {
            return Err(parse_err);
        }

        if radix != 10 {
            number_part = &number_part[2..];
        }

        let number_part = i32::from_str_radix(number_part, radix).map_err(|_| parse_err)?;

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
            false,
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
            true,
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
            true,
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

    fn string_starts_with_number(ss: &str) -> bool {
        if let Some(chr) = ss.chars().next() {
            chr.is_digit(10)
        } else {
            false
        }
    }

    fn parse_potential_symbol(ss: &str) -> Result<LangLiteral, LangParseError> {
        Ok(LangLiteral::Symbol(
            Self::validate_symbol_name(ss)?.to_string(),
        ))
    }

    pub fn from_string(ss: &str) -> Result<LangLiteral, LangParseError> {
        if ss.starts_with('r') {
            Self::parse_potential_register(ss)
        } else if ss.starts_with('#') {
            Self::parse_potential_constant(ss)
        } else if ss.starts_with('\"') {
            Ok(LangLiteral::String(ss.to_string()))
        } else if ss.starts_with("#'") {
            Self::parse_potential_char(ss)
        } else if Self::string_starts_with_number(ss) {
            Self::parse_potential_address(ss)
        } else {
            Self::parse_potential_symbol(ss)
        }
    }
}
