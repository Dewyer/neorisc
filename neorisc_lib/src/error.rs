use thiserror::Error;

#[derive(Error, Debug)]
pub enum LangParseError {
    #[error("Duplicate label on line")]
    DuplicateLabel,
    #[error("InvalidLabelName")]
    InvalidLabelName,
    #[error("LabelNameExpected")]
    LabelNameExpected,
    #[error("PossibleForgottenColon")]
    PossibleForgottenColon,
    #[error("InvalidMnemonic")]
    InvalidMnemonic,
    #[error("Other exception occured")]
    Other,
}
