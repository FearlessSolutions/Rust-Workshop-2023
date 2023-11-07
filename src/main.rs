use clap::{Args, Parser, Subcommand};

fn main() {
    let my_object = GetOutline::parse();

    if let Subcommands::SayHello(args) = &my_object.subcommand {
        if let Some(name) = &args.name {
            println!("Hello, {}!", name);
        } else {
            println!("Hello, world!");
        }
    }
}

/// The GetOutline CLI can list and download documents from Outline
#[derive(Parser)]
#[command(author, version, about, long_about)]
struct GetOutline {
    #[command(subcommand)]
    subcommand: Subcommands,
}

#[derive(Subcommand)]
enum Subcommands {
    /// Print a greeting. We're a polite CLI!
    SayHello(SayHelloArgs),
}

#[derive(Args)]
struct SayHelloArgs {
    /// The name of the person to greet.
    #[arg(long)]
    name: Option<String>,
}
