use std::collections::HashMap;
use std::slice::Iter;

use crate::atoms::*;
use crate::lexer::{Keyword, Operator, Token, Type};
pub struct Parser {
    tokens: Vec<Token>,
    map: HashMap<String, *const Atom>,
}
struct ParseError;

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            map: HashMap::new(),
        }
    }
    pub fn parse_next(&self, mut iter: Iter<'_, Token>) -> Result<Atom, ParseError> {
        match iter.next() {
            Some(tok) => match tok {
                Token::Keyword(Keyword::Fn) => {}
            },
            None => {
                return Ok(Atom::new(AtomType::Nil, None));
            }
        }
        Ok(Atom::new(atype, body))
    }

    fn parse_function(&mut self, mut iter: Iter<'_, Token>) -> Result<Atom, ParseError> {
        if let Some(Token::Identifier(str)) = iter.next() {
            let id = str.to_string();
            if let Ok(arguments) = Parser::parse_args(&mut iter) {
                if let Some(Token::Arrow) = iter.next() {
                    if let Some(Token::Keyword(Keyword::Type(return_type))) = iter.next() {
                        if let Ok(statements) = self.parse_statements(&mut iter) {
                            let at = Atom::new(
                                AtomType::BoxFunction,
                                Some(Box::new(Function::new(
                                    id.clone(),
                                    arguments,
                                    return_type.clone(),
                                    statements,
                                ))),
                            );
                            self.map.insert(id, &at);
                            return Ok(at);
                        }
                    }
                }
            }
        }
        Err(ParseError)
    }

    fn parse_args(iter: &mut Iter<'_, Token>) -> Result<Vec<Atom>, ParseError> {
        let mut res = Vec::new();
        if let Some(Token::OpenParanthesis) = iter.next() {
            while let Some(tok) = iter.next() {
                if *tok == Token::CloseParanthesis {
                    break;
                }
                if *tok == Token::Comma {
                    continue;
                }
                let id: String;
                let vtype: Type;
                if let Token::Identifier(str) = tok {
                    id = str.to_owned();
                    if let Some(Token::Colon) = iter.next() {
                        if let Some(Token::Keyword(Keyword::Type(t))) = iter.next() {
                            vtype = t.clone();
                            res.push(Atom::new(
                                AtomType::Arg,
                                Some(Box::new(Arg::new(id, vtype))),
                            ));
                        } else {
                            return Err(ParseError);
                        }
                    } else {
                        return Err(ParseError);
                    }
                } else {
                    return Err(ParseError);
                }
            }
        }
        Ok(res)
    }

    fn parse_statements(&mut self, iter: &mut Iter<'_, Token>) -> Result<Vec<Atom>, ParseError> {
        let mut res = Vec::new();
        if let Some(Token::OpenBrace) = iter.next() {
            while let Some(tok) = iter.next() {
                match tok {
                    Token::Keyword(Keyword::Let) => {
                        if let Ok(decordef) = self.parse_decordef(iter) {
                            res.push(decordef);
                        }
                    }
                    Token::Identifier(str) => unsafe {
                        if let Some(&at) = self.map.get(str) {
                            match (*at).atype {
                                AtomType::Variable => {
                                    if let Some(Token::Operator(Operator::Assign)) = iter.next() {
                                        if let Ok(expr) = self.parse_expression(iter) {
                                            res.push(Atom::new(
                                                AtomType::Assignment,
                                                Some(Box::new(Assignment::new(at, expr))),
                                            ))
                                        }
                                    } else {
                                        return Err(ParseError);
                                    }
                                }
                                AtomType::BoxFunction => {
                                    if let Ok(inputs) = self.parse_input(iter) {
                                        res.push(Atom::new(
                                            AtomType::FunctionCall,
                                            Some(Box::new(FunctionCall::new(at, inputs))),
                                        ));
                                    } else {
                                        return Err(ParseError);
                                    }
                                }
                                _ => {
                                    return Err(ParseError);
                                }
                            }
                        }
                    },
                    Token::Keyword(Keyword::Return) => {
                        if let Ok(expr) = self.parse_expression(iter) {
                            res.push(Atom::new(
                                AtomType::Return,
                                Some(Box::new(Return::new(expr))),
                            ));
                        } else {
                            return Err(ParseError);
                        }
                    }
                    Token::CloseBrace => {
                        break;
                    }
                    _ => {
                        return Err(ParseError);
                    }
                }
            }
        }
        Ok(res)
    }

    fn parse_input(&mut self, iter: &mut Iter<'_, Token>) -> Result<Vec<Atom>, ParseError> {
        unimplemented!();
    }

    fn parse_expression(&mut self, iter: &mut Iter<'_, Token>) -> Result<Atom, ParseError> {
        unimplemented!();
    }

    fn parse_decordef(&mut self, iter: &mut Iter<'_, Token>) -> Result<Atom, ParseError> {
        unimplemented!();
    }
}
