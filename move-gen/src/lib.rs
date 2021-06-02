
use interface::game::{ChessMove, SimpleChessMove, Position, BoardMask, Piece, Side};
use smallvec::SmallVec;
use interface::engine::{Board, CachedBoardState};

// TODO: Tune this value (be just above average for number of moves so that most move generation calls don't need any reallocation on the heap)
const BASE_MOVES_CAPACITY: usize = 50;

// Generate moves for a given non-pawn/non-king piece
fn generate_moves<P: Position, B: Board<P>, const N: usize>(board: &B, move_list: &mut SmallVec<[B::Move; N]>, piece: P::Piece, side: P::Side, target_mask: P::BoardMask, checks_only: bool) {
    debug_assert_ne!(piece, P::Piece::KING, "King moves aren't supported in generate_moves they're added after depending on checks");
    debug_assert_ne!(piece, P::Piece::PAWN, "Pawn moves aren't supported in generate_moves use generate_pawns");

    let piece_mask = board.position().piece_mask(piece);

    for from_square in piece_mask {

    }
}

#[inline(always)]
fn pseudo_legal_filter<P: Position, B: Board<P>>(board: &B, chess_move: &B::Move) -> bool {
    let side_to_move = board.position().side_to_move();
    let is_pinned = board.state().blocking_mask(side_to_move) & board.position().mask_for_side(side_to_move) != P::BoardMask::EMPTY;

    (!is_pinned && chess_move.from() != board.position().king_square(side_to_move) && chess_move.is_en_passant_capture()) || board.is_legal(chess_move)
}

#[inline]
pub fn legal_moves<P: Position, B: Board<P>>(board: &B) -> impl Iterator<Item=B::Move> {
    let mut list = SmallVec::<[B::Move; BASE_MOVES_CAPACITY]>::new();

    if board.in_check() {
        list.extend(evasion_moves::<P, B>(board));
    } else {
        list.extend(non_evasion_moves::<P, B>(board));
    };

    list.retain(|m| pseudo_legal_filter::<P, B>(board, m));

    list.into_iter()
}

#[inline]
pub fn capture_moves<P: Position, B: Board<P>>(board: &B) -> impl Iterator<Item=B::Move> {
    let mut list = SmallVec::<[B::Move; BASE_MOVES_CAPACITY]>::new();
    let side_to_move = board.position().side_to_move();
    let opposite_side = side_to_move.opposite_side();
    let target_mask = board.position().mask_for_side(opposite_side);

    todo!();

    list.into_iter()
}

#[inline]
pub fn quiet_moves<P: Position, B: Board<P>>(board: &B) -> impl Iterator<Item=B::Move> {
    let mut list = SmallVec::<[B::Move; BASE_MOVES_CAPACITY]>::new();

    todo!();

    list.into_iter()
}

#[inline]
pub fn quiet_check_moves<P: Position, B: Board<P>>(board: &B) -> impl Iterator<Item=B::Move> {
    let mut list = SmallVec::<[B::Move; BASE_MOVES_CAPACITY]>::new();

    todo!();

    list.into_iter()
}

/// Generate evasion moves (getting out of check)
#[inline]
pub fn evasion_moves<P: Position, B: Board<P>>(board: &B) -> impl Iterator<Item=B::Move> {
    debug_assert!(board.in_check(), "Attempting to get evasion moves for a position not in check");
    let mut list = SmallVec::<[B::Move; BASE_MOVES_CAPACITY]>::new();

    todo!();

    list.into_iter()
}

#[inline]
pub fn non_evasion_moves<P: Position, B: Board<P>>(board: &B) -> impl Iterator<Item=B::Move> {
    debug_assert!(!board.in_check(), "Attempting to get non-evasion moves for a position while in check");
    let mut list = SmallVec::<[B::Move; BASE_MOVES_CAPACITY]>::new();

    todo!();

    list.into_iter()
}

#[cfg(test)]
mod tests {
    use oxide_interface::engine::{OxidePosition, OxideBoard};
    use oxide_interface::game::{OxideMove, OxideSimpleMove, OxideSquare::*};
    use crate::legal_moves;
    use interface::game::{Position, SimpleChessMove, ChessMove};

    /*
    fn test_legal_moves(board: OxideBoard, expected_moves: &[OxideMove]) {
        let move_list = legal_moves(&board).collect::<Vec<OxideMove>>();
        for legal_move in &move_list {
            assert!(expected_moves.contains(legal_move), "Legal move gen had extra move {}", legal_move);
        }
        for expected_move in expected_moves {
            assert!(move_list.contains(expected_move), "Legal move gen was missing expected move {}", expected_move);
        }
        assert_eq!(move_list.len(), expected_moves.len(), "Move gen found different number of legal moves than expected");
    }

    #[test]
    fn default_position_legal_works() {
        let position = OxidePosition::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").expect("Failed to parse test case FEN");
        test_legal_moves(position, &[
            // Pawns
            OxideMove::new(A2, A3),
            OxideMove::new_double_pawn_push(A2, A4),
            OxideMove::new(B2, B3),
            OxideMove::new_double_pawn_push(B2, B4),
            OxideMove::new(C2, C3),
            OxideMove::new_double_pawn_push(C2, C4),
            OxideMove::new(D2, D3),
            OxideMove::new_double_pawn_push(D2, D4),
            OxideMove::new(E2, E3),
            OxideMove::new_double_pawn_push(E2, E4),
            OxideMove::new(F2, F3),
            OxideMove::new_double_pawn_push(F2, F4),
            OxideMove::new(G2, G3),
            OxideMove::new_double_pawn_push(G2, G4),
            OxideMove::new(H2, H3),
            OxideMove::new_double_pawn_push(H2, H4),
            // Knights
            OxideMove::new(B1, A3),
            OxideMove::new(B1, C3),
            OxideMove::new(G1, F3),
            OxideMove::new(G1, H3),
        ]);
    }*/
}
