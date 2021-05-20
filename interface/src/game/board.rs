use crate::game::{Position, Square, BoardMask, SidedPiece, Piece, Side, CastleRights};
use crate::types::PlyCount;
use crate::game::chess_move::ChessMove;

/// Board state that can't be maintained across moves (required to undo a move)
pub trait IdempotentBoardState<SideType, PieceType, SidedPieceType, BitboardType, SquareType, CastleType, MoveType>: Sized
    where SideType: Side,
          PieceType: Piece<SideType>,
          SidedPieceType: SidedPiece<SideType>,
          BitboardType: BoardMask,
          SquareType: Square<BitboardType, 64>,
          CastleType: CastleRights<SideType>,
          MoveType: ChessMove<SideType, PieceType, BitboardType, SquareType>,
{
    fn castle_rights(&self) -> CastleType;
    fn en_passant_square(&self) -> Option<SquareType>;
    fn captured_piece(&self) -> PieceType;
    fn halfmove_clock(&self) -> PlyCount;
}

pub trait Board<SideType, PieceType, SidedPieceType, BitboardType, SquareType, CastleType, MoveType, BoardState>: Position<SideType, PieceType, SidedPieceType, BitboardType, SquareType, CastleType>
    where SideType: Side,
          PieceType: Piece<SideType>,
          SidedPieceType: SidedPiece<SideType>,
          BitboardType: BoardMask,
          SquareType: Square<BitboardType, 64>,
          CastleType: CastleRights<SideType>,
          MoveType: ChessMove<SideType, PieceType, BitboardType, SquareType>,
          BoardState: IdempotentBoardState<SideType, PieceType, SidedPieceType, BitboardType, SquareType, CastleType, MoveType>,
{
    /// Mask of pieces attacking the side to move's king square
    fn checking_piece_mask(&self) -> BitboardType;
    /// Mask for all squares a given piece for the side to move can give check from
    fn piece_check_mask(&self, piece: PieceType) -> BitboardType;
    /// Make a move on the board and return the required state to undo
    fn make_move(&mut self, chess_move: MoveType) -> BoardState; // TODO: Use Result here
    /// Undo a previously made move on the board given a previous state
    fn undo_move(&mut self, chess_move: MoveType, previous_state: BoardState); // TODO: Use Result here
}