use crate::game::{Side, Piece, BoardMask, SidedPiece, Square, CastleRights};

pub trait PieceArrangement<SideType, PieceType, SidedPieceType, BitboardType, SquareType>
    where SideType: Side,
          PieceType: Piece<SideType>,
          SidedPieceType: SidedPiece<SideType>,
          BitboardType: BoardMask,
          SquareType: Square<BitboardType, 64>
{
    fn piece_mask(&self, piece: PieceType) -> BitboardType;
    fn sided_piece_mask(&self, sided_piece: SidedPieceType) -> BitboardType;
    fn piece_mask_for_side(&self, side: SideType) -> BitboardType;
    fn piece_on_square(&self, square: SquareType) -> PieceType;
    fn side_on_square(&self, square: SquareType) -> Option<SideType>;
    fn king_square(&self, side: SideType) -> SquareType;
}

pub trait Position<SideType, PieceType, SidedPieceType, BitboardType, SquareType, CastleType: CastleRights<SideType>>: Sized+ Clone + PieceArrangement<SideType, PieceType, SidedPieceType, BitboardType, SquareType>
    where SideType: Side,
          PieceType: Piece<SideType>,
          SidedPieceType: SidedPiece<SideType>,
          BitboardType: BoardMask,
          SquareType: Square<BitboardType, 64>
{
    fn from_fen(fen: &str) -> Self;
    fn to_fen(&self) -> &str;
    fn side_to_move(&self) -> SideType;
    fn castle_rights(&self) -> CastleType;
    fn en_passant_square(&self) -> Option<SquareType>;
}