use clap::Parser;

// #[derive(Clone)]
// enum Uploader {
//     Catgirls,
//     TheNullPointer,
// }

#[derive(Parser)]
struct Args {
    uploader: String,
}

fn main() {
    let args = Args::parse();
    println!("{}", args.uploader);
}
