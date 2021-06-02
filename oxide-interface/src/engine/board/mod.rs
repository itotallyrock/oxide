use crate::engine::zobrist::{OxideZobristHasher, BASE_KEY};
use interface::game::{PieceArrangement, SimpleChessMove, ChessMove, Side, BoardMask, CastleRights, Piece, SidedPiece, Square, Position};
use crate::game::{OxidePiece, OxideBitboard, OxideSquare, OxideCastleRights, OxideSide, OxideSidedPiece, OxideMove, OxideSimpleMove, OxideIllegalMoveError};
use crate::engine::position::OxidePosition;
use std::fmt::Debug;
use std::error::Error;
use crate::engine::OxideFenParseError;
use std::hash::{Hash, Hasher};
use interface::engine::{IdempotentBoardState, CachedBoardState, BoardState, Board};

#[derive(Copy, Clone, Debug)]
pub struct OxideBoardState {
    // Cached board state
    white_pinning: OxideBitboard,
    white_blocking: OxideBitboard,
    black_pinning: OxideBitboard,
    black_blocking: OxideBitboard,
    checkers: OxideBitboard,
    check_piece_masks: [OxideBitboard; 6],
    // Idempotent state
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
    // Board state
    state: OxideBoardState,
}

impl const IdempotentBoardState<OxidePosition> for OxideBoardState {
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

impl CachedBoardState<OxidePosition> for OxideBoardState {
    #[inline]
    fn pinning_mask(&self, side: OxideSide) -> OxideBitboard {
        if side.is_white() {
            self.white_pinning
        } else {
            self.black_pinning
        }
    }
    #[inline]
    fn blocking_mask(&self, side: OxideSide) -> OxideBitboard {
        if side.is_white() {
            self.white_blocking
        } else {
            self.black_blocking
        }
    }
    #[inline]
    fn checkers_mask(&self) -> OxideBitboard {
        self.checkers
    }
    #[inline]
    fn piece_check_mask(&self, piece: OxidePiece) -> OxideBitboard {
        match piece {
            OxidePiece::Pawn => self.check_piece_masks[0],
            OxidePiece::Knight => self.check_piece_masks[1],
            OxidePiece::Bishop => self.check_piece_masks[2],
            OxidePiece::Rook => self.check_piece_masks[3],
            OxidePiece::Queen => self.check_piece_masks[4],
            OxidePiece::King => self.check_piece_masks[5],
            OxidePiece::Empty => panic!("Can't get piece checks for empty piece"),
        }
    }
}

impl BoardState<OxidePosition> for OxideBoardState {
    fn new(position: &OxidePosition) -> Self {
        let state = Self {
            white_pinning: OxideBitboard::EMPTY,
            white_blocking: OxideBitboard::EMPTY,
            black_pinning: OxideBitboard::EMPTY,
            black_blocking: OxideBitboard::EMPTY,
            checkers: OxideBitboard::EMPTY,
            check_piece_masks: [OxideBitboard::EMPTY; 6],
            zobrist_hasher: OxideZobristHasher(BASE_KEY),
            castle_rights: position.castle_rights(),
            en_passant_square: position.en_passant_square(),
            captured_piece: OxidePiece::Empty,
            halfmove_clock: position.halfmove_clock() as u8
        };

        // TODO: Call something to add pins and blocks to state
        // TODO: Call something to add checkers and update check_piece_masks to state

        state
    }
}

impl Board<OxidePosition> for OxideBoard {
    type BoardState = OxideBoardState;
    type SimpleMove = OxideSimpleMove;
    type Move = OxideMove;
    type IllegalMoveError = OxideIllegalMoveError;
    type UndoMoveError = OxideIllegalMoveError;

    fn new(position: OxidePosition) -> Self {
        Self {
            state: OxideBoardState::new(&position),
            position,
        }
    }

    #[inline]
    fn state(&self) -> &Self::BoardState {
        &self.state
    }

    fn position(&self) -> &OxidePosition {
        &self.position
    }

    fn make_move(&mut self, chess_move: Self::Move) -> Result<Self::BoardState, Self::IllegalMoveError> {
        todo!()
    }

    fn make_move_unchecked(&mut self, chess_move: Self::Move) -> Self::BoardState {
        todo!()
    }

    fn undo_move(&mut self, chess_move: Self::Move, previous_state: Self::BoardState) -> Result<(), Self::UndoMoveError> {
        todo!()
    }

    fn undo_move_unchecked(&mut self, chess_move: Self::Move, previous_state: Self::BoardState) {
        todo!()
    }
    #[inline]
    fn in_check(&self) -> bool {
        self.state.checkers_mask() != OxideBitboard::EMPTY
    }

    fn is_discovery(&self, chess_move: &Self::Move) -> bool {
        todo!()
    }

    fn gives_check(&self, chess_move: &Self::Move) -> bool {
        todo!()
    }

    fn is_legal(&self, chess_move: &Self::Move) -> bool {
        // todo!()
        true
    }
}

impl Hash for OxideBoard {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.position.hash(state);
    }
}

/*
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