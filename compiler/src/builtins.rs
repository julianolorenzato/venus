#[derive(PartialEq, Eq, Debug)]
pub(crate) enum Builtin {
    Define,
    DefinePriv,
    Pipeline,
    Import,
}
