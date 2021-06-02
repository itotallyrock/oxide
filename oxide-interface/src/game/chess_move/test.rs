
#[cfg(test)]
mod test {
    use interface::game::{ChessMove, SimpleChessMove, Square};
    use crate::game::square::OxideSquare::*;
    use crate::game::{OxideMove, OxidePiece, OxideSimpleMove, OxideSquare};

    #[test]
    fn is_castle_works() {
        assert!(OxideMove::WHITE_KING_CASTLE.is_king_castle());
        assert!(!OxideMove::WHITE_KING_CASTLE.is_queen_castle());
        assert!(OxideMove::BLACK_KING_CASTLE.is_king_castle());
        assert!(!OxideMove::BLACK_KING_CASTLE.is_queen_castle());
        assert!(!OxideMove::WHITE_QUEEN_CASTLE.is_king_castle());
        assert!(OxideMove::WHITE_QUEEN_CASTLE.is_queen_castle());
        assert!(!OxideMove::BLACK_QUEEN_CASTLE.is_king_castle());
        assert!(OxideMove::BLACK_QUEEN_CASTLE.is_queen_castle());
        // Non castle moves
        assert!(!OxideMove::new(A2, A3).is_queen_castle());
        assert!(!OxideMove::new(A2, A3).is_queen_castle());
        assert!(!OxideMove::new_promoting_capture(A7, B8, OxidePiece::Queen).is_queen_castle());
        assert!(!OxideMove::new_promoting_capture(A7, B8, OxidePiece::Queen).is_queen_castle());
        assert!(!OxideMove::new_promotion(A7, A8, OxidePiece::Queen).is_queen_castle());
        assert!(!OxideMove::new_promotion(A7, A8, OxidePiece::Queen).is_queen_castle());
        assert!(!OxideMove::new_double_pawn_push(A2, A4).is_queen_castle());
        assert!(!OxideMove::new_double_pawn_push(A2, A4).is_queen_castle());
    }


    #[test]
    fn new_double_pawn_push_works() {
        let m = OxideMove::new_double_pawn_push(H7, H5);
        assert!(!m.is_capture());
        assert!(!m.is_en_passant_capture());
        assert!(!m.is_quiet());
        assert!(!m.is_promotion());
        assert!(m.is_double_pawn_push());
        assert!(!m.is_king_castle());
        assert!(!m.is_queen_castle());
        assert_eq!(m.from(), H7);
        assert_eq!(m.to(), H5);
    }

    #[test]
    fn new_en_passant_capture_works() {
        let m = OxideMove::new_en_passant_capture(A2, B3);
        assert!(m.is_capture());
        assert!(m.is_en_passant_capture());
        assert!(!m.is_quiet());
        assert!(!m.is_promotion());
        assert!(!m.is_double_pawn_push());
        assert!(!m.is_king_castle());
        assert!(!m.is_queen_castle());
        assert_eq!(m.from(), A2);
        assert_eq!(m.to(), B3);
    }

    #[test]
    fn new_capture_works() {
        let m = OxideMove::new_capture(A2, B3);
        assert!(m.is_capture());
        assert!(!m.is_en_passant_capture());
        assert!(!m.is_quiet());
        assert!(!m.is_promotion());
        assert!(!m.is_double_pawn_push());
        assert!(!m.is_king_castle());
        assert!(!m.is_queen_castle());
        assert_eq!(m.from(), A2);
        assert_eq!(m.to(), B3);
    }

    #[test]
    fn new_promotion_works() {
        let m = OxideMove::new_promotion(B7, B8, OxidePiece::Queen);
        assert!(!m.is_capture());
        assert!(!m.is_en_passant_capture());
        assert!(!m.is_quiet());
        assert!(m.is_promotion());
        assert!(!m.is_double_pawn_push());
        assert!(!m.is_king_castle());
        assert!(!m.is_queen_castle());
        assert_eq!(m.promotion(), OxidePiece::Queen);
        assert_eq!(m.from(), B7);
        assert_eq!(m.to(), B8);

        let m = OxideMove::new_promotion(F2, F1, OxidePiece::Knight);
        assert!(!m.is_capture());
        assert!(!m.is_en_passant_capture());
        assert!(!m.is_quiet());
        assert!(m.is_promotion());
        assert!(!m.is_double_pawn_push());
        assert!(!m.is_king_castle());
        assert!(!m.is_queen_castle());
        assert_eq!(m.promotion(), OxidePiece::Knight);
        assert_eq!(m.from(), F2);
        assert_eq!(m.to(), F1);
    }

