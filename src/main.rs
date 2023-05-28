// Copyright PwnWriter // METIS Linux 2022 - present under the MIT License :) 

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = "None")]

struct Arguments {
    #[arg(short, long)]
    /// App to swallow
    app: String,

}

fn main() {
    let arguments = Arguments::parse();
    println!("{}", arguments.app);
}
