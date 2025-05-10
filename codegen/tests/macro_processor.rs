// use assert_cmd::{self, output::OutputOkExt, Command};

use std::env::{self, temp_dir};

use assert_cmd::cargo::CommandCargoExt;

#[test]
fn macro_processor_tests() {
    std::process::Command::new();
    use assembler::*;

    println!("{}", env!("CARGO_BIN_EXE_assembler"));

    let input = "
        MACRO GN &A &B
            MACRO BA &X &Y &A
                ADD &X
                ADD &Y
                ADD &A
            MEND

            MACRO BO &X &Y &B
                SUB &X
                SUB &Y
                SUB &B
            MEND

            BA &B &A 5
        MEND

        GN -5 8
        BO -7 8 4

        MULT 9 9";

    let expected = "
        ADD 8
        ADD -5
        ADD 5
        SUB -7
        SUB 8
        SUB 4
        MULT 9 9
    ";

    // let mut cmd = Command::cargo_bin("assembler").unwrap();

    // let out = cmd.args(&["-f", "tests/test_correct.asm"]).unwrap().stdout;

    // println!("{:?}", out)
}
