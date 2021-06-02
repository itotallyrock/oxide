
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
        for &from_square in &OxideSquare::SQUARES {
            for &to_square in &OxideSquare::SQUARES {
                if from_square != to_square {
                    // Make sure from_simple_move matches new with the same squares
                    assert_eq!(OxideMove::from_simple_move(OxideSimpleMove::new(from_square, to_square)), OxideMove::new(from_square, to_square), "from_simple_move(A, B) doesn't match new(A, B)");
                }
            }
        }
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

    /*
    #[test]
    fn promotes_works() {
        assert!(!OxideMoveType::Quiet.promotes());
        assert!(!OxideMoveType::DoublePawnPush.promotes());
        assert!(!OxideMoveType::KingSideCastle.promotes());
        assert!(!OxideMoveType::QueenSideCastle.promotes());
        assert!(!OxideMoveType::Capture.promotes());
        assert!(!OxideMoveType::EnPassantCapture.promotes());
        assert!(OxideMoveType::KnightPromotion.promotes());
        assert!(OxideMoveType::BishopPromotion.promotes());
        assert!(OxideMoveType::RookPromotion.promotes());
        assert!(OxideMoveType::QueenPromotion.promotes());
        assert!(OxideMoveType::KnightPromotingCapture.promotes());
        assert!(OxideMoveType::BishopPromotingCapture.promotes());
        assert!(OxideMoveType::RookPromotingCapture.promotes());
        assert!(OxideMoveType::QueenPromotingCapture.promotes());
    }

    #[test]
    fn captures_works() {
        assert!(OxideMoveType::EnPassantCapture.captures());
        assert!(OxideMoveType::BishopPromotingCapture.captures());
        assert!(OxideMoveType::KnightPromotingCapture.captures());
        assert!(OxideMoveType::QueenPromotingCapture.captures());
        assert!(OxideMoveType::RookPromotingCapture.captures());
        assert!(OxideMoveType::Capture.captures());
        // Non captures are false
        assert!(!OxideMoveType::BishopPromotion.captures());
        assert!(!OxideMoveType::KnightPromotion.captures());
        assert!(!OxideMoveType::QueenPromotion.captures());
        assert!(!OxideMoveType::RookPromotion.captures());
        assert!(!OxideMoveType::Quiet.captures());
        assert!(!OxideMoveType::QueenSideCastle.captures());
        assert!(!OxideMoveType::KingSideCastle.captures());
        assert!(!OxideMoveType::DoublePawnPush.captures());
    }

    #[test]
    fn promotion_works() {
        // Non promoting moves are none
        assert_eq!(OxideMoveType::QueenSideCastle.promotion(), OxidePiece::Empty);
        assert_eq!(OxideMoveType::KingSideCastle.promotion(), OxidePiece::Empty);
        assert_eq!(OxideMoveType::Quiet.promotion(), OxidePiece::Empty);
        assert_eq!(OxideMoveType::Capture.promotion(), OxidePiece::Empty);
        assert_eq!(OxideMoveType::EnPassantCapture.promotion(), OxidePiece::Empty);
        assert_eq!(OxideMoveType::DoublePawnPush.promotion(), OxidePiece::Empty);
        // Promotions are their pieces
        assert_eq!(OxideMoveType::BishopPromotingCapture.promotion(), OxidePiece::Bishop);
        assert_eq!(OxideMoveType::KnightPromotingCapture.promotion(), OxidePiece::Knight);
        assert_eq!(OxideMoveType::QueenPromotingCapture.promotion(), OxidePiece::Queen);
        assert_eq!(OxideMoveType::RookPromotingCapture.promotion(), OxidePiece::Rook);
        assert_eq!(OxideMoveType::BishopPromotion.promotion(), OxidePiece::Bishop);
        assert_eq!(OxideMoveType::KnightPromotion.promotion(), OxidePiece::Knight);
        assert_eq!(OxideMoveType::QueenPromotion.promotion(), OxidePiece::Queen);
        assert_eq!(OxideMoveType::RookPromotion.promotion(), OxidePiece::Rook);
    }

    #[test]
    fn add_capture_works() {
        // Non captures become their capture counter-parts
        assert_eq!(OxideMoveType::Quiet.add_capture(), OxideMoveType::Capture);
        assert_eq!(OxideMoveType::RookPromotion.add_capture(), OxideMoveType::RookPromotingCapture);
        assert_eq!(OxideMoveType::BishopPromotion.add_capture(), OxideMoveType::BishopPromotingCapture);
        assert_eq!(OxideMoveType::QueenPromotion.add_capture(), OxideMoveType::QueenPromotingCapture);
        assert_eq!(OxideMoveType::KnightPromotion.add_capture(), OxideMoveType::KnightPromotingCapture);
        // Captures keep their captures
        assert_eq!(OxideMoveType::Capture.add_capture(), OxideMoveType::Capture);
        assert_eq!(OxideMoveType::EnPassantCapture.add_capture(), OxideMoveType::EnPassantCapture);
        assert_eq!(OxideMoveType::RookPromotingCapture.add_capture(), OxideMoveType::RookPromotingCapture);
        assert_eq!(OxideMoveType::BishopPromotingCapture.add_capture(), OxideMoveType::BishopPromotingCapture);
        assert_eq!(OxideMoveType::QueenPromotingCapture.add_capture(), OxideMoveType::QueenPromotingCapture);
        assert_eq!(OxideMoveType::KnightPromotingCapture.add_capture(), OxideMoveType::KnightPromotingCapture);
    }

    #[test]
    #[should_panic]
    fn add_capture_double_pawn_push_panics() {
        OxideMoveType::DoublePawnPush.add_capture();
    }

    #[test]
    #[should_panic]
    fn add_capture_king_side_castle_panics() {
        OxideMoveType::KingSideCastle.add_capture();
    }

    #[test]
    #[should_panic]
    fn add_capture_queen_side_castle_panics() {
        OxideMoveType::QueenSideCastle.add_capture();
    }

    #[test]
    fn remove_capture_works() {
        // Non captures become remain the same
        assert_eq!(OxideMoveType::Quiet.remove_capture(), OxideMoveType::Quiet);
        assert_eq!(OxideMoveType::RookPromotion.remove_capture(), OxideMoveType::RookPromotion);
        assert_eq!(OxideMoveType::BishopPromotion.remove_capture(), OxideMoveType::BishopPromotion);
        assert_eq!(OxideMoveType::QueenPromotion.remove_capture(), OxideMoveType::QueenPromotion);
        assert_eq!(OxideMoveType::KnightPromotion.remove_capture(), OxideMoveType::KnightPromotion);
        assert_eq!(OxideMoveType::KingSideCastle.remove_capture(), OxideMoveType::KingSideCastle);
        assert_eq!(OxideMoveType::QueenSideCastle.remove_capture(), OxideMoveType::QueenSideCastle);
        assert_eq!(OxideMoveType::DoublePawnPush.remove_capture(), OxideMoveType::DoublePawnPush);
        // CapturesOxideMoveType their captures
        assert_eq!(OxideMoveType::Capture.remove_capture(), OxideMoveType::Quiet);
        assert_eq!(OxideMoveType::EnPassantCapture.remove_capture(), OxideMoveType::Quiet);
        assert_eq!(OxideMoveType::RookPromotingCapture.remove_capture(), OxideMoveType::RookPromotion);
        assert_eq!(OxideMoveType::BishopPromotingCapture.remove_capture(), OxideMoveType::BishopPromotion);
        assert_eq!(OxideMoveType::QueenPromotingCapture.remove_capture(), OxideMoveType::QueenPromotion);
        assert_eq!(OxideMoveType::KnightPromotingCapture.remove_capture(), OxideMoveType::KnightPromotion);
    }*/

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
