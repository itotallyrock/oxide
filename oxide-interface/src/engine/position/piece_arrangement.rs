use interface::game::{SidedPiece, Piece, Square, BoardMask, Side, PieceArrangement};
use crate::game::{OxideBitboard, OxidePiece, OxideSidedPiece, OxideSquare, OxideSide};
use crate::engine::zobrist::piece_key;
use std::hash::{Hasher, Hash};
use crate::engine::OxidePosition;


#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OxidePieceArrangement {
    pawns: OxideBitboard,
    queens: OxideBitboard,
    bishops: OxideBitboard,
    kings: OxideBitboard,
    rooks: OxideBitboard,
    knights: OxideBitboard,
    white: OxideBitboard,
    black: OxideBitboard,
}

impl Hash for OxidePieceArrangement {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let piece_square_iter = self.queens.into_iter().map(|s| (s, OxidePiece::Queen))
            .chain(self.bishops.into_iter().map(|s| (s, OxidePiece::Bishop)))
            .chain(self.rooks.into_iter().map(|s| (s, OxidePiece::Rook)))
            .chain(self.pawns.into_iter().map(|s| (s, OxidePiece::Pawn)))
            .chain(self.kings.into_iter().map(|s| (s, OxidePiece::King)))
            .chain(self.knights.into_iter().map(|s| (s, OxidePiece::Knight)));

        for (square, piece) in piece_square_iter {
            let square_side = self.side_on_square(square).expect("Piece on square without side in hash");
            let sided_piece = <OxidePiece as Piece<OxidePosition>>::add_side(piece, square_side);
            state.write_u64(piece_key(sided_piece, square));
        }
    }
}

// TODO: Make this const
impl PieceArrangement<OxidePosition> for OxidePieceArrangement {
    type Side = OxideSide;
    type Piece = OxidePiece;
    type SidedPiece = OxideSidedPiece;
    type BoardMask = OxideBitboard;
    type Square = OxideSquare;
    const EMPTY: Self = OxidePieceArrangement {
        pawns: OxideBitboard(0),
        queens: OxideBitboard(0),
        bishops: OxideBitboard(0),
        kings: OxideBitboard(0),
        rooks: OxideBitboard(0),
        knights: OxideBitboard(0),
        white: OxideBitboard(0),
        black: OxideBitboard(0)
    };

