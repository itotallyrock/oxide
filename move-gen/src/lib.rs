#![feature(const_generics)]

use interface::game::{ChessMove, SimpleChessMove, Position, BoardMask, Piece, Side, Square, LineMask, Shiftable, CastleRights};
use smallvec::SmallVec;
use interface::engine::{Board, CachedBoardState};
use attacks::{pseudo_attacks, pawn_pushes, pawn_east_attacks, pawn_west_attacks, king_attacks};

// TODO: Tune this value (be just above average for number of moves so that most move generation calls don't need any reallocation on the heap)
const BASE_MOVES_CAPACITY: usize = 50;

#[inline]
fn new_quiet_or_capture<P: Position, B: Board<P>>(occupied: P::BoardMask, from_square: P::Square, to_square: P::Square) -> B::Move {
    if occupied & to_square.to_mask() != P::BoardMask::EMPTY {
        B::Move::new_capture(from_square, to_square)
    } else {
        B::Move::new(from_square, to_square)
    }
}

#[inline]
fn generate_discoveries<P: Position, B: Board<P>, const N: usize>(board: &B, move_list: &mut SmallVec<[B::Move; N]>, piece: P::Piece, side_moving: P::Side) {
    let opposite_side = side_moving.opposite_side();
    let enemy_king_square = board.position().king_square(opposite_side);
    let occupied_mask = board.position().occupied();
    let friendly_mask = board.position().mask_for_side(side_moving);
    let friendly_blockers_mask = board.state().blocking_mask(opposite_side) & friendly_mask;
    let enemy_mask = !friendly_mask;

    for from_square in friendly_blockers_mask {
        let attacks_mask = pseudo_attacks::<P>(piece, from_square, occupied_mask) & enemy_mask;
        move_list.extend(attacks_mask.filter_map(|to_square| {
            if !P::BoardMask::aligned(from_square, to_square, enemy_king_square) {
                Some(new_quiet_or_capture::<P, B>(occupied_mask, from_square, to_square))
            } else {
                None
            }
        }))
    }
}

#[inline]
fn extend_attack_mask_moves<P: Position, B: Board<P>, const N: usize>(board: &B, move_list: &mut SmallVec<[B::Move; N]>, from_square: P::Square, attacks_mask: P::BoardMask) {
    debug_assert_ne!(board.position().piece_on_square(from_square), P::Piece::PAWN, "Attempting to extend attacks mask moves for a pawn, use generate_pawns");

    move_list.extend(attacks_mask.map(|to_square| {
        if board.position().occupied() & to_square.to_mask() != P::BoardMask::EMPTY {
            B::Move::new_capture(from_square, to_square)
        } else {
            B::Move::new(from_square, to_square)
        }
    }));
}

// Generate moves for a given non-pawn/non-king piece
#[inline]
fn generate_piece_moves<P: Position, B: Board<P>, const N: usize>(board: &B, move_list: &mut SmallVec<[B::Move; N]>, piece: P::Piece, side_moving: P::Side, target_mask: P::BoardMask, checks_only: bool) {
    debug_assert_ne!(piece, P::Piece::KING, "King moves aren't supported in generate_moves they're added after depending on checks");
    debug_assert_ne!(piece, P::Piece::PAWN, "Pawn moves aren't supported in generate_moves use generate_pawns");

    let occupied_mask = board.position().occupied();
    let piece_mask = board.position().sided_piece_mask(piece.add_side(side_moving));
    let piece_mask = if checks_only {
        piece_mask ^ board.position().sided_piece_mask(P::Piece::KING.add_side(side_moving))
    } else {
        piece_mask
    };

    for from_square in piece_mask {
        let attacks_mask = pseudo_attacks::<P>(piece, from_square, occupied_mask) & target_mask;
        let attacks_mask = if checks_only {
            attacks_mask & board.state().piece_check_mask(piece)
        } else {
            attacks_mask
        };

        extend_attack_mask_moves(board, move_list, from_square, attacks_mask);
    }

    if checks_only {
        generate_discoveries(board, move_list, piece, side_moving);
    }
}

#[inline]
fn reverse_pawn_shift<P: Position>(side: P::Side, to_square: P::Square, west: bool) -> P::Square {
    if side.is_white() {
        if west {
            to_square.south_east_shift()
        } else {
            to_square.south_west_shift()
        }
    } else {
        if west {
            to_square.north_east_shift()
        } else {
            to_square.north_west_shift()
        }
    }
}

