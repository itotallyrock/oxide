
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum OxideMoveType {
    // Quiet move
    Quiet,
    // Double pawn push
    DoublePawnPush,
    // Castles
    KingSideCastle,
    QueenSideCastle,
    // Captures
    Capture,
    EnPassantCapture,
    // Promotions
    KnightPromotion = 8, // Set to 8 to align promotion captures with capture set bit
    BishopPromotion,
    RookPromotion,
    QueenPromotion,
    // Promoting Captures
    KnightPromotingCapture,
    BishopPromotingCapture,
    RookPromotingCapture,
    QueenPromotingCapture = 15,
}
