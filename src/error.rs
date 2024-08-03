#[derive(Debug)]
pub enum TokenScanError {
    InvalidNumberLiteral,
    InvalidCharacter,
}

#[derive(Debug, PartialEq)]
pub enum ParseError {
    StackNotEmpty,
    NoOperator,
    NoNumber,
    IncorrectOperator,
}
