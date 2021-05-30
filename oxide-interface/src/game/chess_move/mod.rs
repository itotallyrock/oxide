// mod test;

mod simple_move;
mod move_type;

pub use simple_move::OxideSimpleMove;

use interface::game::{SimpleChessMove, ChessMove};
use std::fmt::{Display, Formatter, Result as FormatResult};
use crate::game::{OxideSquare, OxidePiece};
use crate::game::square::OxideSquare::{E1, G1, E8, G8, C1, C8};
use crate::game::chess_move::move_type::OxideMoveType;
use crate::engine::OxidePosition;


#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct OxideMove {
    simple_move: OxideSimpleMove,
    move_type: OxideMoveType,
}

impl Display for OxideMove {
    fn fmt(&self, f: &mut Formatter<'_>) -> FormatResult {
        let OxideSimpleMove { from, to } = self.simple_move;
        match self.move_type {
            OxideMoveType::Quiet | OxideMoveType::DoublePawnPush => write!(f, "{}{}", from, to),
            OxideMoveType::KingSideCastle | OxideMoveType::QueenSideCastle => write!(f, "{}{}", from, to),
            OxideMoveType::Capture | OxideMoveType::EnPassantCapture => write!(f, "{}x{}", from, to),
            OxideMoveType::KnightPromotion => write!(f, "{}{}n", from, to),
            OxideMoveType::BishopPromotion => write!(f, "{}{}b", from, to),
            OxideMoveType::RookPromotion => write!(f, "{}{}r", from, to),
            OxideMoveType::QueenPromotion => write!(f, "{}{}q", from, to),
            OxideMoveType::KnightPromotingCapture => write!(f, "{}x{}n", from, to),
            OxideMoveType::BishopPromotingCapture => write!(f, "{}x{}b", from, to),
            OxideMoveType::RookPromotingCapture => write!(f, "{}x{}r", from, to),
            OxideMoveType::QueenPromotingCapture => write!(f, "{}x{}q", from, to),
        }
    }
}

impl SimpleChessMove<OxidePosition> for OxideMove {
    #[inline]
    fn new(from: OxideSquare, to: OxideSquare) -> Self {
        Self {
            simple_move: OxideSimpleMove { from, to },
            move_type: OxideMoveType::Quiet,
        }
    }

    #[inline]
    fn from(&self) -> OxideSquare {
        self.simple_move.from
    }

    #[inline]
    fn to(&self) -> OxideSquare {
        self.simple_move.to
    }
}

impl ChessMove<OxidePosition> for OxideMove {
    type SimpleChessMove = OxideSimpleMove;
    const WHITE_KING_CASTLE: Self = Self {
        simple_move: OxideSimpleMove { from: E1, to: G1 },
        move_type: OxideMoveType::KingSideCastle
    };
    const WHITE_QUEEN_CASTLE: Self = Self {
        simple_move: OxideSimpleMove { from: E1, to: C1 },
        move_type: OxideMoveType::QueenSideCastle
    };
    const BLACK_KING_CASTLE: Self = Self {
        simple_move: OxideSimpleMove { from: E8, to: G8 },
        move_type: OxideMoveType::KingSideCastle
    };
    const BLACK_QUEEN_CASTLE: Self = Self {
        simple_move: OxideSimpleMove { from: E8, to: C8 },
        move_type: OxideMoveType::QueenSideCastle
    };

    fn from_simple_move(simple_move: OxideSimpleMove) -> Self {
        Self {
            simple_move,
            move_type: OxideMoveType::Quiet,
        }
    }

    fn simple_move(&self) -> OxideSimpleMove {
        self.simple_move
    }
    #[inline]
    fn promotion(&self) -> OxidePiece {
        match self.move_type {
            OxideMoveType::Quiet | OxideMoveType::DoublePawnPush | OxideMoveType::KingSideCastle | OxideMoveType::QueenSideCastle | OxideMoveType::Capture | OxideMoveType::EnPassantCapture => OxidePiece::Empty,
            OxideMoveType::QueenPromotion | OxideMoveType::QueenPromotingCapture => OxidePiece::Queen,
            OxideMoveType::KnightPromotion | OxideMoveType::KnightPromotingCapture => OxidePiece::Knight,
            OxideMoveType::RookPromotion | OxideMoveType::RookPromotingCapture => OxidePiece::Rook,
            OxideMoveType::BishopPromotion | OxideMoveType::BishopPromotingCapture => OxidePiece::Bishop,
        }
    }
    #[inline]
    fn is_quiet(&self) -> bool {
        self.move_type == OxideMoveType::Quiet
    }
    #[inline]
    fn is_double_pawn_push(&self) -> bool {
        self.move_type == OxideMoveType::DoublePawnPush
    }

    fn is_promotion(&self) -> bool {
        self.move_type as u8 & OxideMoveType::KnightPromotion as u8 != 0
    }

    fn is_capture(&self) -> bool {
        self.move_type as u8 & OxideMoveType::Capture as u8 != 0
    }

    fn is_king_castle(&self) -> bool {
        self.move_type == OxideMoveType::KingSideCastle
    }

    fn is_queen_castle(&self) -> bool {
        self.move_type == OxideMoveType::QueenSideCastle
    }

    fn is_en_passant_capture(&self) -> bool {
        self.move_type == OxideMoveType::EnPassantCapture
    }

    fn set_capture(&mut self) {
        self.move_type = match self.move_type {
            OxideMoveType::Quiet | OxideMoveType::Capture => OxideMoveType::Capture,
            OxideMoveType::DoublePawnPush   => panic!("cannot add capture to double pawn push"),
            OxideMoveType::KingSideCastle   => panic!("cannot add capture to king side castle"),
            OxideMoveType::QueenSideCastle  => panic!("cannot add capture to queen side castle"),
            OxideMoveType::EnPassantCapture => OxideMoveType::EnPassantCapture,
            OxideMoveType::KnightPromotion  | OxideMoveType::KnightPromotingCapture => OxideMoveType::KnightPromotingCapture,
            OxideMoveType::BishopPromotion  | OxideMoveType::BishopPromotingCapture => OxideMoveType::BishopPromotingCapture,
            OxideMoveType::RookPromotion    | OxideMoveType::RookPromotingCapture   => OxideMoveType::RookPromotingCapture,
            OxideMoveType::QueenPromotion   | OxideMoveType::QueenPromotingCapture  => OxideMoveType::QueenPromotingCapture,
        };
    }

    fn set_promotion(&mut self, promotion: OxidePiece) {
        let is_capture = self.is_capture();
        self.move_type = match promotion {
            OxidePiece::Bishop   if !is_capture  => OxideMoveType::BishopPromotion,
            OxidePiece::Rook     if !is_capture  => OxideMoveType::RookPromotion,
            OxidePiece::Knight   if !is_capture  => OxideMoveType::KnightPromotion,
            OxidePiece::Queen    if !is_capture  => OxideMoveType::QueenPromotion,
            OxidePiece::Bishop   if is_capture   => OxideMoveType::BishopPromotingCapture,
            OxidePiece::Rook     if is_capture   => OxideMoveType::RookPromotingCapture,
            OxidePiece::Knight   if is_capture   => OxideMoveType::KnightPromotingCapture,
            OxidePiece::Queen    if is_capture   => OxideMoveType::QueenPromotingCapture,
            _ => panic!("invalid promotion piece {}", promotion),
        };
    }
}