    #[test]
    fn new_promoting_capture_works() {
        let m = OxideMove::new_promoting_capture(B7, C8, OxidePiece::Rook);
        assert!(m.is_capture());
        assert!(!m.is_en_passant_capture());
        assert!(!m.is_quiet());
        assert!(m.is_promotion());
        assert!(!m.is_double_pawn_push());
        assert!(!m.is_king_castle());
        assert!(!m.is_queen_castle());
        assert_eq!(m.promotion(), OxidePiece::Rook);
        assert_eq!(m.from(), B7);
        assert_eq!(m.to(), C8);

        let m = OxideMove::new_promoting_capture(F2, E1, OxidePiece::Bishop);
        assert!(m.is_capture());
        assert!(!m.is_en_passant_capture());
        assert!(!m.is_quiet());
        assert!(m.is_promotion());
        assert!(!m.is_double_pawn_push());
        assert!(!m.is_king_castle());
        assert!(!m.is_queen_castle());
        assert_eq!(m.promotion(), OxidePiece::Bishop);
        assert_eq!(m.from(), F2);
        assert_eq!(m.to(), E1);
    }

    #[test]
    fn from_simple_move_works() {
        todo!("Test a bunch of simple moves that are actually side-effect moves")
    }

    #[test]
    fn simple_move_works() {
        for &from_square in &OxideSquare::SQUARES {
            for &to_square in &OxideSquare::SQUARES {
                if from_square != to_square {
                    // Make sure new matches simple_move_new
                    assert_eq!(OxideMove::new(from_square, to_square).simple_move, OxideSimpleMove::new(from_square, to_square));
                }
            }
        }
    }

    #[test]
    fn promotion_works() {
        assert_eq!(OxideMove::new_promotion(H2, H1, OxidePiece::Queen).promotion(), OxidePiece::Queen);
        assert_eq!(OxideMove::new_promoting_capture(F7, E8, OxidePiece::Queen).promotion(), OxidePiece::Queen);
        assert_eq!(OxideMove::new_promotion(D2, D1, OxidePiece::Bishop).promotion(), OxidePiece::Bishop);
        assert_eq!(OxideMove::new_promoting_capture(E7, F8, OxidePiece::Bishop).promotion(), OxidePiece::Bishop);
        assert_eq!(OxideMove::new_promotion(B2, B1, OxidePiece::Rook).promotion(), OxidePiece::Rook);
        assert_eq!(OxideMove::new_promoting_capture(E7, F8, OxidePiece::Rook).promotion(), OxidePiece::Rook);
        assert_eq!(OxideMove::new_promotion(C2, C1, OxidePiece::Knight).promotion(), OxidePiece::Knight);
        assert_eq!(OxideMove::new_promoting_capture(E7, F8, OxidePiece::Knight).promotion(), OxidePiece::Knight);
        assert_eq!(OxideMove::new_double_pawn_push(H7, H5).promotion(), OxidePiece::Empty);
        assert_eq!(OxideMove::new_en_passant_capture(A2, B3).promotion(), OxidePiece::Empty);
        assert_eq!(OxideMove::new_capture(D7, E6).promotion(), OxidePiece::Empty);
        assert_eq!(OxideMove::WHITE_KING_CASTLE.promotion(), OxidePiece::Empty);
        assert_eq!(OxideMove::BLACK_KING_CASTLE.promotion(), OxidePiece::Empty);
        assert_eq!(OxideMove::BLACK_QUEEN_CASTLE.promotion(), OxidePiece::Empty);
        assert_eq!(OxideMove::WHITE_QUEEN_CASTLE.promotion(), OxidePiece::Empty);
        assert_eq!(OxideMove::new(D7, E6).promotion(), OxidePiece::Empty);
    }

    #[test]
    fn is_quiet_works() {
        assert!(OxideMove::new(D7, E6).is_quiet());
        assert!(OxideMove::new(F2, E8).is_quiet());
        assert!(OxideMove::new(A1, H3).is_quiet());
        assert!(!OxideMove::new_en_passant_capture(A2, B3).is_quiet());
        assert!(!OxideMove::new_en_passant_capture(B2, A3).is_quiet());
        assert!(!OxideMove::new_en_passant_capture(H7, G6).is_quiet());
        assert!(!OxideMove::new_en_passant_capture(D7, E6).is_quiet());
        assert!(!OxideMove::new_capture(D7, E6).is_quiet());
        assert!(!OxideMove::new_promoting_capture(E7, F8, OxidePiece::Queen).is_quiet());
        assert!(!OxideMove::WHITE_KING_CASTLE.is_quiet());
        assert!(!OxideMove::BLACK_KING_CASTLE.is_quiet());
        assert!(!OxideMove::BLACK_QUEEN_CASTLE.is_quiet());
        assert!(!OxideMove::WHITE_QUEEN_CASTLE.is_quiet());
        assert!(!OxideMove::new_double_pawn_push(E2, E4).is_quiet());
        assert!(!OxideMove::new_promotion(H2, H1, OxidePiece::Queen).is_quiet());
    }

