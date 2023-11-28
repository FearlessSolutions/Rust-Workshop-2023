use std::{env, process};
use clap::{Parser};
use dotenvy::dotenv;

mod command;
mod get_outline_connection;

fn main() {
    if dotenv().is_err() {
        println!("Failed to read .env file");
    }
    let Ok(api_key) = env::var("GETOUTLINE_API_KEY") else {
        println!("Could not find API key in environment!");
        process::exit(1);
    };

    let client = match get_outline_connection::blocking_client(&api_key) {
        Ok(ok_res) => ok_res,
        Err(err_res) => {
            println!("Could not construct client");
            println!("{}", err_res);
            process::exit(1);
        }
    };
    let docs = match get_outline_connection::documents::list(&client) {
        Ok(ok_docs) => ok_docs,
        Err(err_docs) => {
            println!("Could not parse docs");
            println!("{}", err_docs);
            process::exit(1);
        }
    };
    println!("{:#?}", docs)

    // Namespace navigation is done with ::

    // let my_object = command::GetOutline::parse();
    //
    // match &my_object.subcommand {
    //     command::Subcommands::SayHello( args ) => command::SayHello( args )
    // }
}
