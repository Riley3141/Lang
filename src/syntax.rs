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

pub struct Parsed<'src> {
    pub funcs: HashMap<FuncSig<'src>, Func<'src>>,
    pub types: HashMap<TypeSig<'src>, Type<'src>>,
    pub errors: Vec<String>,
}

#[derive(Debug)]
pub enum Code<'src> {
    ConstInt(u64),
    FuncCall(&'src str),
    FuncArg(&'src str),
    GoTo(usize),
    GoToIf(&'src str),
}

#[derive(Eq, PartialEq, Hash, Debug)]
pub struct TypeSig<'src> {
    pub name: &'src str, // TODO full path
}

#[derive(Debug)]
pub struct Type<'src> {
    pub name: &'src str, // TODO full path
}

#[derive(Eq, PartialEq, Hash, Debug)]
pub struct FuncSig<'src> {
    pub name: &'src str, // TODO full path
    pub arg_types: Vec<&'src str>
}

#[derive(Debug)]
pub struct Func<'src> {
    pub ret_type: &'src str,
    pub code: Vec<Code<'src>>,
}

#[test]
fn lexer() {
    let mut lexer = logos::Lexer::<Token>::new(&include_str!("../test.lang"));
    while let Some(token) = lexer.next() {
        println!("{:?} '{}'", token, lexer.slice());
        assert_ne!(token, Token::Unknown);
    }
}
