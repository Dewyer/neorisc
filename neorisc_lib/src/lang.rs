pub enum LangCommand {
    ADD,
    JMP,
}

pub enum LangLiteral {
    Register(u8),
    Constant(u8),
    Address(u8),
    Char(char),
    String(String),
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
