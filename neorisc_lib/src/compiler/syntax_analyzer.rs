use crate::error::LangSyntaxError;
use crate::lang::LangCommand;
use crate::parser::command_parser::SourceLine;

trait ValidateCommand {
    fn no_args(&self) -> Option<LangSyntaxError>;
}

impl ValidateCommand for SourceLine {
    fn no_args(&self) -> Option<LangSyntaxError> {
        if self.arguments.len() != 0 {
            Some(LangSyntaxError::InstructionTakesZeroArguments)
        } else {
            None
        }
    }
}

fn validate_source_line(line: &SourceLine) -> Option<LangSyntaxError> {
    if let Some(mnemonic) = line.mnemonic.as_ref() {
        match mnemonic {
            LangCommand::CODE => line.no_args(),
            _ => Some(LangSyntaxError::Other),
        }
    } else if line.arguments.len() > 0 {
        Some(LangSyntaxError::UnexpectedArguments)
    } else {
        None
    }
}

pub fn analyze_syntax(lines: &Vec<SourceLine>) -> Vec<LangSyntaxError> {
    let mut errors = Vec::new();
    for line in lines.iter() {
        if let Some(err) = validate_source_line(line) {
            errors.push(err);
        }
    }

    errors
}
