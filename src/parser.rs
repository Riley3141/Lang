use crate::syntax::{
    FuncSig, Func, TypeSig, Type, 
    Code, Parsed, Token, Token::*
};
use std::collections::HashMap;

pub struct Parser<'src> {
    pub parsed: Parsed<'src>,
    vars: HashMap<&'src str, (usize, &'src str)>,
    lexer: logos::Lexer<'src, Token>,
    peek: Token,
    slice: &'src str,
}

impl<'src> Parser<'src> {
    
    pub fn new() -> Self {
        Self {
            parsed: Parsed {
                funcs: HashMap::new(),
                types: HashMap::new(),
                errors: Vec::new(),
            },
            vars: HashMap::new(),
            slice: "",
            peek: EndFile,
            lexer: logos::Lexer::<Token>::new("")
        }
    }

    pub fn parse(&mut self, source: &'src str) {
        self.lexer = logos::Lexer::<Token>::new(source);
        self.next();
        loop {
            match self.next() {
                EndFile => break,
                EndLine => continue,
                Const => self.parse_func(),
                other => self.token_error(other, &[EndFile, EndLine, Const]),
            }
        }
    }

    fn next(&mut self) -> Token {
        let next = self.peek;
        self.slice = self.lexer.slice();
        self.peek = self.lexer.next().unwrap_or(EndFile);
        next
    }

    fn take(&mut self, token: Token) {
        let next = self.next();
        if next != token {
            self.token_error(next, &[token]);
        }
    }

    fn token_error(&mut self, got: Token, want: &[Token]) {
        self.parsed.errors.push(format!(
            "Unexpected token {:?} '{}', Expected {:?}",
            got, self.slice, want));
    }

    fn parse_func(&mut self) {
        self.take(VarName);
        let name = self.slice;
        
        self.take(LeftParen);
        self.vars.clear();
        let mut arg_types = Vec::new();
        if self.peek == RightParen {
            self.next();
        } else {
            loop {
                self.take(VarName);
                let name = self.slice;
                self.take(Slash);
                self.take(TypeName);
                arg_types.push(self.slice);
                self.vars.insert(name, (0, self.slice));
                if self.peek != Comma {
                    break;
                }
                self.next();
            }
            self.take(RightParen);
        }
        
        let ret_type = if self.peek == Slash {
            self.next();
            self.take(TypeName);
            self.slice
        } else {
            ""
        };

        let mut code = Vec::new();
        self.parse_block(&mut code);
        self.parsed.funcs.insert(FuncSig { name, arg_types },
                          Func { ret_type, code });
    }

    fn parse_block(&mut self, code: &mut Vec<Code>, ) {
        self.take(LeftBrace);
        
        self.take(RightBrace);
    }

}

#[test]
fn parser() {
    let mut parser = Parser::new();
    parser.parse(&include_str!("../test.lang"));
    dbg!(&parser.parsed.funcs);
    dbg!(&parser.parsed.types);
    dbg!(&parser.parsed.errors);
    assert_eq!(parser.parsed.errors.len(), 0);
}
