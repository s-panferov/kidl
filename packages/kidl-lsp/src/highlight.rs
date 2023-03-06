use std::{ops::Range, path::Path};

use kidl_db::Database;
use kidl_syn::{
    ast::{
        r#struct::{Struct, StructField},
        r#type::Type,
        schema::{Declaration, Schema},
        AstNode as _, NodeOrToken, SyntaxNode, SyntaxToken,
    },
    kind::{NodeKind, SyntaxKind, TokenKind},
};
use lsp_types::{
    SemanticToken, SemanticTokenModifier, SemanticTokenType, SemanticTokens,
    SemanticTokensFullOptions, SemanticTokensLegend, SemanticTokensOptions, SemanticTokensParams,
    SemanticTokensResult, SemanticTokensServerCapabilities,
};
use ropey::Rope;

#[allow(non_camel_case_types)]
pub enum TokenType {
    NAMESPACE = 0,
    TYPE,
    CLASS,
    ENUM,
    INTERFACE,
    STRUCT,
    TYPE_PARAMETER,
    PARAMETER,
    VARIABLE,
    PROPERTY,
    ENUM_MEMBER,
    EVENT,
    FUNCTION,
    METHOD,
    MACRO,
    KEYWORD,
    MODIFIER,
    COMMENT,
    STRING,
    NUMBER,
    REGEXP,
    OPERATOR,
}

bitflags::bitflags! {
    struct TokenModifier: u32 {
        const DECLARATION     = 0b0000000001;
        const DEFINITION      = 0b0000000010;
        const READONLY        = 0b0000000100;
        const STATIC          = 0b0000001000;
        const DEPRECATED      = 0b0000010000;
        const ABSTRACT        = 0b0000100000;
        const ASYNC           = 0b0001000000;
        const MODIFICATION    = 0b0010000000;
        const DOCUMENTATION   = 0b0100000000;
        const DEFAULT_LIBRARY = 0b1000000000;
    }
}

pub fn capabilities() -> SemanticTokensServerCapabilities {
    SemanticTokensServerCapabilities::SemanticTokensOptions(SemanticTokensOptions {
        legend: SemanticTokensLegend {
            token_types: vec![
                SemanticTokenType::NAMESPACE,
                SemanticTokenType::TYPE,
                SemanticTokenType::CLASS,
                SemanticTokenType::ENUM,
                SemanticTokenType::INTERFACE,
                SemanticTokenType::STRUCT,
                SemanticTokenType::TYPE_PARAMETER,
                SemanticTokenType::PARAMETER,
                SemanticTokenType::VARIABLE,
                SemanticTokenType::PROPERTY,
                SemanticTokenType::ENUM_MEMBER,
                SemanticTokenType::EVENT,
                SemanticTokenType::FUNCTION,
                SemanticTokenType::METHOD,
                SemanticTokenType::MACRO,
                SemanticTokenType::KEYWORD,
                SemanticTokenType::MODIFIER,
                SemanticTokenType::COMMENT,
                SemanticTokenType::STRING,
                SemanticTokenType::NUMBER,
                SemanticTokenType::REGEXP,
                SemanticTokenType::OPERATOR,
            ],
            token_modifiers: vec![
                SemanticTokenModifier::DECLARATION,
                SemanticTokenModifier::DEFINITION,
                SemanticTokenModifier::READONLY,
                SemanticTokenModifier::STATIC,
                SemanticTokenModifier::DEPRECATED,
                SemanticTokenModifier::ABSTRACT,
                SemanticTokenModifier::ASYNC,
                SemanticTokenModifier::MODIFICATION,
                SemanticTokenModifier::DOCUMENTATION,
                SemanticTokenModifier::DEFAULT_LIBRARY,
            ],
        },
        full: Some(SemanticTokensFullOptions::Bool(true)),
        ..Default::default()
    })
}

pub struct DeltaEncoder<'a> {
    prev_line: usize,
    prev_offset: usize,
    buffer: Vec<SemanticToken>,
    source: &'a Rope,
}

impl<'a> DeltaEncoder<'a> {
    pub fn new(source: &'a Rope) -> Self {
        DeltaEncoder {
            prev_line: 0,
            prev_offset: 0,
            buffer: Vec::new(),
            source,
        }
    }

    fn push(&mut self, token: &SyntaxToken, ty: TokenType) {
        let token = self.encode(token, ty, TokenModifier::empty());
        self.buffer.push(token)
    }

