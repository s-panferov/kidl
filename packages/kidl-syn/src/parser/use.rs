use rowan::Checkpoint;

use crate::{
    kind::{NodeKind, TokenKind},
    lexer::Token,
    parser::Parser,
};

use super::TokenIter;
impl<'c, 't, T: TokenIter<'t>> Parser<'c, 't, T> {
    pub(crate) fn parse_use(&mut self, checkpoint: Checkpoint, token: Token<'t>) {
        self.builder.start_node_at(checkpoint, NodeKind::Use.into());

        debug_assert!(token.slice == "use");
        self.consume(token);
        self.consume_trivia();

        if !self.maybe_parse_path() {
            let err = format!("Expected valid path, got {:?}", self.tokens.peek());
            self.error(err)
        }

        self.consume_maybe(TokenKind::Semicolon);
        self.consume_trivia_until_nl();

        self.builder.finish_node();
    }
}
