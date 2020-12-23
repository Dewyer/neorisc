use crate::error::LangParseError;

struct LangTokenizerContext {
    pub buffer: String,
    pub in_char: bool,
    pub in_string: bool,
    pub tokens: Vec<String>,
    pub last_backslash: bool,
    pub next_char_to_add: Option<char>,
}

fn flush_buffer(mut ctx: LangTokenizerContext) -> LangTokenizerContext {
    if ctx.buffer.len() > 0 {
        ctx.tokens.push(ctx.buffer);
    }
    ctx.buffer = String::new();

    ctx
}

fn get_escaped_character(next_char: char) -> char {
    match next_char {
        '\\' => '\\',
        't' => '\t',
        'n' => '\n',
        'r' => '\r',
        _ => next_char,
    }
}

fn get_next_escaped_char(ctx: &LangTokenizerContext, next_char: char) -> char {
    if !ctx.last_backslash {
        next_char
    } else {
        get_escaped_character(next_char)
    }
}

pub fn tokenize_source_line(line: &str) -> Result<Vec<String>, LangParseError> {
    let mut ctx = LangTokenizerContext {
        buffer: String::new(),
        in_char: false,
        in_string: false,
        tokens: vec![],
        last_backslash: false,
        next_char_to_add: None,
    };

    for char in line.chars() {
        match char {
            '\\' => {
                if ctx.last_backslash {
                    ctx.buffer.push('\\');
                } else {
                    ctx.last_backslash = true;
                }
            }
            ' ' | '\r' | '\n' | '\t' => {
                if ctx.in_char || ctx.in_string {
                    ctx.buffer.push(char);
                } else {
                    ctx = flush_buffer(ctx);
                }
            }
            '\"' | '\'' => {
                if !ctx.last_backslash {
                    if char == '\"' {
                        ctx.in_string = !ctx.in_string;
                    } else {
                        ctx.in_char = !ctx.in_char;
                    }
                }
                ctx.buffer.push(char);
            }
            ',' | ':' => {
                ctx = flush_buffer(ctx);
                ctx.buffer.push(char);
                ctx = flush_buffer(ctx);
            }
            ';' => {
                if ctx.last_backslash || ctx.in_string || ctx.in_char {
                    ctx.buffer.push(char);
                } else {
                    break;
                }
            }
            _ => {
                ctx.buffer.push(get_next_escaped_char(&ctx, char));
            }
        }
    }
    ctx = flush_buffer(ctx);

    Ok(ctx.tokens)
}

#[cfg(test)]
mod tests {
    use crate::parser::line_tokenizer::tokenize_source_line;

    #[test]
    fn it_works() {
        let tokens = tokenize_source_line("hello  : world").expect("Tokenizer failed");

        println!("tokens: {:?}", tokens);
        assert_eq!(tokens.len(), 3);
    }

    #[test]
    fn char_parse() {
        let tokens = tokenize_source_line("hello ' '").expect("Tokenizer failed");

        println!("tokens: {:?}", tokens);
        println!("second: {}", tokens[1]);
        assert_eq!(tokens.len(), 2);
    }

    #[test]
    fn str_parse() {
        let tokens =
            tokenize_source_line("MOV r0, \"Hello World!\\n\\r\"").expect("Tokenizer failed");

        println!("tokens: {:?}", tokens);
        println!("last: {}", tokens[3]);
        assert_eq!(tokens.len(), 4);
    }
}
