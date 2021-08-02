use super::path_parser::ParseNode;
use super::str_reader::StrRange;
use super::tokens::{FilterToken, ParseToken};

pub trait ParseNodeVisitor<'a> {
    fn visit<F, F1>(&self, parse_node: &ParseNode, token_handler: &mut F, parse_value_reader: &F1)
        where
            F: ParseTokenHandler<'a>,
            F1: Fn(&StrRange) -> &'a str
    {
        trace!("visit {:?}", parse_node);
        match &parse_node.token {
            ParseToken::Absolute
            | ParseToken::Relative
            | ParseToken::All
            | ParseToken::Key(_)
            | ParseToken::Keys(_)
            | ParseToken::Range(_, _, _)
            | ParseToken::Union(_)
            | ParseToken::Number(_)
            | ParseToken::Bool(_) => {
                token_handler.handle(&parse_node.token, parse_value_reader);
            }
            ParseToken::In | ParseToken::Leaves => {
                if let Some(n) = &parse_node.left {
                    self.visit(&*n, token_handler, parse_value_reader);
                }

                token_handler.handle(&parse_node.token, parse_value_reader);

                if let Some(n) = &parse_node.right {
                    self.visit(&*n, token_handler, parse_value_reader);
                }
            }
            ParseToken::Array => {
                if let Some(n) = &parse_node.left {
                    self.visit(&*n, token_handler, parse_value_reader);
                }

                token_handler.handle(&parse_node.token, parse_value_reader);

                if let Some(n) = &parse_node.right {
                    self.visit(&*n, token_handler, parse_value_reader);
                }

                token_handler.handle(&ParseToken::ArrayEof, parse_value_reader);
            }
            ParseToken::Filter(FilterToken::And) | ParseToken::Filter(FilterToken::Or) => {
                if let Some(n) = &parse_node.left {
                    self.visit(&*n, token_handler, parse_value_reader);
                }

                if let Some(n) = &parse_node.right {
                    self.visit(&*n, token_handler, parse_value_reader);
                }

                token_handler.handle(&parse_node.token, parse_value_reader);
            }
            ParseToken::Filter(_) => {
                if let Some(n) = &parse_node.left {
                    self.visit(&*n, token_handler, parse_value_reader);
                }

                if let Some(n) = &parse_node.right {
                    self.visit(&*n, token_handler, parse_value_reader);
                }

                token_handler.handle(&parse_node.token, parse_value_reader);
            }
            _ => {}
        }
    }
}

pub trait ParseTokenHandler<'a> {
    fn handle<F>(&mut self, token: &ParseToken, parse_value_reader: &F)
        where
            F: Fn(&StrRange) -> &'a str;
}