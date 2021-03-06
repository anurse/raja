#[derive(Copy,Clone,Debug,Eq,PartialEq)]
pub enum Operator {
    LBrace,
    RBrace,
    LParen,
    RParen,
    LBracket,
    RBracket,
    Dot,
    Elipsis,
    Semicolon,
    Comma,
    LessThan,
    GreaterThan,
    LessThanEqual,
    GreaterThanEqual,
    Equal,
    NotEqual,
    Identical,
    NotIdentical,
    Plus,
    Minus,
    Star,
    Divide,
    Percent,
    Increment,
    Decrement,
    LeftShift,
    RightShift,
    UnsignedRightShift,
    And,
    Or,
    Xor,
    Not,
    Tilde,
    AndAnd,
    OrOr,
    Question,
    Colon,
    Assign,
    PlusAssign,
    MinusAssign,
    StarAssign,
    DivideAssign,
    PercentAssign,
    LeftShiftAssign,
    RightShiftAssign,
    UnsignedRightShiftAssign,
    AndAssign,
    OrAssign,
    XorAssign,
    Arrow
}
