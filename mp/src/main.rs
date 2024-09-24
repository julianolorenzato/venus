use std::io::Cursor;

use mp::run;

const PROGRAM: &str = "
START TESTE
* HERE ARE THE
MACRO
SCALE &RP
    MACRO
    MULTSC &A &B &C
    LOAD &A
    MULT &B
    SHIFTR &RP
    STORE &C
    MEND

    MACRO
    DIVSC &A &B &C
    LOAD &A
    DIV &B
    SHIFTL &RP
    STORE &C
    MEND
MEND
*
MACRO
DISCR &A &B &C &D
MULTSC &A &C TEMP1
MULTSC TEMP1 @4 TEMP1
MULTSC &A &B TEMP2
SUB TEMP1
STORE &D
MEND
*
READ A 
READ B 

* random stuff...
READ C 
SCALE 3 
DISCR A B C D 
WRITE D 
STOP 
*
A SPACE
B SPACE
C SPACE
D SPACE
TEMP1 SPACE
TEMP2 SPACE
*
END
";

fn main() {
    let p = Cursor::new(PROGRAM);
    run(p)
}
