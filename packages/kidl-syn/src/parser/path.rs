use crate::{
    kind::{NodeKind, TokenKind},
    parser::Parser,
};

use super::{utility::keyword::is_keyword, TokenIter};
impl<'c, 't, T: TokenIter<'t>> Parser<'c, 't, T> {
    pub(crate) fn maybe_parse_path(&mut self) -> bool {
        let checkpoint = self.builder.checkpoint();

        let mut consumed = false;
        while let Some(token) = self.tokens.peek() {
            match token.kind {
                TokenKind::Ident if !is_keyword(token) => {
                    self.consume_next();
                    self.consume_trivia();
                    consumed = true;

                    if self.consume_maybe(TokenKind::Colon) && self.consume_maybe(TokenKind::Colon)
                    {
                        continue;
                    } else {
                        break;
                    }
                }
                TokenKind::String => {
                    self.consume_next();
                    self.consume_trivia();
                    consumed = true;

                    if self.consume_maybe(TokenKind::Colon) && self.consume_maybe(TokenKind::Colon)
                    {
                        continue;
                    } else {
                        break;
                    }
                }
                _ => {
                    break;
                }
            }
        }

        if consumed {
            self.builder
                .start_node_at(checkpoint, NodeKind::Path.into());
            self.builder.finish_node();
        }

        return consumed;
    }
}
