mod data;
mod error;

use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    /// The cookie string to use for authentication
    #[clap(env = "COOKIE", short = 'c', long = "cookie")]
    cookie: String,

    /// Player UID to redeem the code for
    #[clap(name = "UID", env = "GAME_UID", short = 'u', long = "uid")]
    uid: String,

    // positional argument for the code
    /// The code to redeem
    code: String,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let client = data::GiftClient::from_cookie_string(&cli.cookie);
    let gift = client.redeem(&cli.code, &cli.uid).await;

    match gift {
        Ok(gift) => {
            println!("Gift: {}", gift);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            // error out
            std::process::exit(1);
        }
    }
}