#[inline]
fn extend_pawn_promotion<P: Position, B: Board<P>, const N: usize>(move_list: &mut SmallVec<[B::Move; N]>, side: P::Side, target_mask: P::BoardMask, capture: bool, west: bool) {
    move_list.reserve(2 * target_mask.count());
    for to_square in target_mask {
        if capture {
            let from_square = reverse_pawn_shift::<P>(side, to_square, west);
            move_list.push(B::Move::new_promoting_capture(from_square, to_square, P::Piece::QUEEN));
            move_list.push(B::Move::new_promoting_capture(from_square, to_square, P::Piece::KNIGHT));
        } else {
            let from_square = if side.is_white() {
                to_square.south_shift()
            } else {
                to_square.north_shift()
            };
            move_list.push(B::Move::new_promotion(from_square, to_square, P::Piece::QUEEN));
            move_list.push(B::Move::new_promotion(from_square, to_square, P::Piece::KNIGHT));
        }
    }
}

#[inline]
fn extend_pawn_pushes<P: Position, B: Board<P>, const N: usize>(move_list: &mut SmallVec<[B::Move; N]>, side: P::Side, target_mask: P::BoardMask, double_jump: bool) {
    move_list.extend(target_mask.map(|to_square| {
        let from_square = if side.is_white() {
            if double_jump { to_square.south_shift().south_shift() } else { to_square.south_shift() }
        } else {
            if double_jump { to_square.north_shift().north_shift() } else { to_square.north_shift() }
        };

        if double_jump {
            B::Move::new_double_pawn_push(from_square, to_square)
        } else {
            B::Move::new(from_square, to_square)
        }
    }));
}

#[inline]
fn extend_pawn_captures<P: Position, B: Board<P>, const N: usize>(move_list: &mut SmallVec<[B::Move; N]>, side: P::Side, target_mask: P::BoardMask, en_passant_capture: bool, west: bool) {
    move_list.extend(target_mask.map(|to_square| {
        let from_square = reverse_pawn_shift::<P>(side, to_square, west);

        if en_passant_capture {
            B::Move::new_en_passant_capture(from_square, to_square)
        } else {
            B::Move::new_capture(from_square, to_square)
        }
    }));
}


#[inline]
fn generate_pawn_moves<P: Position, B: Board<P>, const N: usize>(board: &B, move_list: &mut SmallVec<[B::Move; N]>, side_moving: P::Side, target_mask: P::BoardMask, checks_only: bool) {
    let opposite_side = side_moving.opposite_side();
    let enemy_mask = board.position().mask_for_side(opposite_side);
    let (promoting_from_rank, en_passant_rank, en_passant_attack_rank) = if side_moving.is_white() {
        (P::BoardMask::RANK_7, P::BoardMask::RANK_3, P::BoardMask::RANK_5)
    } else {
        (P::BoardMask::RANK_2, P::BoardMask::RANK_6, P::BoardMask::RANK_4)
    };
    let pawn_mask = board.position().sided_piece_mask(P::Piece::PAWN.add_side(side_moving));
    let promotable_pawns = pawn_mask & promoting_from_rank;
    let non_promoting_pawns = pawn_mask ^ promotable_pawns;
    let en_passant_attackers = non_promoting_pawns & en_passant_attack_rank;
    let en_passant_mask = board.position().en_passant_square().map_or(P::BoardMask::EMPTY, |s| s.to_mask());
    let empty_mask = board.position().empty();
    let target_mask = if checks_only {
        target_mask & board.state().piece_check_mask(P::Piece::PAWN)
    } else {
        target_mask
    };

    // Add promotions
    let west_promoting_attacks = pawn_west_attacks::<P>(promotable_pawns, side_moving) & enemy_mask & target_mask;
    extend_pawn_promotion::<P, B, N>(move_list, side_moving, west_promoting_attacks, true, true);
    let east_promoting_attacks = pawn_east_attacks::<P>(promotable_pawns, side_moving) & enemy_mask & target_mask;
    extend_pawn_promotion::<P, B, N>(move_list, side_moving, east_promoting_attacks, true, false);
    let push_promotions = pawn_pushes::<P>(promotable_pawns, side_moving) & empty_mask & target_mask;
    extend_pawn_promotion::<P, B, N>(move_list, side_moving, push_promotions, false, false);

    // Add pushes
    let pawn_pushers = pawn_pushes::<P>(non_promoting_pawns, side_moving) & empty_mask;
    extend_pawn_pushes::<P, B, N>(move_list, side_moving, pawn_pushers, false);
    let double_pawn_pushers = pawn_pushes::<P>(pawn_pushers & en_passant_rank, side_moving) & empty_mask;
    extend_pawn_pushes::<P, B, N>(move_list, side_moving, double_pawn_pushers, true);

    // Add normal pawn captures
    let west_attacks = pawn_west_attacks::<P>(non_promoting_pawns, side_moving) & enemy_mask;
    extend_pawn_captures::<P, B, N>(move_list, side_moving, west_attacks, false, true);
    let east_attacks = pawn_east_attacks::<P>(non_promoting_pawns, side_moving) & enemy_mask;
    extend_pawn_captures::<P, B, N>(move_list, side_moving, east_attacks, false, false);

    // Add en-passant captures
    let west_en_passant_captures = pawn_west_attacks::<P>(en_passant_attackers, side_moving) & enemy_mask & en_passant_mask;
    extend_pawn_captures::<P, B, N>(move_list, side_moving, west_en_passant_captures, true, true);
    let east_en_passant_captures = pawn_east_attacks::<P>(en_passant_attackers, side_moving) & enemy_mask & en_passant_mask;
    extend_pawn_captures::<P, B, N>(move_list, side_moving, east_en_passant_captures, true, false);
}

