use crate::error::LangParseError;

struct LangTokenizerContext {
    pub buffer: String,
    pub in_char: bool,
    pub in_string: bool,
    pub tokens: Vec<String>,
    pub last_backslash: bool,
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

fn set_in_char_string(&mut ctx: LangTokenizerContext, new_char: char) {
    if !ctx.last_backslash {
        if new_char == '\'' {
            ctx.in_char = ctx.in;
        }
    }
}

pub fn tokenize_source_line(line: &str) -> Result<Vec<String>, LangParseError> {
    let mut ctx = LangTokenizerContext {
        buffer: String::new(),
        in_char: false,
        in_string: false,
        tokens: vec![],
        last_backslash: false,
    };

    for char in line.chars() {
        if !ctx.in_string && !ctx.in_char {
        } else {
            if ctx.last_backslash {}
        }

        if !ctx.last_backslash && char == '\\' {
            ctx.last_backslash = true;
        }
    }

    Ok(ctx.tokens)
}
