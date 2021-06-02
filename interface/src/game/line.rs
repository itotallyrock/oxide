use crate::game::Position;

/// Board mask specifically for lines between any given squares
pub trait LineMask<P: Position> {
    /// Board mask between two squares if they're connected including endpoints (empty if mis-aligned)
    fn line_fill(a: P::Square, b: P::Square) -> Self;
    /// Board mask between two squares if they're connected without including endpoints (empty if mis-aligned)
    fn between_fill(a: P::Square, b: P::Square) -> Self;
    /// If three squares are aligned (can have a diagonal or cardinal line drawn between them)
    fn aligned(a: P::Square, b: P::Square, c: P::Square) -> bool;
}