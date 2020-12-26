use crate::error::LangParseError;
use crate::lang::{LangCommand, LangLiteral};
use crate::parser::line_tokenizer::tokenize_source_line;

pub struct LangCommandParserContext {
    line: SourceLine,
    last_token_was_argument: bool,
    last_token_was_label: bool,
}

pub struct LangCommandParserTokenContext<'src> {
    at_index: usize,
    at_token: &'src str,
    tokens: &'src Vec<String>,
}

impl<'src> LangCommandParserTokenContext<'src> {
    pub fn previous_token(&self) -> Option<&'src str> {
        if self.at_index != 0 {
            Some(&self.tokens[self.at_index - 1])
        } else {
            None
        }
    }

    pub fn next_token(&self) -> Option<&'src str> {
        if self.at_index != self.tokens.len() - 1 {
            Some(&self.tokens[self.at_index + 1])
        } else {
            None
        }
    }
}

pub struct SourceLine {
    pub label: Option<String>,
    pub mnemonic: Option<LangCommand>,
    pub arguments: Vec<LangLiteral>,
}

fn handle_expected_label(
    mut ctx: LangCommandParserContext,
    token_ctx: &LangCommandParserTokenContext,
) -> Result<LangCommandParserContext, LangParseError> {
    ctx.line.label = Some(LangLiteral::validate_symbol_name(token_ctx.at_token)?.to_string());
    ctx.last_token_was_label = true;

    Ok(ctx)
}

pub fn handle_expected_mnemonic(
    mut ctx: LangCommandParserContext,
    token_ctx: &LangCommandParserTokenContext,
) -> Result<LangCommandParserContext, LangParseError> {
    if let Some(at_mnemonic) = LangCommand::from_string(token_ctx.at_token) {
        ctx.line.mnemonic = Some(at_mnemonic);
    } else if let Some(next_token) = token_ctx.next_token() {
        if next_token != ":" {
            return Err(LangParseError::InvalidMnemonic);
        }
    } else {
        return Err(LangParseError::InvalidMnemonic);
    }

    Ok(ctx)
}

pub fn parse_command_line(line: &str) -> Result<SourceLine, LangParseError> {
    let tokens = tokenize_source_line(line)?;
    let mut ctx = LangCommandParserContext {
        line: SourceLine {
            label: None,
            mnemonic: None,
            arguments: vec![],
        },
        last_token_was_argument: false,
        last_token_was_label: false,
    };

    for (ii, token) in tokens.iter().enumerate() {
        let token_ctx = LangCommandParserTokenContext {
            at_index: ii,
            at_token: token,
            tokens: &tokens,
        };
        let mut to_set_was_arg = false;
        let mut to_set_was_label = false;

        if token == ":" {
            if !ctx.last_token_was_label {
                return Err(LangParseError::UnexpectedToken);
            }
        } else if ctx.line.label.is_none()
            && ctx.line.mnemonic.is_none()
            && token_ctx.next_token().map_or_else(|| false, |v| v == ":")
        {
            ctx = handle_expected_label(ctx, &token_ctx)?;
            to_set_was_label = true;
        } else if ctx.line.mnemonic.is_none() {
            ctx = handle_expected_mnemonic(ctx, &token_ctx)?;
        } else {
            if ctx.last_token_was_argument && token != "," {
                return Err(LangParseError::ExpectedComma);
            } else if token_ctx
                .previous_token()
                .map_or_else(|| false, |v| v == ",")
                && token == ","
            {
                return Err(LangParseError::UnexpectedToken);
            } else if !ctx.last_token_was_argument && token != "," {
                let arg = LangLiteral::from_string(token)?;
                ctx.line.arguments.push(arg);
                to_set_was_arg = true;
            }
        }

        ctx.last_token_was_argument = to_set_was_arg;
        ctx.last_token_was_label = to_set_was_label;
    }

    Ok(ctx.line)
}

#[cfg(test)]
mod tests {
    use crate::lang::{LangCommand, LangLiteral};
    use crate::parser::command_parser::parse_command_line;
    use std::borrow::Borrow;

    #[test]
    fn label_parsed() {
        let source_line = parse_command_line("asd :").expect("Line parsing failed");
        assert_eq!(source_line.label.expect("Label is empty"), "asd");
    }

    #[test]
    fn mnemonic_parsed() {
        let source_line = parse_command_line("ADD").expect("Line parsing failed");
        assert_eq!(
            source_line.mnemonic.expect("Mnemonic is empty"),
            LangCommand::ADD
        );
    }

    #[test]
    fn comibned_parsed() {
        let source_line = parse_command_line("asd : ADD").expect("Line parsing failed");
        assert_eq!(source_line.label.expect("Label is empty"), "asd");
        assert_eq!(
            source_line.mnemonic.expect("Mnemonic is empty"),
            LangCommand::ADD
        );
    }

    #[test]
    fn args_parsed() {
        let source_line =
            parse_command_line("ADD r0, 100, LD, 0xff, #0b101").expect("Line parsing failed");
        assert_eq!(source_line.arguments.len(), 5);
        if let LangLiteral::Register(reg) = source_line.arguments[0] {
            assert_eq!(reg, 0);
        } else {
            assert!(false);
        }

        if let LangLiteral::Address(addr) = source_line.arguments[1] {
            assert_eq!(addr, 100);
        } else {
            assert!(false);
        }

        if let LangLiteral::Symbol(symb) = source_line.arguments[2].borrow() {
            assert_eq!(symb, "LD");
        } else {
            assert!(false);
        }

        if let LangLiteral::Address(addr) = source_line.arguments[3] {
            assert_eq!(addr, 255);
        } else {
            assert!(false);
        }

        if let LangLiteral::Constant(addr) = source_line.arguments[4] {
            assert_eq!(addr, 5);
        } else {
            assert!(false);
        }
    }
}
