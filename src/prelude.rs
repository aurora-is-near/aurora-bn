#[cfg(not(feature = "std"))]
pub use alloc::{
    string::{String, ToString},
    vec::Vec,
};
#[cfg(not(feature = "std"))]
pub use core::{
    cmp::Ordering,
    fmt,
    fmt::Formatter,
    marker::PhantomData,
    ops::{Add, Mul, Neg, Sub},
    str::FromStr,
};
#[cfg(feature = "std")]
pub use std::{
    cmp::Ordering,
    fmt,
    fmt::Formatter,
    marker::PhantomData,
    ops::{Add, Mul, Neg, Sub},
    str::FromStr,
    string::{String, ToString},
    vec::Vec,
};
