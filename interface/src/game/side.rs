use std::fmt::{Debug, Display};

pub trait Side: Sized + Debug + Display + Debug + Eq + PartialEq {
    const SIDES: [Self; 2];
    const WHITE: Self;
    const BLACK: Self;
    fn switch_sides(self) -> Self;
    fn is_white(self) -> bool;
    fn is_black(self) -> bool;
}