    #[test]
    fn is_double_pawn_push_works() {
        assert!(OxideMove::new_double_pawn_push(E2, E4).is_double_pawn_push());
        assert!(OxideMove::new_double_pawn_push(F2, F4).is_double_pawn_push());
        assert!(OxideMove::new_double_pawn_push(E7, E5).is_double_pawn_push());
        assert!(OxideMove::new_double_pawn_push(H7, H5).is_double_pawn_push());
        assert!(!OxideMove::new_en_passant_capture(A2, B3).is_double_pawn_push());
        assert!(!OxideMove::new_en_passant_capture(B2, A3).is_double_pawn_push());
        assert!(!OxideMove::new_en_passant_capture(H7, G6).is_double_pawn_push());
        assert!(!OxideMove::new_en_passant_capture(D7, E6).is_double_pawn_push());
        assert!(!OxideMove::new_capture(D7, E6).is_double_pawn_push());
        assert!(!OxideMove::new_promoting_capture(E7, F8, OxidePiece::Queen).is_double_pawn_push());
        assert!(!OxideMove::WHITE_KING_CASTLE.is_double_pawn_push());
        assert!(!OxideMove::BLACK_KING_CASTLE.is_double_pawn_push());
        assert!(!OxideMove::BLACK_QUEEN_CASTLE.is_double_pawn_push());
        assert!(!OxideMove::WHITE_QUEEN_CASTLE.is_double_pawn_push());
        assert!(!OxideMove::new(D7, E6).is_double_pawn_push());
        assert!(!OxideMove::new_promotion(H2, H1, OxidePiece::Queen).is_double_pawn_push());
    }

    #[test]
    fn is_promotion_works() {
        assert!(OxideMove::new_promotion(H2, H1, OxidePiece::Queen).is_promotion());
        assert!(OxideMove::new_promoting_capture(F7, E8, OxidePiece::Queen).is_promotion());
        assert!(OxideMove::new_promotion(D2, D1, OxidePiece::Bishop).is_promotion());
        assert!(OxideMove::new_promoting_capture(E7, F8, OxidePiece::Bishop).is_promotion());
        assert!(OxideMove::new_promotion(B2, B1, OxidePiece::Rook).is_promotion());
        assert!(OxideMove::new_promoting_capture(E7, F8, OxidePiece::Rook).is_promotion());
        assert!(OxideMove::new_promotion(C2, C1, OxidePiece::Knight).is_promotion());
        assert!(OxideMove::new_promoting_capture(E7, F8, OxidePiece::Knight).is_promotion());
        assert!(!OxideMove::new_en_passant_capture(A2, B3).is_promotion());
        assert!(!OxideMove::new_en_passant_capture(B2, A3).is_promotion());
        assert!(!OxideMove::new_en_passant_capture(H7, G6).is_promotion());
        assert!(!OxideMove::new_en_passant_capture(D7, E6).is_promotion());
        assert!(!OxideMove::new_capture(D7, E6).is_promotion());
        assert!(!OxideMove::WHITE_KING_CASTLE.is_promotion());
        assert!(!OxideMove::BLACK_KING_CASTLE.is_promotion());
        assert!(!OxideMove::BLACK_QUEEN_CASTLE.is_promotion());
        assert!(!OxideMove::WHITE_QUEEN_CASTLE.is_promotion());
        assert!(!OxideMove::new(D7, E6).is_promotion());
        assert!(!OxideMove::new_double_pawn_push(E2, E4).is_promotion());
    }

    #[test]
    fn is_capture_works() {
        assert!(OxideMove::new_en_passant_capture(A2, B3).is_capture());
        assert!(OxideMove::new_en_passant_capture(B2, A3).is_capture());
        assert!(OxideMove::new_en_passant_capture(H7, G6).is_capture());
        assert!(OxideMove::new_en_passant_capture(D7, E6).is_capture());
        assert!(OxideMove::new_capture(D7, E6).is_capture());
        assert!(OxideMove::new_promoting_capture(E7, F8, OxidePiece::Queen).is_capture());
        assert!(!OxideMove::WHITE_KING_CASTLE.is_capture());
        assert!(!OxideMove::BLACK_KING_CASTLE.is_capture());
        assert!(!OxideMove::BLACK_QUEEN_CASTLE.is_capture());
        assert!(!OxideMove::WHITE_QUEEN_CASTLE.is_capture());
        assert!(!OxideMove::new(D7, E6).is_capture());
        assert!(!OxideMove::new_double_pawn_push(E2, E4).is_capture());
        assert!(!OxideMove::new_promotion(H2, H1, OxidePiece::Queen).is_capture());
    }

