
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn promotes_works() {
        assert!(!Flags::Quiet.promotes());
        assert!(!Flags::DoublePawnPush.promotes());
        assert!(!Flags::KingSideCastle.promotes());
        assert!(!Flags::QueenSideCastle.promotes());
        assert!(!Flags::Capture.promotes());
        assert!(!Flags::EnPassantCapture.promotes());
        assert!(Flags::KnightPromotion.promotes());
        assert!(Flags::BishopPromotion.promotes());
        assert!(Flags::RookPromotion.promotes());
        assert!(Flags::QueenPromotion.promotes());
        assert!(Flags::KnightPromotingCapture.promotes());
        assert!(Flags::BishopPromotingCapture.promotes());
        assert!(Flags::RookPromotingCapture.promotes());
        assert!(Flags::QueenPromotingCapture.promotes());
    }

    #[test]
    fn for_promotion_works() {
        assert_eq!(Flags::for_promotion(Piece::Knight), Flags::KnightPromotion);
        assert_eq!(Flags::for_promotion(Piece::Rook), Flags::RookPromotion);
        assert_eq!(Flags::for_promotion(Piece::Queen), Flags::QueenPromotion);
        assert_eq!(Flags::for_promotion(Piece::Bishop), Flags::BishopPromotion);
    }

    #[test]
    #[should_panic]
    fn for_promotion_panics_pawn_promotion() {
        Flags::for_promotion(Piece::Pawn);
    }

    #[test]
    #[should_panic]
    fn for_promotion_panics_king_promotion() {
        Flags::for_promotion(Piece::King);
    }

    #[test]
    fn captures_works() {
        assert!(Flags::EnPassantCapture.captures());
        assert!(Flags::BishopPromotingCapture.captures());
        assert!(Flags::KnightPromotingCapture.captures());
        assert!(Flags::QueenPromotingCapture.captures());
        assert!(Flags::RookPromotingCapture.captures());
        assert!(Flags::Capture.captures());
        // Non captures are false
        assert!(!Flags::BishopPromotion.captures());
        assert!(!Flags::KnightPromotion.captures());
        assert!(!Flags::QueenPromotion.captures());
        assert!(!Flags::RookPromotion.captures());
        assert!(!Flags::Quiet.captures());
        assert!(!Flags::QueenSideCastle.captures());
        assert!(!Flags::KingSideCastle.captures());
        assert!(!Flags::DoublePawnPush.captures());
    }

    #[test]
    fn promotion_works() {
        // Non promoting moves are none
        assert_eq!(Flags::QueenSideCastle.promotion(), Piece::None);
        assert_eq!(Flags::KingSideCastle.promotion(), Piece::None);
        assert_eq!(Flags::Quiet.promotion(), Piece::None);
        assert_eq!(Flags::Capture.promotion(), Piece::None);
        assert_eq!(Flags::EnPassantCapture.promotion(), Piece::None);
        assert_eq!(Flags::DoublePawnPush.promotion(), Piece::None);
        // Promotions are their pieces
        assert_eq!(Flags::BishopPromotingCapture.promotion(), Piece::Bishop);
        assert_eq!(Flags::KnightPromotingCapture.promotion(), Piece::Knight);
        assert_eq!(Flags::QueenPromotingCapture.promotion(), Piece::Queen);
        assert_eq!(Flags::RookPromotingCapture.promotion(), Piece::Rook);
        assert_eq!(Flags::BishopPromotion.promotion(), Piece::Bishop);
        assert_eq!(Flags::KnightPromotion.promotion(), Piece::Knight);
        assert_eq!(Flags::QueenPromotion.promotion(), Piece::Queen);
        assert_eq!(Flags::RookPromotion.promotion(), Piece::Rook);
    }

    #[test]
    fn add_capture_works() {
        // Non captures become their capture counter-parts
        assert_eq!(Flags::Quiet.add_capture(), Flags::Capture);
        assert_eq!(Flags::RookPromotion.add_capture(), Flags::RookPromotingCapture);
        assert_eq!(Flags::BishopPromotion.add_capture(), Flags::BishopPromotingCapture);
        assert_eq!(Flags::QueenPromotion.add_capture(), Flags::QueenPromotingCapture);
        assert_eq!(Flags::KnightPromotion.add_capture(), Flags::KnightPromotingCapture);
        // Captures keep their captures
        assert_eq!(Flags::Capture.add_capture(), Flags::Capture);
        assert_eq!(Flags::EnPassantCapture.add_capture(), Flags::EnPassantCapture);
        assert_eq!(Flags::RookPromotingCapture.add_capture(), Flags::RookPromotingCapture);
        assert_eq!(Flags::BishopPromotingCapture.add_capture(), Flags::BishopPromotingCapture);
        assert_eq!(Flags::QueenPromotingCapture.add_capture(), Flags::QueenPromotingCapture);
        assert_eq!(Flags::KnightPromotingCapture.add_capture(), Flags::KnightPromotingCapture);
    }

    #[test]
    #[should_panic]
    fn add_capture_double_pawn_push_panics() {
        Flags::DoublePawnPush.add_capture();
    }

    #[test]
    #[should_panic]
    fn add_capture_king_side_castle_panics() {
        Flags::KingSideCastle.add_capture();
    }

    #[test]
    #[should_panic]
    fn add_capture_queen_side_castle_panics() {
        Flags::QueenSideCastle.add_capture();
    }

    #[test]
    fn remove_capture_works() {
        // Non captures become remain the same
        assert_eq!(Flags::Quiet.remove_capture(), Flags::Quiet);
        assert_eq!(Flags::RookPromotion.remove_capture(), Flags::RookPromotion);
        assert_eq!(Flags::BishopPromotion.remove_capture(), Flags::BishopPromotion);
        assert_eq!(Flags::QueenPromotion.remove_capture(), Flags::QueenPromotion);
        assert_eq!(Flags::KnightPromotion.remove_capture(), Flags::KnightPromotion);
        assert_eq!(Flags::KingSideCastle.remove_capture(), Flags::KingSideCastle);
        assert_eq!(Flags::QueenSideCastle.remove_capture(), Flags::QueenSideCastle);
        assert_eq!(Flags::DoublePawnPush.remove_capture(), Flags::DoublePawnPush);
        // Captures lose their captures
        assert_eq!(Flags::Capture.remove_capture(), Flags::Quiet);
        assert_eq!(Flags::EnPassantCapture.remove_capture(), Flags::Quiet);
        assert_eq!(Flags::RookPromotingCapture.remove_capture(), Flags::RookPromotion);
        assert_eq!(Flags::BishopPromotingCapture.remove_capture(), Flags::BishopPromotion);
        assert_eq!(Flags::QueenPromotingCapture.remove_capture(), Flags::QueenPromotion);
        assert_eq!(Flags::KnightPromotingCapture.remove_capture(), Flags::KnightPromotion);
    }

    #[test]
    fn castles_work() {
        // Check the move's flags
        assert_eq!(ChessMove::WHITE_KING_CASTLE.flags(), Flags::KingSideCastle);
        assert_eq!(ChessMove::BLACK_KING_CASTLE.flags(), Flags::KingSideCastle);
        assert_eq!(ChessMove::WHITE_QUEEN_CASTLE.flags(), Flags::QueenSideCastle);
        assert_eq!(ChessMove::BLACK_QUEEN_CASTLE.flags(), Flags::QueenSideCastle);
        // Check the move's from
        assert_eq!(ChessMove::WHITE_KING_CASTLE.from(), E1);
        assert_eq!(ChessMove::BLACK_KING_CASTLE.from(), E8);
        assert_eq!(ChessMove::WHITE_QUEEN_CASTLE.from(), E1);
        assert_eq!(ChessMove::BLACK_QUEEN_CASTLE.from(), E8);
        // Check the move's to
        assert_eq!(ChessMove::WHITE_KING_CASTLE.to(), G1);
        assert_eq!(ChessMove::BLACK_KING_CASTLE.to(), G8);
        assert_eq!(ChessMove::WHITE_QUEEN_CASTLE.to(), C1);
        assert_eq!(ChessMove::BLACK_QUEEN_CASTLE.to(), C8);
    }
}
