use async_std;
use aydo_peaq_connector::run;
use cli::args::{Args, Config};
use std::process::id;
use std::{env, process};

#[async_std::main]
async fn main() {
    let raw_args = env::args().skip(1).collect::<Vec<String>>();
    let args = Args::new(raw_args);
    let config = Config::new(args).unwrap_or_else(|err| {
        eprintln!("ARGUMENTS ERROR\n{}", err);
        process::exit(1);
    });

    println!("Aydo <> Peaq Connector");
    println!("PID: {}", id());
    println!("Network: {}", config.network);
    println!("Host: {}", config.host);
    println!("Port: {}", config.port);

    println!("Listening to IoT sensors...");

    let url = format!("{}:{}", config.host, config.port);

    if let Err(e) = run(&url).await {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