    #[test]
    fn is_king_castle_works() {
        assert!(OxideMove::WHITE_KING_CASTLE.is_king_castle());
        assert!(OxideMove::BLACK_KING_CASTLE.is_king_castle());
        assert!(!OxideMove::BLACK_QUEEN_CASTLE.is_king_castle());
        assert!(!OxideMove::WHITE_QUEEN_CASTLE.is_king_castle());
        assert!(!OxideMove::new_en_passant_capture(A2, B3).is_king_castle());
        assert!(!OxideMove::new_en_passant_capture(B2, A3).is_king_castle());
        assert!(!OxideMove::new_en_passant_capture(H7, G6).is_king_castle());
        assert!(!OxideMove::new_en_passant_capture(D7, E6).is_king_castle());
        assert!(!OxideMove::new(D7, E6).is_king_castle());
        assert!(!OxideMove::new_capture(D7, E6).is_king_castle());
        assert!(!OxideMove::new_double_pawn_push(E2, E4).is_king_castle());
        assert!(!OxideMove::new_promoting_capture(E7, F8, OxidePiece::Queen).is_king_castle());
        assert!(!OxideMove::new_promotion(H2, H1, OxidePiece::Queen).is_king_castle());
    }

    #[test]
    fn is_queen_castle_works() {
        assert!(OxideMove::BLACK_QUEEN_CASTLE.is_queen_castle());
        assert!(OxideMove::WHITE_QUEEN_CASTLE.is_queen_castle());
        assert!(!OxideMove::WHITE_KING_CASTLE.is_queen_castle());
        assert!(!OxideMove::BLACK_KING_CASTLE.is_queen_castle());
        assert!(!OxideMove::new_en_passant_capture(A2, B3).is_queen_castle());
        assert!(!OxideMove::new_en_passant_capture(B2, A3).is_queen_castle());
        assert!(!OxideMove::new_en_passant_capture(H7, G6).is_queen_castle());
        assert!(!OxideMove::new_en_passant_capture(D7, E6).is_queen_castle());
        assert!(!OxideMove::new(D7, E6).is_queen_castle());
        assert!(!OxideMove::new_capture(D7, E6).is_queen_castle());
        assert!(!OxideMove::new_double_pawn_push(E2, E4).is_queen_castle());
        assert!(!OxideMove::new_promoting_capture(E7, F8, OxidePiece::Queen).is_queen_castle());
        assert!(!OxideMove::new_promotion(H2, H1, OxidePiece::Queen).is_queen_castle());
    }

    #[test]
    fn is_en_passant_capture_works() {
        assert!(OxideMove::new_en_passant_capture(A2, B3).is_en_passant_capture());
        assert!(OxideMove::new_en_passant_capture(B2, A3).is_en_passant_capture());
        assert!(OxideMove::new_en_passant_capture(H7, G6).is_en_passant_capture());
        assert!(OxideMove::new_en_passant_capture(D7, E6).is_en_passant_capture());
        assert!(!OxideMove::new(D7, E6).is_en_passant_capture());
        assert!(!OxideMove::new_capture(D7, E6).is_en_passant_capture());
        assert!(!OxideMove::new_double_pawn_push(E2, E4).is_en_passant_capture());
        assert!(!OxideMove::new_promoting_capture(E7, F8, OxidePiece::Queen).is_en_passant_capture());
        assert!(!OxideMove::new_promotion(H2, H1, OxidePiece::Queen).is_en_passant_capture());
        assert!(!OxideMove::BLACK_QUEEN_CASTLE.is_en_passant_capture());
        assert!(!OxideMove::WHITE_QUEEN_CASTLE.is_en_passant_capture());
        assert!(!OxideMove::WHITE_KING_CASTLE.is_en_passant_capture());
        assert!(!OxideMove::BLACK_KING_CASTLE.is_en_passant_capture());
    }

    #[test]
    fn castles_work() {
        // Check the move's from
        assert_eq!(OxideMove::WHITE_KING_CASTLE.from(), E1);
        assert_eq!(OxideMove::BLACK_KING_CASTLE.from(), E8);
        assert_eq!(OxideMove::WHITE_QUEEN_CASTLE.from(), E1);
        assert_eq!(OxideMove::BLACK_QUEEN_CASTLE.from(), E8);
        // Check the move's to
        assert_eq!(OxideMove::WHITE_KING_CASTLE.to(), G1);
        assert_eq!(OxideMove::BLACK_KING_CASTLE.to(), G8);
        assert_eq!(OxideMove::WHITE_QUEEN_CASTLE.to(), C1);
        assert_eq!(OxideMove::BLACK_QUEEN_CASTLE.to(), C8);
    }
}
