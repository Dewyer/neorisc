pub mod syntax_analyzer;
use crate::parser::source_parser::{parse_source, LangParsingArtifact};
use std::borrow::Borrow;
use std::time::Instant;

pub struct RiscCompilerConfig {
    max_instruction_count: usize,
}

impl Default for RiscCompilerConfig {
    fn default() -> Self {
        Self {
            max_instruction_count: 256,
        }
    }
}

pub struct RiscCompiler {
    code: String,
    parsed: Option<LangParsingArtifact>,
    config: RiscCompilerConfig,
}

impl RiscCompiler {
    pub fn new(code: String, config: RiscCompilerConfig) -> Self {
        Self {
            code,
            parsed: None,
            config,
        }
    }

    pub fn parse(&mut self) {
        let now = Instant::now();
        let artifact = parse_source(&self.code);
        let took = now.elapsed().as_millis();
        println!("Parsing took: {}", took as f64 / 1000.0);
        if artifact.errors.is_some() {
            println!("Errors!!");
            for (ii, err) in artifact.errors.as_ref().unwrap().iter().enumerate() {
                println!("{} | {} on {}", ii, err.error, err.line);
            }
        }

        self.parsed = Some(artifact);
    }

    pub fn compile(&mut self) {
        self.parse();
        if let Some(parsed) = &self.parsed {
            if let Some(lines) = &parsed.lines {}
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::compiler::{RiscCompiler, RiscCompilerConfig};

    #[test]
    fn it_works() {
        let mut interpreter = RiscCompiler::new(
            "MOV r0, #100\nADD r0, #0xA".to_string(),
            RiscCompilerConfig::default(),
        );
        interpreter.compile();
        let art = &interpreter.parsed.expect("Didn't parse");
        assert_eq!(art.errors.is_none(), true);
    }
}
