use crate::{
    kind::{NodeKind, TokenKind},
    lexer::Token,
    parser::Parser,
};

use super::TokenIter;

impl<'c, 't, T: TokenIter<'t>> Parser<'c, 't, T> {
    pub(crate) fn parse_type(&mut self, ident: Token<'t>) {
        self.builder.start_node(NodeKind::Type.into());

        self.consume(ident);

        let checkpoint = self.builder.checkpoint();
        if self.consume_maybe(TokenKind::AngleOpen) {
            self.builder
                .start_node_at(checkpoint, NodeKind::TypeArguments.into());
            // has type arguments
            self.consume_trivia();

            if let Some(ident) = self.maybe(TokenKind::Ident) {
                self.parse_type(ident);
            }

            self.consume_trivia();
            while self.consume_maybe(TokenKind::Comma) {
                self.consume_trivia();
                if let Some(ident) = self.maybe(TokenKind::Ident) {
                    self.parse_type(ident);
                }
            }
            self.expect_token(TokenKind::AngleClose);
            self.builder.finish_node();
        }

        self.builder.finish_node()
    }
}
