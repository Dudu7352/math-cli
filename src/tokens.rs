#[derive(Debug)]
pub enum Token {
    Op(Operator),
    Number(f32),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Operator {
    Plus,
    Minus,
    Star,
    Slash,
    LParen,
    RParen,
    Exponentiation,
}

impl Operator {
    fn weight(&self) -> u8 {
        match self {
            Operator::Plus => 1,
            Operator::Minus => 1,
            Operator::Star => 2,
            Operator::Slash => 2,
            Operator::LParen => 0,
            Operator::RParen => 0,
            Operator::Exponentiation => 3,
        }
    }
}

impl PartialOrd for Operator {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        return Some(self.weight().cmp(&other.weight()));
    }
}
