use rowan::GreenNode;

use crate::kind::{NodeKind, TokenKind};

use super::{Parser, TokenIter};

impl<'c, 't, T: TokenIter<'t>> Parser<'c, 't, T> {
    pub fn parse_schema(mut self) -> GreenNode {
        self.builder.start_node(NodeKind::Root.into());

        let mut checkpoint = self.builder.checkpoint();

        while let Some(token) = self.tokens.next() {
            match token.kind {
                trivia_with_newline!() => self.consume(token),
                TokenKind::Ident => match &*token.slice {
                    "struct" => {
                        self.parse_struct(checkpoint, token);
                        checkpoint = self.builder.checkpoint();
                    }
                    _ => self.unexpected(token),
                },
                _ => self.unexpected(token),
            }
        }

        self.builder.finish_node();
        self.builder.finish()
    }
}
