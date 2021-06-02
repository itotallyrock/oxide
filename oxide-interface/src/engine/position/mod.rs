mod piece_arrangement;


pub use piece_arrangement::OxidePieceArrangement;
use crate::engine::zobrist::{OxideZobristHasher, piece_key, castle_key, en_passant_key, SIDE_KEY, BASE_KEY};
use crate::game::{OxideBitboard, OxideSide, OxidePiece, OxideSidedPiece, OxideSquare, OxideCastleRights, OxideSquare::A8};
use interface::game::{Position, PieceArrangement, Square, Side, CastleRights, Piece};
use interface::types::PlyCount;
use std::convert::TryFrom;
use std::fmt::{Debug, Formatter, Result as FormatResult};
use std::hash::{Hasher, Hash};

#[derive(Copy, Clone)]
pub struct OxidePosition {
    arrangement: OxidePieceArrangement,
    side: OxideSide,
    zobrist_hasher: OxideZobristHasher,
    castle_rights: OxideCastleRights,
    en_passant_square: Option<OxideSquare>,
    halfmove_clock: u8,
    halfmove_count: PlyCount,
}

impl Debug for OxidePosition {
    fn fmt(&self, f: &mut Formatter<'_>) -> FormatResult {
        let mut ranks: [String; 8] = [
            String::with_capacity(8),
            String::with_capacity(8),
            String::with_capacity(8),
            String::with_capacity(8),
            String::with_capacity(8),
            String::with_capacity(8),
            String::with_capacity(8),
            String::with_capacity(8),
        ];
        for &square in &OxideSquare::SQUARES {
            let piece = self.piece_on_square(square);
            let side = self.side_on_square(square);
            let sided_piece: OxideSidedPiece = <OxidePiece as Piece<OxidePosition>>::add_side(piece, side.unwrap_or(OxideSide::WHITE));
            ranks[OxideSquare::y_offset(&square) as usize].push(sided_piece.into());
        }
        writeln!(f, "  ABCDEFGH")?;
        writeln!(f, "8|{}|8     En-Passant: {}", ranks[7], self.en_passant_square().map(|s| format!("{:?}", s)).unwrap_or("-".into()))?;
        writeln!(f, "7|{}|7  Castle-Rights: {}", ranks[6], self.castle_rights())?;
        writeln!(f, "6|{}|6   Side-to-Move: {}", ranks[5], self.side_to_move())?;
        writeln!(f, "5|{}|5 Halfmove-Clock: {}", ranks[4], self.halfmove_clock())?;
        writeln!(f, "4|{}|4    Zobrist-Key: {}", ranks[3], self.zobrist_hasher.finish())?;
        writeln!(f, "3|{}|3", ranks[2])?;
        writeln!(f, "2|{}|2", ranks[1])?;
        writeln!(f, "1|{}|1", ranks[0])?;
        writeln!(f, "  ABCDEFGH")
    }
}

impl Hash for OxidePosition {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Hash piece arrangement
        self.arrangement.hash(state);
        // Hash side to move
        if self.side.is_black() {
            state.write_u64(SIDE_KEY)
        }
        // Hash castle rights
        state.write_u64(castle_key(self.castle_rights));
        // Hash en-passant
        if let Some(en_passant_square) = self.en_passant_square {
            state.write_u64(en_passant_key(en_passant_square));
        }
    }
}

impl PieceArrangement<OxidePosition> for OxidePosition {
    type Side = OxideSide;
    type Piece = OxidePiece;
    type SidedPiece = OxideSidedPiece;
    type BoardMask = OxideBitboard;
    type Square = OxideSquare;
    const EMPTY: Self = Self {
        arrangement: OxidePieceArrangement::EMPTY,
        side: OxideSide::White,
        zobrist_hasher: OxideZobristHasher(BASE_KEY),
        castle_rights: CastleRights::NONE,
        en_passant_square: None,
        halfmove_clock: 0,
        halfmove_count: 0,
    };

