pub struct RiscInterpreter {
    source_lines: Vec<String>,
}

impl RiscInterpreter {
    pub fn new(code: String) -> Self {
        Self {
            source_lines: code.split(" ").map(|str| str.to_string()).collect(),
        }
    }
}
