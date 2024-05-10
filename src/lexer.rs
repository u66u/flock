extern crate logos;
use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
pub enum Token {
    #[token("bool")]
    TypeBool,

    #[token("else")]
    Else,

    #[token("false")]
    False,

    #[token("fst")]
    Fst,

    #[token("fun")]
    Fun,

    #[token("if")]
    If,

    #[token("int")]
    TypeInt,

    #[token("is")]
    Is,

    #[token("let")]
    Let,

    #[token("list")]
    TypeList,

    #[token("match")]
    Match,

    #[token("rec")]
    Rec,

    #[token("snd")]
    Snd,

    #[token("then")]
    Then,

    #[token("true")]
    True,

    #[token(":quit")]
    Quit,

    #[token("with")]
    With,

    #[token("->")]
    DashArrow,

    #[token("=>")]
    EqualsArrow,

    #[token("::")]
    Cons,

    #[token(";;")]
    DoubleSemicolon,

    #[token("%")]
    Mod,

    #[token("(")]
    LParen,

    #[token(")")]
    RParen,

    #[token("*")]
    Times,

    #[token("+")]
    Plus,

    #[token(",")]
    Comma,

    #[token("-")]
    Minus,

    #[token("/")]
    Divide,

    #[token(":")]
    Colon,

    #[token("<")]
    Less,

    #[token("=")]
    Equal,

    #[token("[")]
    LSquareBrack,

    #[token("]")]
    RSquareBrack,

    #[token("|")]
    Alternative,

    #[regex(r"--[^\n]*\n?", logos::skip)] // Skip comments
    #[regex(r"[ \t\n\r]+", logos::skip)] // Skip whitespace
    Comment,

    #[regex(r"[+-]?([0-9]*)(\.([0-9]+))?([eE][+-]?[0-9]+)?", |lex| lex.slice().parse::<f64>().unwrap())]
    Float(f64),

    #[regex("[+-]?[0-9]+", |lex| lex.slice().parse::<i64>().unwrap())]
    Integer(i64),

    #[regex(r#""([^"\\]|\\["\\bnfrt]|u[a-fA-F0-9]{4})*""#, |lex| lex.slice().to_owned())]
    Var(String),

    #[end]
    EOF,
}
