pub mod nil;
pub mod cons;
pub mod symbol;
pub mod mchar;
pub mod mint;
pub mod mfloat;

#[derive(Debug, PartialEq, Eq)]
pub enum MType {
    Nil,
    Cons,
    Symbol,
    MChar,
    MInt,
    MFloat,
}
