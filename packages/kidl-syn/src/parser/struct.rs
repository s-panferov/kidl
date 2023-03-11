use rowan::Checkpoint;

use crate::{
    kind::{NodeKind, TokenKind},
    lexer::Token,
    parser::{
        utility::combinators::{kind, TRIVIA_NL},
        Parser,
    },
};

use super::TokenIter;

impl<'c, 't, T: TokenIter<'t>> Parser<'c, 't, T> {
    pub(crate) fn parse_struct(&mut self, checkpoint: Checkpoint, token: Token<'t>) {
        self.builder
            .start_node_at(checkpoint, NodeKind::Struct.into());

        debug_assert_eq!(token.slice, "struct");
        self.consume(token);

        self.expect(kind(TokenKind::CurlyOpen), &TRIVIA_NL, |t| match t.kind {
            TokenKind::CurlyClose => true,
            _ => false,
        });

        while let Some(token) = self.tokens.next() {
            match token.kind {
                trivia_with_newline!() => self.consume(token),
                TokenKind::Ident => self.parse_struct_field(token, TokenKind::Comma),
                TokenKind::CurlyClose => {
                    self.consume(token);
                    break;
                }
                _ => self.unexpected(token),
            }
        }

        self.consume_trivia_until_nl();
        self.builder.finish_node();
    }

    pub(crate) fn parse_struct_field(&mut self, token: Token<'t>, separator: TokenKind) {
        self.builder.start_node(NodeKind::StructField.into());
        self.consume(token);
        self.consume_trivia();
        self.consume_maybe(TokenKind::Question);
        self.consume_trivia();

        self.expect(kind(TokenKind::Colon), &TRIVIA_NL, |t| match t.kind {
            TokenKind::Comma | TokenKind::CurlyClose => true,
            _ => false,
        });

        self.consume_trivia();
        if let Some(ident) = self.maybe(TokenKind::Ident) {
            self.parse_type(ident);
        }

        self.consume_trivia();
        self.consume_maybe(separator);
        self.builder.finish_node()
    }
}
