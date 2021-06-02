// mod test;

mod simple_move;
mod move_type;
mod test;

pub use simple_move::OxideSimpleMove;

use interface::game::{SimpleChessMove, ChessMove, Square};
use std::fmt::{Display, Formatter, Result as FormatResult};
use crate::game::{OxideSquare, OxidePiece};
use crate::game::square::OxideSquare::{E1, G1, E8, G8, C1, C8};
use crate::game::chess_move::move_type::OxideMoveType;
use crate::engine::OxidePosition;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum OxideIllegalMoveError {
    MovingPieceForWrongSide, // Moving a piece for the side that isn't currently supposed to move
    CapturingOwnPiece, // Move onto square with the same side
    NonCapturingCapture, // Piece being moved was expecting to capture but targeted an empty square
    MovingFromEmptySquare, // Piece being moved doesn't exist
    CastlingWithoutPermission, // Castle without permissions
    CastlingThroughAttack, // Castle path is attacked
    HorizontalPawnPush, // Pawn push doesn't stay in the same file
    VerticalPawnCapture, // Pawn capture doesn't go left or right a file
    InvalidPawnPush, // Pawn skipped a square forward (moved more than 2 for double jump or 1 for single jump)
    NonExistentEnPassantCapture, // Attempting to en passant capture non-existing en-passant pawn
    InvalidDiagonalMovement, // For bishops/queens when they don't stick to a valid diagonal
    InvalidCardinalMovement, // For rooks/queens when they don't stick to a valid cardinal
    InvalidKnightJump, // If a knight jump doesn't goto a valid knight square
    SelfCheck, // If a move would result in a check on yourself
    ThreeFoldRepetition, // When a move would result in a drawn game for threefold repetition
}

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
        debug_assert_ne!(from, to, "Attempting to create simple move which goes to the same square");
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

#[inline]
fn promotion_piece_to_move_type(promotion: OxidePiece, capture: bool) -> OxideMoveType {
    match promotion {
        // Non captures
        OxidePiece::Queen if !capture => OxideMoveType::QueenPromotion,
        OxidePiece::Knight if !capture => OxideMoveType::KnightPromotion,
        OxidePiece::Rook if !capture => OxideMoveType::RookPromotion,
        OxidePiece::Bishop if !capture => OxideMoveType::BishopPromotion,
        // Captures
        OxidePiece::Queen if capture => OxideMoveType::QueenPromotingCapture,
        OxidePiece::Knight if capture => OxideMoveType::KnightPromotingCapture,
        OxidePiece::Rook if capture => OxideMoveType::RookPromotingCapture,
        OxidePiece::Bishop if capture => OxideMoveType::BishopPromotingCapture,
        _ => panic!("invalid promotion piece {}", promotion),
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
    #[inline]
    fn new_double_pawn_push(from: OxideSquare, to: OxideSquare) -> Self {
        debug_assert_ne!(from, to, "Attempting to create move which goes to the same square");
        debug_assert_eq!((to.y_offset() as i8 - from.y_offset() as i8).abs(), 2, "Attempting to create double-pawn-push that doesn't move forward two squares");
        debug_assert_eq!((to.x_offset() as i8 - from.x_offset() as i8).abs(), 0, "Attempting to create double-pawn-push that moves sideways");
        Self {
            simple_move: OxideSimpleMove { from, to },
            move_type: OxideMoveType::DoublePawnPush,
        }
    }
    #[inline]
    fn new_en_passant_capture(from: OxideSquare, to: OxideSquare) -> Self {
        debug_assert_ne!(from, to, "Attempting to create move which goes to the same square");
        debug_assert_eq!((from.y_offset() as i8 - to.y_offset() as i8).abs(), 1, "Attempting to create en-passant-capture that doesnt move forward one square");
        debug_assert_eq!((from.x_offset() as i8 - to.x_offset() as i8).abs(), 1, "Attempting to create en-passant-capture that doesnt move sideways one square");
        Self {
            simple_move: OxideSimpleMove { from, to },
            move_type: OxideMoveType::EnPassantCapture,
        }
    }
    #[inline]
    fn new_capture(from: OxideSquare, to: OxideSquare) -> Self {
        debug_assert_ne!(from, to, "Attempting to create move which goes to the same square");
        Self {
            simple_move: OxideSimpleMove { from, to },
            move_type: OxideMoveType::Capture,
        }
    }
    #[inline]
    fn new_promotion(from: OxideSquare, to: OxideSquare, promotion: OxidePiece) -> Self {
        debug_assert_ne!(from, to, "Attempting to create move which goes to the same square");
        debug_assert!(to.y_offset() == 7 || to.y_offset() == 0, "Attempting to create promotion which doesn't move to last rank");
        debug_assert_eq!((to.x_offset() as i8 - from.x_offset() as i8).abs(), 0, "Attempting to create promotion push which moves sideways");

        Self {
            simple_move: OxideSimpleMove { from, to },
            move_type: promotion_piece_to_move_type(promotion, false),
        }
    }
    #[inline]
    fn new_promoting_capture(from: OxideSquare, to: OxideSquare, promotion: OxidePiece) -> Self {
        debug_assert_ne!(from, to, "Attempting to create move which goes to the same square");
        debug_assert!(to.y_offset() == 7 || to.y_offset() == 0, "Attempting to create promotion which doesn't move to last rank");
        debug_assert_eq!((to.x_offset() as i8 - from.x_offset() as i8).abs(), 1, "Attempting to create promoting capture which doesn't capture sideways");

        Self {
            simple_move: OxideSimpleMove { from, to },
            move_type: promotion_piece_to_move_type(promotion, true),
        }
    }
    #[inline]
    fn from_simple_move(simple_move: OxideSimpleMove) -> Self {
        debug_assert_ne!(simple_move.from, simple_move.to, "Attempting to create move from simple move which goes to the same square");
        Self {
            simple_move,
            move_type: OxideMoveType::Quiet,
        }
    }
    #[inline]
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
    #[inline]
    fn is_promotion(&self) -> bool {
        self.move_type as u8 & OxideMoveType::KnightPromotion as u8 != 0
    }
    #[inline]
    fn is_capture(&self) -> bool {
        self.move_type as u8 & OxideMoveType::Capture as u8 != 0
    }
    #[inline]
    fn is_king_castle(&self) -> bool {
        self.move_type == OxideMoveType::KingSideCastle
    }
    #[inline]
    fn is_queen_castle(&self) -> bool {
        self.move_type == OxideMoveType::QueenSideCastle
    }
    #[inline]
    fn is_en_passant_capture(&self) -> bool {
        self.move_type == OxideMoveType::EnPassantCapture
    }
}

