use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
pub enum Token {
    #[token(",")]
    Comma,

    #[token("\n")]
    NewLine,

    #[regex(r"\.[a-zA-Z]+ [a-zA-Z0-9]+")]
    Metadata,

    #[regex("[a-zA-Z][a-zA-Z0-9]+:")]
    Label,

    #[regex("(R|r)(1|2|3|4|5|6|(PC|pc)|(SP|sp)|(BP|bp))")]
    Register,

    #[regex("(0x[0-9a-fA-F]+|[0-9]+)")]
    Number,

    #[regex(r"\$(0[xX][0-9a-fA-F]+|[0-9]+)")]
    Address,

    #[regex("[a-zA-Z][a-zA-Z0-9]+")]
    Identifier,

    #[error]
    #[regex(r"[ \t\f]+", logos::skip)]
    #[regex(r";.+", logos::skip)]
    Error,
}