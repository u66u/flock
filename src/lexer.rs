extern crate logos;
use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
pub enum Token {
    #[token("bool")]
    TBool,

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
    TInt,

    #[token("is")]
    Is,

    #[token("let")]
    Let,

    #[token("list")]
    TList,

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
    TArrow,

    #[token("=>")]
    DArrow,

    #[token("::")]
    Cons,

    #[token(";;")]
    Semicolon2,

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
    LBrack,

    #[token("]")]
    RBrack,

    #[token("|")]
    Alternative,

    #[regex(r"--[^\n]*\n?", logos::skip)] // Skip comments
    #[regex(r"[ \t\n\r]+", logos::skip)] // Skip whitespace
    Comment,

    #[regex(r"[0-9]+")]
    Int,

    #[end]
    EOF,
}