    #[inline]
    fn piece_mask(&self, piece: OxidePiece) -> OxideBitboard {
        self.arrangement.piece_mask(piece)
    }
    #[inline]
    fn sided_piece_mask(&self, sided_piece: OxideSidedPiece) -> OxideBitboard {
        self.arrangement.sided_piece_mask(sided_piece)
    }
    #[inline]
    fn occupied(&self) -> OxideBitboard {
        self.arrangement.occupied()
    }
    #[inline]
    fn empty(&self) -> OxideBitboard {
        self.arrangement.empty()
    }
    #[inline]
    fn mask_for_side(&self, side: OxideSide) -> OxideBitboard {
        self.arrangement.mask_for_side(side)
    }
    #[inline]
    fn piece_on_square(&self, square: OxideSquare) -> OxidePiece {
        self.arrangement.piece_on_square(square)
    }
    #[inline]
    fn side_on_square(&self, square: OxideSquare) -> Option<OxideSide> {
        self.arrangement.side_on_square(square)
    }
    #[inline]
    fn king_square(&self, side: OxideSide) -> OxideSquare {
        self.arrangement.king_square(side)
    }
    #[inline]
    fn add_piece(&mut self, piece: OxideSidedPiece, to_square: OxideSquare) {
        self.arrangement.add_piece(piece, to_square);
        self.zobrist_hasher.write_u64(piece_key(piece, to_square));
    }
    #[inline]
    fn remove_piece(&mut self, piece: OxideSidedPiece, from_square: OxideSquare) {
        self.arrangement.remove_piece(piece, from_square);
        self.zobrist_hasher.write_u64(piece_key(piece, from_square));
    }
    #[inline]
    fn move_piece(&mut self, piece: OxideSidedPiece, from_square: OxideSquare, to_square: OxideSquare) {
        self.arrangement.move_piece(piece, from_square, to_square);
        self.zobrist_hasher.write_u64(piece_key(piece, from_square));
        self.zobrist_hasher.write_u64(piece_key(piece, to_square));
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum OxideFenParseError {
    InvalidFileChar,
    InvalidRankChar,
    HalfMovesParseError,
    FullMovesParseError,
    InvalidPieceChar,
    InvalidBoardOffset,
    InvalidEnPassantSquare,
    CastleParseError,
    InvalidSideChar,
    EmptyFen,
}

impl Position for OxidePosition {
    type CastleRights = OxideCastleRights;
    type FenParseError = OxideFenParseError;
    fn from_fen(fen: &str) -> Result<Self, Self::FenParseError> {
        let mut position = OxidePosition::EMPTY;
        let mut offset = A8.offset();
        let mut chunks = fen.split_ascii_whitespace();

        let board_str = chunks.next().ok_or(OxideFenParseError::EmptyFen)?;
        for row in board_str.split('/').take(8) {
            for c in row.chars() {
                if let Some(displacement) = c.to_digit(10) {
                    offset += displacement as u8;
                } else {
                    let sided_piece = OxideSidedPiece::from(c);
                    if sided_piece == OxideSidedPiece::Empty {
                        return Err(OxideFenParseError::InvalidPieceChar);
                    }
                    position.add_piece(sided_piece, Square::from_offset(offset).ok_or(OxideFenParseError::InvalidBoardOffset)?);
                    offset += 1;
                }
            }
            offset = offset.saturating_sub(16);
        }

        let side_str = chunks.next().ok_or(OxideFenParseError::InvalidSideChar)?;
        let side_char = side_str.chars().next().ok_or(OxideFenParseError::InvalidSideChar)?.to_ascii_lowercase();
        if side_char == 'b' {
            position.switch_sides()
        }

        let castle_str = chunks.next().ok_or(OxideFenParseError::CastleParseError)?;
        position.set_castle_rights(OxideCastleRights::try_from(castle_str)?);

        let en_passant_str = chunks.next().ok_or(OxideFenParseError::InvalidEnPassantSquare)?;
        if en_passant_str != "-" {
            position.set_en_passant(OxideSquare::try_from(en_passant_str)?);
        } else {
            position.clear_en_passant();
        }

        if let Some(halfmove_clock_str) = chunks.next() {
            let half_moves = halfmove_clock_str.parse::<u8>().ok().ok_or(OxideFenParseError::HalfMovesParseError)?;
            position.halfmove_clock = half_moves;

            if let Some(full_move_count_str) = chunks.next() {
                let full_moves = full_move_count_str.parse::<PlyCount>().ok().ok_or(OxideFenParseError::FullMovesParseError)?;
                position.halfmove_count = (full_moves - 1) * 2 + if position.side.is_black() { 1 } else { 0 };
            }
        }

        Ok(position)
    }

    fn to_fen(&self) -> String {
        // 84 is longest possible FEN for standard chess
        let mut builder = String::with_capacity(84);

        let mut squares_mapped: [[OxideSidedPiece; 8]; 8] = [[OxideSidedPiece::Empty; 8]; 8];
        for &square in &OxideSquare::SQUARES {
            let rank = square.y_offset();
            let file = square.x_offset();
            let side = self.side_on_square(square).unwrap_or(OxideSide::White);
            let piece = self.piece_on_square(square);
            let sided_piece = <OxidePiece as Piece<OxidePosition>>::add_side(piece, side);

            squares_mapped[rank as usize][file as usize] = sided_piece;
        }

        for rank in (0..8).rev() {
            let mut current_blanks = 0;
            for file in 0..8 {
                let piece = *&squares_mapped[rank][file];
                if piece == OxideSidedPiece::Empty {
                    if file == 7 {
                        builder.push(char::from_digit(current_blanks + 1, 10).unwrap());
                        break;
                    }
                    current_blanks += 1;
                } else {
                    if current_blanks > 0 {
                        builder.push(char::from_digit(current_blanks, 10).unwrap());
                        current_blanks = 0;
                    }
                    builder.push(piece.into());
                }
            }
            if rank > 0 {
                builder.push('/');
            }
        }

        builder.push(' ');
        builder.push_str(self.side_to_move().to_string().as_str());
        builder.push(' ');
        builder.push_str(self.castle_rights().to_string().as_str());
        builder.push(' ');
        if let Some(en_passant_square) = self.en_passant_square() {
            builder.push_str(en_passant_square.to_string().as_str());
        } else {
            builder.push('-');
        }
        builder.push(' ');
        builder.push_str(self.halfmove_clock().to_string().as_str());
        builder.push(' ');
        // Convert halfmove to fullmove by dividing by 2, add 1 since we are 0 indexed
        builder.push_str((self.fullmove_count() / 2 + 1).to_string().as_str());

        builder
    }
    #[inline]
    fn side_to_move(&self) -> OxideSide {
        self.side
    }
    #[inline]
    fn castle_rights(&self) -> OxideCastleRights {
        self.castle_rights
    }
    #[inline]
    fn en_passant_square(&self) -> Option<OxideSquare> {
        self.en_passant_square
    }
    #[inline]
    fn halfmove_clock(&self) -> PlyCount {
        self.halfmove_clock as PlyCount
    }
    #[inline]
    fn fullmove_count(&self) -> u16 {
        self.halfmove_count / 2 + 1
    }

    #[inline]
    fn set_castle_rights(&mut self, castle_rights: OxideCastleRights) {
        self.remove_castle_rights(OxideCastleRights::All);
        self.add_castle_rights(castle_rights);
    }
    #[inline]
    fn add_castle_rights(&mut self, castle_rights: OxideCastleRights) {
        let added_rights = !self.castle_rights & castle_rights;
        self.castle_rights.insert(added_rights);
        self.zobrist_hasher.write_u64(castle_key(added_rights));
    }
    #[inline]
    fn remove_castle_rights(&mut self, castle_rights: OxideCastleRights) {
        let removed_rights = !self.castle_rights & castle_rights;
        self.castle_rights.remove(removed_rights);
        self.zobrist_hasher.write_u64(castle_key(removed_rights));
    }
    #[inline]
    fn set_en_passant(&mut self, en_passant_square: OxideSquare) {
        self.clear_en_passant();
        self.en_passant_square = Some(en_passant_square);
        self.zobrist_hasher.write_u64(en_passant_key(en_passant_square))
    }
    #[inline]
    fn clear_en_passant(&mut self) {
        if let Some(en_passant_square) = self.en_passant_square {
            self.en_passant_square = None;
            self.zobrist_hasher.write_u64(en_passant_key(en_passant_square));
        }
    }
    #[inline]
    fn switch_sides(&mut self) {
        self.side = self.side.opposite_side();
        self.zobrist_hasher.write_u64(SIDE_KEY);
    }
    #[inline]
    fn reset_halfmove_clock(&mut self) {
        self.halfmove_clock = 0;
    }
    #[inline]
    fn increment_halfmove_clock(&mut self) {
        self.halfmove_clock += 1;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn from_default_fen_works() {
        let position = OxidePosition::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").expect("Failed to parse valid FEN");
        assert!(position.side_to_move().is_white());
        assert_eq!(position.castle_rights(), OxideCastleRights::All);
        assert_eq!(position.halfmove_clock(), 0);
        assert_eq!(position.fullmove_count(), 1);
        assert_eq!(position.mask_for_side(OxideSide::White), OxideBitboard(0xFFFFu64));
        assert_eq!(position.mask_for_side(OxideSide::Black), OxideBitboard(0xFFFF000000000000u64));
        assert_eq!(position.sided_piece_mask(OxideSidedPiece::WhitePawn), OxideBitboard(0xFF00u64));
        assert_eq!(position.sided_piece_mask(OxideSidedPiece::BlackPawn), OxideBitboard(0xFF000000000000u64));
        assert_eq!(position.sided_piece_mask(OxideSidedPiece::WhiteRook), OxideBitboard(0x81u64));
        assert_eq!(position.sided_piece_mask(OxideSidedPiece::BlackRook), OxideBitboard(0x8100000000000000u64));
        assert_eq!(position.sided_piece_mask(OxideSidedPiece::WhiteKnight), OxideBitboard(0x42u64));
        assert_eq!(position.sided_piece_mask(OxideSidedPiece::BlackKnight), OxideBitboard(0x4200000000000000u64));
        assert_eq!(position.sided_piece_mask(OxideSidedPiece::WhiteBishop), OxideBitboard(0x24u64));
        assert_eq!(position.sided_piece_mask(OxideSidedPiece::BlackBishop), OxideBitboard(0x2400000000000000u64));
        assert_eq!(position.sided_piece_mask(OxideSidedPiece::WhiteQueen), OxideBitboard(0x8u64));
        assert_eq!(position.sided_piece_mask(OxideSidedPiece::BlackQueen), OxideBitboard(0x800000000000000u64));
        assert_eq!(position.sided_piece_mask(OxideSidedPiece::WhiteKing), OxideBitboard(0x10u64));
        assert_eq!(position.sided_piece_mask(OxideSidedPiece::BlackKing), OxideBitboard(0x1000000000000000u64));
    }
}