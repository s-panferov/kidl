use std::path::Path;

use kidl_syn::debug::DebugNodePrinter;
use snapdown::{run_test, Syntax};

fn parser(path: &Path) -> datatest_stable::Result<()> {
    run_test::<Syntax, _>(path, |blocks| {
        blocks.chunks_mut(2).for_each(|test| {
            let input = &test[0];
            let output = &test[1];

            let tree = kidl_syn::parser::parse_str(input.text);
            output
                .result
                .set(Some(format!("{:#?}", DebugNodePrinter(tree))));
        });
    })
}

datatest_stable::harness!(parser, "tests/parser", r".*");
