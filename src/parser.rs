use std::collections::HashMap;

//TODO comments, chars, strings, binary, float
#[derive(logos::Logos, Debug, Copy, Clone, Eq, PartialEq)]
pub enum Token {
    EndFile,
    #[error] Unknown,
    #[token("return ")] Return,
    #[token("struct ")] Struct,
    #[token("const ")] Const,
    #[token("if ")] If,
    #[token("else")] Else,
    #[token("else if ")] ElseIf,
    #[token("while ")] While,
    #[token("loop")] Loop,
    #[token("breakif ")] BreakIf,
    #[token("break")] Break,
    #[token(" = ")] Equal,
    #[token(".")] Dot,
    #[token(", ")] Comma,
    #[token(" /")] Slash,
    #[token(":")] Colon,
    #[token("[")] LeftBracket,
    #[token("]")] RightBracket,
    #[token("(")] LeftParen,
    #[token(")")] RightParen,
    #[regex("(    )+", |lex| lex.slice().len() as u16)] Indent(u16),
    #[regex(" \\{\r?\n")] LeftBrace,
    #[regex("}\r?\n")] RightBrace,
    #[regex("\r?\n")] EndLine,
    #[regex("-?[0-9]+")] Integer,
    #[regex("[a-z][_a-z]*")] VarName,
    #[regex("[A-Z][A-Za-z]*")] TypeName,
}

#[test]
fn lexer() {
    let mut lexer = logos::Lexer::<Token>::new(&include_str!("../test.lang"));
    while let Some(token) = lexer.next() {
        println!("{:?} '{}'", token, lexer.slice());
        assert_ne!(token, Token::Unknown);
    }
}
