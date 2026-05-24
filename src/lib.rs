use int_interval::*;

mod int_co_set;

pub use int_co_set::IntCOSet;

pub type I8COSet = IntCOSet<I8CO>;
pub type I16COSet = IntCOSet<I16CO>;
pub type I32COSet = IntCOSet<I32CO>;
pub type I64COSet = IntCOSet<I64CO>;
pub type I128COSet = IntCOSet<I128CO>;
pub type IsizeCOSet = IntCOSet<IsizeCO>;

pub type U8COSet = IntCOSet<U8CO>;
pub type U16COSet = IntCOSet<U16CO>;
pub type U32COSet = IntCOSet<U32CO>;
pub type U64COSet = IntCOSet<U64CO>;
pub type U128COSet = IntCOSet<U128CO>;
pub type UsizeCOSet = IntCOSet<UsizeCO>;
