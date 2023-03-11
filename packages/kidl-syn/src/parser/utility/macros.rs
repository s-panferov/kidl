macro_rules! trivia {
    () => {
        TokenKind::Space | TokenKind::Comment
    };
}

macro_rules! trivia_with_newline {
    () => {
        trivia!() | TokenKind::NewLine
    };
}
