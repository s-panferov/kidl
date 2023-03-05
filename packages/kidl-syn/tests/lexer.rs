use std::path::Path;

use kidl_syn::source::StrSource;
use snapdown::{run_test, Syntax};

fn lexer(path: &Path) -> datatest_stable::Result<()> {
    run_test::<Syntax, _>(path, |blocks| {
        blocks.chunks_mut(2).for_each(|test| {
            let input = &test[0];
            let output = &test[1];

            let tree = kidl_syn::lexer::tokenize(StrSource::new(input.text))
                .map(|t| format!("{:?}", t))
                .collect::<Vec<_>>();

            output.result.set(Some(format!("{:#?}", tree)));
        });
    })
}

datatest_stable::harness!(lexer, "tests/lexer", r".*");
