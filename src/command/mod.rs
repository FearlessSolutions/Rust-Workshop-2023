use clap::{Args, Parser, Subcommand};

// Avoid a garbage collector by passing a reference
// Borrow the args:
pub fn SayHello( args : &SayHelloArgs ){
    match args {
        SayHelloArgs{
            name:Some( user_name)
        } => println!( "Hello, {user_name}!" ),
        SayHelloArgs{
            name: None
        } => println!( "Hello, world!" )
    }
}

/// The GetOutline CLI can list and download documents from Outline
#[derive(Parser)]
#[command(author, version, about, long_about)]
pub struct GetOutline {
    #[command(subcommand)]
    pub subcommand: Subcommands,
}

#[derive(Subcommand)]
pub enum Subcommands {
    /// Print a greeting. We're a polite CLI!
    SayHello(SayHelloArgs),
}

#[derive(Args)]
pub struct SayHelloArgs {
    /// The name of the person to greet.
    #[arg(long)]
    pub name: Option<String>,
}