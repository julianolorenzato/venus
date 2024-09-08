use clap::Parser;

#[derive(Parser)]
#[command(author, version, about="Assemble Venus mnemonic code.", long_about = None)]
struct AssemblerArgs {
    #[arg(short, long)]
    filename: String,
}

pub fn run() {
    let args = AssemblerArgs::parse();

    println!("Searching for file at {}", args.filename);

    super::run();
}
