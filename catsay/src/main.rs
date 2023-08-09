extern crate structopt;

use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "catsay", about = "It's like cowsay but for cats")]
struct Options {
    message: String
}

fn main() {
    let options = Options::from_args();

    println!("{}", options.message);
    println!("{:>4}", "\\");
    println!("{:>5}", "\\");
    println!("     /\\_/\\");
    println!("    ( o o )");
    println!("    =( I )=");
}
