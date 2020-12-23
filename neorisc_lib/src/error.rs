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
    #[error("InvalidRegisterArgument")]
    InvalidRegisterArgument,
    #[error("RegisterIndexOutOfBounds")]
    RegisterIndexOutOfBounds,
    #[error("InvalidConstantArgument")]
    InvalidConstantArgument,
    #[error("ConstantOutOfBounds")]
    ConstantOutOfBounds,
    #[error("InvalidAddressArgument")]
    InvalidAddressArgument,
    #[error("AddressOutOfBounds")]
    AddressOutOfBounds,
    #[error("InvalidCharArgument")]
    InvalidCharArgument,
    #[error("CharOutOfBounds")]
    CharOutOfBounds,
    #[error("ExpectedComma")]
    ExpectedComma,
    #[error("UnexpectedToken")]
    UnexpectedToken,
    #[error("Other exception occured")]
    Other,
}
