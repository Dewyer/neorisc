use crate::error::LangParseError;
use crate::lang::{LangCommand, LangLiteral};

use regex::Regex;
use std::ops::Add;

lazy_static! {
    static ref LABEL_REGEX: Regex = Regex::new(r"^[A-Za-z_0-9]+$").unwrap();
}

pub struct LangCommandParser<'src> {
    tokens: Vec<&'src str>,
    buffer: String,
    line: SourceLine,
}

pub struct SourceLine {
    pub label: Option<String>,
    pub mnemonic: Option<LangCommand>,
    pub arguments: Vec<LangLiteral>,
}

fn is_label_name_valid(string: &str) -> Result<(), LangParseError> {
    if string == "" {
        Err(LangParseError::LabelNameExpected)
    } else if !LABEL_REGEX.is_match(string) {
        Err(LangParseError::InvalidLabelName)
    } else if LangCommand::from_string(string).is_some() {
        Err(LangParseError::InvalidLabelName)
    } else {
        Ok(())
    }
}

fn is_char_whitespace(chr: char) -> bool {
    match chr {
        ' ' => true,
        '\t' => true,
        '\n' => true,
        '\r' => true,
        _ => false,
    }
}
type IntermediaryResult = Result<(), LangParseError>;

impl<'src> LangCommandParser<'src> {
    pub fn new(source: &'src str) -> Self {
        Self {
            tokens: source.split(vec![" ", "\t"]).collect(),
            buffer: String::new(),
            line: SourceLine {
                label: None,
                mnemonic: None,
                arguments: vec![],
            },
        }
    }

    fn handle_colon(&mut self) -> IntermediaryResult {
        if self.line.label.is_some() {
            return Err(LangParseError::DuplicateLabel);
        }
        is_label_name_valid(&self.buffer)?;

        self.line.label = Some(self.buffer.clone());
        self.buffer = String::new();

        Ok(())
    }

    fn handle_whitespace(&mut self) -> IntermediaryResult {
        if self.line.mnemonic.is_none() {
            if let Some(buffer_cmd) = LangCommand::from_string(&self.buffer) {
                self.line.mnemonic = Some(buffer_cmd);
                self.buffer = String::new();
            } else {
                if self.line.label.is_none() {
                    return Err(LangParseError::PossibleForgottenColon);
                }

                return Err(LangParseError::InvalidMnemonic);
            }
        }

        Ok(())
    }

    pub fn parse(mut self) -> Result<SourceLine, LangParseError> {
        for token in self.tokens.iter() {
            if token == ':' {
                Self::handle_colon(&mut self)?;
            } else if is_char_whitespace(char) {
                Self::handle_whitespace(&mut self)?;
            } else {
                self.buffer.push(char);
            }
        }

        Ok(self.line)
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::RiscCommandParser;

    #[test]
    fn label_parsed() {
        let source_line = RiscCommandParser::new("asd :")
            .parse()
            .expect("Label parsing failed");
        assert_eq!(source_line.label.expect("Label is empty"), "asd");
    }
}
