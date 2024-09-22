use assembler::Assembler;
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about="Assemble Venus mnemonic code.", long_about = None)]
struct AssemblerArgs {
    #[arg(short, long)]
    filename: String,
}

fn main() {
    let mut asmr = Assembler::new("assembler/test.asm");

    match asmr.run() {
        Ok(program) => println!("{:?}", program),
        Err(err) => panic!("{err}"),
    }
}
