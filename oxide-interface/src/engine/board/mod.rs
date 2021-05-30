use crate::engine::zobrist::OxideZobristHasher;
use interface::game::IdempotentBoardState;
use crate::game::{OxidePiece, OxideBitboard, OxideSquare, OxideCastleRights};
use crate::engine::position::OxidePosition;
use std::fmt::Debug;

#[derive(Copy, Clone, Debug)]
pub struct OxideIdempotentBoardState {
    zobrist_hasher: OxideZobristHasher,
    castle_rights: OxideCastleRights,
    en_passant_square: Option<OxideSquare>,
    captured_piece: OxidePiece,
    halfmove_clock: u8,
}

#[derive(Copy, Clone, Debug)]
pub struct OxideBoard {
    // Positional information (anything in a FEN)
    position: OxidePosition,
    // Mask of white pinning pieces
    white_pinning: OxideBitboard,
    // Whites pieces blocking a pin on the king
    white_blocking: OxideBitboard,
    // Mask of black pinning pieces
    black_pinning: OxideBitboard,
    // Blacks pieces blocking a pin on the king
    black_blocking: OxideBitboard,
    // Mask of pieces currently giving check
    checkers: OxideBitboard,
    // Masks for each piece type that could give check to other side
    check_piece_masks: [OxideBitboard; 6],
}

impl const IdempotentBoardState<OxidePosition> for OxideIdempotentBoardState {
    type BoardHasher = OxideZobristHasher;

    #[inline]
    fn castle_rights(&self) -> OxideCastleRights {
        self.castle_rights
    }
    #[inline]
    fn en_passant_square(&self) -> Option<OxideSquare> {
        self.en_passant_square
    }
    #[inline]
    fn captured_piece(&self) -> OxidePiece {
        self.captured_piece
    }
    #[inline]
    fn halfmove_clock(&self) -> u16 {
        self.halfmove_clock as u16
    }
    #[inline]
    fn hasher(&self) -> OxideZobristHasher {
        self.zobrist_hasher
    }
}

/*
impl Hash for OxideBoard {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.position.hash(state);
    }
}

impl PieceArrangement<OxideSide, OxidePiece, OxideSidedPiece, OxideBitboard, OxideSquare> for OxideBoard {
    const EMPTY: Self = Self {
        position: OxidePosition::EMPTY,
        state: OxideIdempotentBoardState {
            zobrist_hasher: OxideZobristHasher(BASE_KEY),
            castle_rights: OxideCastleRights::None,
            en_passant_square: None,
            captured_piece: OxidePiece::Empty,
            halfmove_clock: 0
        },
        white_pinning: 0,
        white_blocking: 0,
        black_pinning: 0,
        black_blocking: 0,
        checkers: 0,
        check_piece_masks: [0; 6]
    };

    fn piece_mask(&self, piece: OxidePiece) -> OxideBitboard {
        todo!()
    }

    fn sided_piece_mask(&self, sided_piece: OxideSidedPiece) -> OxideBitboard {
        todo!()
    }

    fn occupied(&self) -> OxideBitboard {
        todo!()
    }

    fn empty(&self) -> OxideBitboard {
        todo!()
    }

    fn piece_mask_for_side(&self, side: OxideSide) -> OxideBitboard {
        todo!()
    }

    fn piece_on_square(&self, square: OxideSquare) -> OxidePiece {
        todo!()
    }

    fn side_on_square(&self, square: OxideSquare) -> Option<OxideSide> {
        todo!()
    }

    fn king_square(&self, side: OxideSide) -> OxideSquare {
        todo!()
    }

    fn add_piece(&mut self, piece: OxideSidedPiece, to_square: OxideSquare) {
        todo!()
    }

    fn remove_piece(&mut self, piece: OxideSidedPiece, from_square: OxideSquare) {
        todo!()
    }

    fn move_piece(&mut self, piece: OxideSidedPiece, to_square: OxideSquare, from_square: OxideSquare) {
        todo!()
    }
}

impl Position<OxideSide, OxidePiece, OxideSidedPiece, OxideBitboard, OxideSquare, OxideCastleRights> for OxideBoard {
    type FenParseError = OxideFenParseError;

    fn from_fen(fen: &str) -> Result<Self, Self::FenParseError> {
        let position = OxidePosition::from_fen(fen)?;
        let state = OxideIdempotentBoardState {
            zobrist_hasher: OxideZobristHasher(BASE_KEY),
            castle_rights: OxideCastleRights::None,
            en_passant_square: None,
            captured_piece: Default::default(),
            halfmove_clock: 0
        };

        // TODO: Slow full computation of all pinning/blocking/checking (instead of an incremental change)

        Ok(Self {
            position,
            state,
            white_pinning: OxideBitboard::EMPTY,
            white_blocking: OxideBitboard::EMPTY,
            black_pinning: OxideBitboard::EMPTY,
            black_blocking: OxideBitboard::EMPTY,
            checkers: OxideBitboard::EMPTY,
            check_piece_masks: [OxideBitboard::EMPTY; 6],
        })
    }

    fn to_fen(&self) -> &str {
        self.position.to_fen()
    }

    fn side_to_move(&self) -> OxideSide {
        self.position.side_to_move()
    }

    fn castle_rights(&self) -> OxideCastleRights {
        self.position.castle_rights()
    }

    fn en_passant_square(&self) -> Option<OxideSquare> {
        self.position.en_passant_square()
    }

    fn halfmove_clock(&self) -> u16 {
        self.position.halfmove_clock()
    }

    fn fullmove_count(&self) -> u16 {
        self.position.fullmove_count()
    }

    fn set_castle_rights(&mut self, castle_rights: OxideCastleRights) {
        self.position.set_castle_rights(castle_rights);
    }

    fn add_castle_rights(&mut self, castle_rights: OxideCastleRights) {
        self.position.add_castle_rights(castle_rights);
    }

    fn remove_castle_rights(&mut self, castle_rights: OxideCastleRights) {
        self.position.remove_castle_rights(castle_rights);
    }

    fn set_en_passant(&mut self, en_passant_square: OxideSquare) {
        self.position.set_en_passant(en_passant_square);
    }

    fn clear_en_passant(&mut self) {
        self.position.clear_en_passant();
    }

    fn switch_sides(&mut self) {
        self.position.switch_sides();
    }

    fn reset_halfmove_clock(&mut self) {
        self.position.reset_halfmove_clock();
    }

    fn increment_halfmove_clock(&mut self) {
        self.position.increment_halfmove_clock();
    }
}

impl Board<OxideSide, OxidePiece, OxideSide, OxideBitboard, OxideSquare, OxideCastleRights, OxideSimpleMove, OxideMove, OxideIdempotentBoardState, OxideZobristHasher> for OxideBoard {
    fn checkers_mask(&self) -> OxideBitboard {
        todo!()
    }

    fn piece_check_mask(&self, piece: OxidePiece) -> OxideBitboard {
        todo!()
    }

    fn make_move(&mut self, chess_move: OxideMove) -> OxideIdempotentBoardState {
        todo!()
    }

    fn undo_move(&mut self, chess_move: OxideMove, previous_state: OxideIdempotentBoardState) {
        todo!()
    }
}
*/