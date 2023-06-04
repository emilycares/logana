use clap::Parser;
use logana::core::config::Args;
use logana::run;

#[tokio::main]
async fn main() {
    let args = Args::parse();

    if let Ok(dir) = std::env::current_dir() {
        if let Some(dir) = dir.to_str() {
            run(args, dir).await;
        }
    }
}
