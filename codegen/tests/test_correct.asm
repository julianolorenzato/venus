
MACRO GN &A &B
    MACRO BA &X &Y &A
        ADD &X
        ADD &Y
        ADD &A
        ADD &B
    MEND

    MACRO BO &X &Y &B
        SUB &X
        SUB &Y
        SUB &B
        SUB &A
    MEND

    BA &B &A 5
MEND

GN -5 8
BO -7 8 4

MULT 9 9