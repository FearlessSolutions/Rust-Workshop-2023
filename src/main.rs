use clap::{Parser};

mod command;
mod get_outline_connection;

fn main() {
    let my_object = command::GetOutline::parse();

    match &my_object.subcommand {
        command::Subcommands::SayHello( args ) => command::SayHello( args )
    }
}
