use super::token_kind::TokenKind;

pub struct Token {
    pub kind: TokenKind,
    pub literal: String,
}
