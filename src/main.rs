use clap::Parser;
use logana::core::config::Args;
use logana::run;

#[tokio::main]
async fn main() {
    let args = Args::parse();

    run(args).await;
}
