mod data;
mod error;

use std::io::BufRead;

use clap::Parser;


#[derive(Parser)]
pub struct Cli {
    /// The cookie string to use for authentication
    #[clap(env = "COOKIE", short = 'c', long = "cookie")]
    cookie: String,

    /// Player UID to redeem the code for
    #[clap(name = "UID", env = "GAME_UID", short = 'u', long = "uid")]
    uid: String,

    /// Take input from stdin
    #[clap(short = 'i', long = "stdin")]
    stdin: bool,
    
    // positional argument for the code
    /// The code to redeem. Can be provided multiple times.
    /// Optional if the code is provided via stdin
    #[clap(name = "CODES", required_unless_present("stdin"))]
    code: Vec<String>,
    
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let client = data::GiftClient::from_cookie_string(&cli.cookie);
    
    let giftcodes = if cli.stdin {
        std::io::stdin().lock().lines().map(|l| l.unwrap()).collect::<Vec<String>>()
    } else {
        cli.code
    };
    
    
    // let gift = client.redeem(&cli.code, &cli.uid).await;

    // match gift {
    //     Ok(gift) => {
    //         println!("Gift: {}", gift);
    //     }
    //     Err(e) => {
    //         eprintln!("Error: {}", e);
    //         // error out
    //         std::process::exit(1);
    //     }
    // }
    // 
    
    
    // Redeem then wait for 5 seconds to avoid rate limiting
    
    for code in giftcodes {
        
        let gift = client.redeem(&code, &cli.uid).await;
        
        match gift {
            Ok(gift) => {
                println!("Gift: {}", gift);
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                // error out
                // std::process::exit(1);
            }
        }
        tokio::time::sleep(std::time::Duration::from_secs(5)).await
    }
}