    #[inline]
    fn piece_mask(&self, piece: OxidePiece) -> OxideBitboard {
        match piece {
            OxidePiece::Pawn => self.pawns,
            OxidePiece::Knight => self.knights,
            OxidePiece::Bishop => self.bishops,
            OxidePiece::Rook => self.rooks,
            OxidePiece::Queen => self.queens,
            OxidePiece::King => self.kings,
            OxidePiece::Empty => panic!("Cannot get piece_mask for empty piece"),
        }
    }
    #[inline]
    fn sided_piece_mask(&self, sided_piece: OxideSidedPiece) -> OxideBitboard {
        match sided_piece {
            OxideSidedPiece::WhitePawn => self.pawns & self.white,
            OxideSidedPiece::BlackPawn => self.pawns & self.black,
            OxideSidedPiece::WhiteKnight => self.knights & self.white,
            OxideSidedPiece::BlackKnight => self.knights & self.black,
            OxideSidedPiece::WhiteBishop => self.bishops & self.white,
            OxideSidedPiece::BlackBishop => self.bishops & self.black,
            OxideSidedPiece::WhiteRook => self.rooks & self.white,
            OxideSidedPiece::BlackRook => self.rooks & self.black,
            OxideSidedPiece::WhiteQueen => self.queens & self.white,
            OxideSidedPiece::BlackQueen => self.queens & self.black,
            OxideSidedPiece::WhiteKing => self.kings & self.white,
            OxideSidedPiece::BlackKing => self.kings & self.black,
            OxideSidedPiece::Empty => panic!("Cannot get empty piece mask"),
        }
    }
    #[inline]
    fn occupied(&self) -> OxideBitboard {
        self.white | self.black
    }
    #[inline]
    fn empty(&self) -> OxideBitboard {
        !self.occupied()
    }
    #[inline]
    fn piece_mask_for_side(&self, side: OxideSide) -> OxideBitboard {
        if side.is_white() {
            self.white
        } else {
            self.black
        }
    }
    #[inline]
    fn piece_on_square(&self, square: OxideSquare) -> OxidePiece {
        let mask = square.to_mask();
        if self.pawns & mask != OxideBitboard::EMPTY {
            OxidePiece::Pawn
        } else if self.knights & mask != OxideBitboard::EMPTY {
            OxidePiece::Knight
        } else if self.bishops & mask != OxideBitboard::EMPTY {
            OxidePiece::Bishop
        } else if self.rooks & mask != OxideBitboard::EMPTY {
            OxidePiece::Rook
        } else if self.kings & mask != OxideBitboard::EMPTY {
            OxidePiece::King
        } else if self.queens & mask != OxideBitboard::EMPTY {
            OxidePiece::Queen
        } else {
            OxidePiece::Empty
        }
    }
    #[inline]
    fn side_on_square(&self, square: OxideSquare) -> Option<OxideSide> {
        let mask = square.to_mask();

        if self.white & mask != OxideBitboard::EMPTY {
            Some(OxideSide::WHITE)
        } else if self.black & mask != OxideBitboard::EMPTY {
            Some(OxideSide::BLACK)
        } else {
            None
        }
    }
    #[inline]
    fn king_square(&self, side: OxideSide) -> OxideSquare {
        (self.kings & self.piece_mask_for_side(side)).into_iter().next().expect("Attempting to get king square on board without king")
    }
    #[inline]
    fn add_piece(&mut self, sided_piece: OxideSidedPiece, to_square: OxideSquare) {
        let piece = <OxideSidedPiece as SidedPiece<OxidePosition>>::unsided_piece(&sided_piece);
        let piece_side = <OxideSidedPiece as SidedPiece<OxidePosition>>::side(&sided_piece);
        let mask = to_square.to_mask();
        debug_assert_eq!(self.piece_on_square(to_square), OxidePiece::Empty, "Attempting to add piece to non-empty square");

        match piece {
            OxidePiece::Pawn => self.pawns |= mask,
            OxidePiece::Knight => self.knights |= mask,
            OxidePiece::Bishop => self.bishops |= mask,
            OxidePiece::Rook => self.rooks |= mask,
            OxidePiece::Queen => self.queens |= mask,
            OxidePiece::King => self.kings |= mask,
            OxidePiece::Empty => panic!("Cannot add empty piece to board, use remove_piece"),
        };
        if piece_side.is_white() {
            self.white |= mask;
        } else {
            self.black |= mask;
        }
    }
    #[inline]
    fn remove_piece(&mut self, sided_piece: OxideSidedPiece, from_square: OxideSquare) {
        let piece = <OxideSidedPiece as SidedPiece<OxidePosition>>::unsided_piece(&sided_piece);
        let piece_side = <OxideSidedPiece as SidedPiece<OxidePosition>>::side(&sided_piece);
        let mask = !from_square.to_mask();
        debug_assert_eq!(self.piece_on_square(from_square), piece, "Attempting to move different piece than specified");

        match piece {
            OxidePiece::Pawn => self.pawns &= mask,
            OxidePiece::Knight => self.knights &= mask,
            OxidePiece::Bishop => self.bishops &= mask,
            OxidePiece::Rook => self.rooks &= mask,
            OxidePiece::Queen => self.queens &= mask,
            OxidePiece::King => panic!("Cannot remove king from board"),
            OxidePiece::Empty => panic!("Cannot remove empty piece to board, use add_piece"),
        };
        if piece_side.is_white() {
            self.white &= mask;
        } else {
            self.black &= mask;
        }
    }
    #[inline]
    fn move_piece(&mut self, sided_piece: OxideSidedPiece, to_square: OxideSquare, from_square: OxideSquare) {
        let piece = <OxideSidedPiece as SidedPiece<OxidePosition>>::unsided_piece(&sided_piece);
        let piece_side = <OxideSidedPiece as SidedPiece<OxidePosition>>::side(&sided_piece);
        let mask = from_square.to_mask() | to_square.to_mask();
        debug_assert_eq!(self.piece_on_square(from_square), piece, "Attempting to move different piece than specified");
        debug_assert_eq!(self.piece_on_square(to_square), OxidePiece::Empty, "Attempting to move to a non-empty square");

        match piece {
            OxidePiece::Pawn => self.pawns ^= mask,
            OxidePiece::Knight => self.knights ^= mask,
            OxidePiece::Bishop => self.bishops ^= mask,
            OxidePiece::Rook => self.rooks ^= mask,
            OxidePiece::Queen => self.queens ^= mask,
            OxidePiece::King => self.kings ^= mask,
            OxidePiece::Empty => panic!("Cannot move empty piece on board"),
        };
        if piece_side.is_white() {
            self.white ^= mask;
        } else {
            self.black ^= mask;
        }
    }
}