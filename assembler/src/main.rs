use assembler::Assembler;
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about="Assemble Venus mnemonic code.", long_about = None)]
struct AssemblerArgs {
    #[arg(short, long)]
    filename: String,
}

fn main() {
    let asmr = Assembler::new("assembler/test.asm");

    match asmr.run() {
        Ok(_) => println!("Opa"),
        Err(err) => panic!("{err}"),
    }
}
