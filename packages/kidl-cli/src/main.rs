use clap::Parser;
use tracing_subscriber::util::SubscriberInitExt;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
enum Args {
    LSP(LanguageServerCommand),
}

#[derive(Parser, Debug)]
struct LanguageServerCommand {}

fn main() {
    let _ = tracing_subscriber::fmt()
        .compact()
        .with_writer(std::io::stderr)
        .finish()
        .try_init();

    let args = Args::parse();

    match args {
        Args::LSP(_server) => {
            let _ = kidl_lsp::start();
        }
    }
}
