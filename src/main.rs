#![feature(or_patterns)]

extern crate clap;

mod cli;
mod client;
mod utils;

use clap::AppSettings::ArgRequiredElseHelp;
use clap::{App, Arg, SubCommand};
use cli::*;
use client::create_client;

fn main() {
    let version = "0.1.0";
    let client = create_client();
    println!("{:?}", client);
    let app = App::new("stonks")
        .version(version)
        .author("Henry Boisdequin")
        .about("Stock CLI tool called stonks which uses the Finnhub API")
        .setting(ArgRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("add").arg(
                Arg::with_name("add")
                    .short("a")
                    .help("Adds a new listed stock to your watch list")
                    .takes_value(true),
            ),
        )
        .subcommand(
            SubCommand::with_name("view").arg(
                Arg::with_name("view")
                    .short("v")
                    .help("Views the current stock price of the symbol provided")
                    .takes_value(true),
            ),
        )
        .subcommand(
            SubCommand::with_name("list").arg(
                Arg::with_name("list")
                    .short("l")
                    .help("Lists all your watched stock prices"),
            ),
        );
    let matches = app.get_matches();

    if let Some(cmd) = matches.subcommand_name() {
        match cmd {
            "list" => list(),
            "view" => view(),
            "add" => add(),
            _ => eprintln!("Command not recognized"),
        }
    }
}