    #[allow(unused)]
    fn push_mod(&mut self, token: &SyntaxToken, ty: TokenType, modifier: TokenModifier) {
        let token = self.encode(token, ty, modifier);
        self.buffer.push(token)
    }

    fn encode(
        &mut self,
        token: &SyntaxToken,
        ty: TokenType,
        modifier: TokenModifier,
    ) -> SemanticToken {
        let range: Range<usize> = token.text_range().into();
        let line = self.source.byte_to_line(range.start);
        let line_offset = self.source.line_to_byte(line);
        let offset = range.start - line_offset;

        let delta_line = line - self.prev_line;

        if self.prev_line < line {
            self.prev_offset = 0;
        }

        self.prev_line = line;

        let delta_start = offset - self.prev_offset;
        self.prev_offset = offset;

        SemanticToken {
            delta_line: delta_line as u32,
            delta_start: delta_start as u32,
            length: range.len() as u32,
            token_type: ty as u32,
            token_modifiers_bitset: modifier.bits(),
        }
    }
}

pub fn collect_semantic_tokens_from_comment(token: &SyntaxToken, encoder: &mut DeltaEncoder) {
    encoder.push(token, TokenType::COMMENT)
}

fn collect_semantic_tokens_from_struct_field(arg: &StructField, encoder: &mut DeltaEncoder) {
    for child in arg.syntax().children_with_tokens() {
        match child {
            NodeOrToken::Token(token) => match token.kind() {
                SyntaxKind::Token(TokenKind::Ident) => encoder.push(&token, TokenType::PARAMETER),
                _ => {}
            },
            NodeOrToken::Node(node) => match node.kind() {
                SyntaxKind::Node(NodeKind::Type) => {
                    collect_semantic_tokens_from_type(&Type::cast(node).unwrap(), encoder)
                }
                _ => {}
            },
        }
    }
}

fn collect_semantic_tokens_from_type(ty: &Type, encoder: &mut DeltaEncoder) {
    for child in ty.syntax().children_with_tokens() {
        match child {
            NodeOrToken::Token(token) => match token.kind() {
                SyntaxKind::Token(TokenKind::Ident) => encoder.push(&token, TokenType::TYPE),
                _ => {}
            },
            NodeOrToken::Node(node) => match node.kind() {
                SyntaxKind::Node(NodeKind::TypeArguments) => {
                    node.children().for_each(|c| {
                        if let Some(ty) = Type::cast(c) {
                            collect_semantic_tokens_from_type(&ty, encoder)
                        }
                    });
                }
                _ => {}
            },
        }
    }
}

fn collect_semantic_tokens_from_struct(s: &Struct, encoder: &mut DeltaEncoder) {
    for child in s.syntax().children_with_tokens() {
        match child {
            NodeOrToken::Token(token) => match token.kind() {
                SyntaxKind::Token(TokenKind::Comment) => {
                    collect_semantic_tokens_from_comment(&token, encoder)
                }
                SyntaxKind::Token(TokenKind::Ident) if token.text() == "struct" => {
                    encoder.push(&token, TokenType::KEYWORD);
                }
                SyntaxKind::Token(TokenKind::Ident) => encoder.push(&token, TokenType::STRUCT),
                _ => {}
            },
            NodeOrToken::Node(node) => match node.kind() {
                SyntaxKind::Node(NodeKind::StructField) => {
                    collect_semantic_tokens_from_struct_field(
                        &StructField::cast(node).unwrap(),
                        encoder,
                    )
                }
                _ => {}
            },
        }
    }
}

pub fn collect_semantic_tokens_from_schema(node: &Schema, encoder: &mut DeltaEncoder) {
    for decl in node.declarations() {
        match decl {
            Declaration::Struct(s) => collect_semantic_tokens_from_struct(&s, encoder),
            _ => {}
        }
    }
}

pub fn semantic_tokens(
    db: &mut Database,
    params: SemanticTokensParams,
) -> Option<SemanticTokensResult> {
    let path = Path::new(params.text_document.uri.path());

    let Some(schema_file) = db.schema_file(path) else  {
        return None;
    };

    let parsed = kidl_db::source::parse(db, schema_file);

    let source = schema_file.text(db);
    let schema = SyntaxNode::new_root(parsed);

    let mut delta = DeltaEncoder::new(&source);

    let schema = Schema::cast(schema).unwrap();

    collect_semantic_tokens_from_schema(&schema, &mut delta);

    Some(SemanticTokensResult::Tokens(SemanticTokens {
        data: delta.buffer,
        ..Default::default()
    }))
}
