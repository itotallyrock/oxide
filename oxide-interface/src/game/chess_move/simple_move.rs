use crate::game::OxideSquare;
use std::fmt::{Display, Formatter, Result as FormatResult};
use interface::game::SimpleChessMove;
use crate::engine::OxidePosition;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct OxideSimpleMove {
    pub(super) from: OxideSquare,
    pub(super) to: OxideSquare,
}

impl Display for OxideSimpleMove {
    fn fmt(&self, f: &mut Formatter<'_>) -> FormatResult {
        write!(f, "{}{}", self.from, self.to)
    }
}

impl SimpleChessMove<OxidePosition> for OxideSimpleMove {
    #[inline]
    fn new(from: OxideSquare, to: OxideSquare) -> Self {
        Self {
            from,
            to
        }
    }

    #[inline]
    fn from(&self) -> OxideSquare {
        self.from
    }

    #[inline]
    fn to(&self) -> OxideSquare {
        self.to
    }
}