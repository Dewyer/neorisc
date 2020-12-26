use crate::error::LangParseError;
use crate::parser::command_parser::{parse_command_line, SourceLine};
use rayon::prelude::*;

pub struct LangParsingArtifact {
    pub lines: Option<Vec<SourceLine>>,
    pub errors: Option<Vec<ParsingError>>,
}

pub struct ParsingError {
    pub line: usize,
    pub error: LangParseError,
}

pub fn parse_source(source: &str) -> LangParsingArtifact {
    let results = source
        .lines()
        .collect::<Vec<&str>>()
        .par_iter()
        .filter(|&line| line != &"")
        .map(|&line| parse_command_line(line))
        .collect::<Vec<Result<SourceLine, LangParseError>>>();

    let mut parsed_lines = Vec::new();
    let mut errors = Vec::new();
    for (ii, result) in results.into_iter().enumerate() {
        match result {
            Ok(source_line) => parsed_lines.push(source_line),
            Err(err) => errors.push(ParsingError {
                line: ii,
                error: err,
            }),
        }
    }
    let had_errors = errors.len() > 0;

    LangParsingArtifact {
        errors: if had_errors { Some(errors) } else { None },
        lines: if !had_errors {
            Some(parsed_lines)
        } else {
            None
        },
    }
}