#[inline]
fn generate_king_moves<P: Position, B: Board<P>, const N: usize>(board: &B, move_list: &mut SmallVec<[B::Move; N]>, side_moving: P::Side, target_mask: P::BoardMask) {
    let king_square = board.position().king_square(side_moving);
    let king_attack_mask = king_attacks::<P>(king_square.to_mask()) & target_mask;
    extend_attack_mask_moves(board, move_list, king_square, king_attack_mask);
}

#[inline]
fn can_castle<P: Position, B: Board<P>, const N: usize>(board: &B, side_moving: P::Side, castle_in_question: P::CastleRights) -> bool {
    let occupied_mask = board.position().occupied();

    board.position().castle_rights().contains(castle_in_question) && castle_in_question.castle_path() & occupied_mask == P::BoardMask::EMPTY
}

#[inline]
fn generate_castles<P: Position, B: Board<P>, const N: usize>(board: &B, move_list: &mut SmallVec<[B::Move; N]>, side_moving: P::Side) {
    let required_king_rights = P::CastleRights::BOTH_KINGS.for_side(side_moving);
    let required_queen_rights = P::CastleRights::BOTH_QUEENS.for_side(side_moving);
    if can_castle::<P, B, N>(board, side_moving, required_king_rights) {
        move_list.push(if side_moving.is_white() {
            B::Move::WHITE_KING_CASTLE
        } else {
            B::Move::BLACK_KING_CASTLE
        });
    }
    if can_castle::<P, B, N>(board, side_moving, required_queen_rights) {
        move_list.push(if side_moving.is_white() {
            B::Move::WHITE_QUEEN_CASTLE
        } else {
            B::Move::BLACK_QUEEN_CASTLE
        });
    }
}

#[inline]
fn generate_all<P: Position, B: Board<P>, const N: usize>(board: &B, move_list: &mut SmallVec<[B::Move; N]>, side_moving: P::Side, target_mask: P::BoardMask, evasion: bool, include_castles: bool, checks_only: bool) {
    generate_pawn_moves(board, move_list, side_moving, target_mask, checks_only);
    generate_piece_moves(board, move_list, P::Piece::KNIGHT, side_moving, target_mask, checks_only);
    generate_piece_moves(board, move_list, P::Piece::BISHOP, side_moving, target_mask, checks_only);
    generate_piece_moves(board, move_list, P::Piece::ROOK, side_moving, target_mask, checks_only);
    generate_piece_moves(board, move_list, P::Piece::QUEEN, side_moving, target_mask, checks_only);

    if evasion && !checks_only {
        generate_king_moves(board, move_list, side_moving, target_mask);

        if include_castles {
            generate_castles(board, move_list, side_moving);
        }
    }
}

#[inline]
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
    let mut move_list = SmallVec::<[B::Move; BASE_MOVES_CAPACITY]>::new();
    let side_moving = board.position().side_to_move();
    let target_mask = !board.position().mask_for_side(side_moving);

    generate_all(board, &mut move_list, side_moving, target_mask, false, true, false);

    move_list.into_iter()
}

#[cfg(test)]
mod tests {
    use oxide_interface::engine::{OxidePosition, OxideBoard};
    use oxide_interface::game::{OxideMove, OxideSimpleMove, OxideSquare::*};
    use crate::legal_moves;
    use interface::game::{Position, SimpleChessMove, ChessMove};
    use interface::engine::Board;


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
        let position = OxideBoard::new(OxidePosition::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").expect("Failed to parse test case FEN"));
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
    }
}
