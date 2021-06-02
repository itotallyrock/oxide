use crate::game::Position;

pub trait LineMask<P: Position> {
    fn line_fill(a: P::Square, b: P::Square) -> Self;
    fn between_fill(a: P::Square, b: P::Square) -> Self;
    fn aligned(a: P::Square, b: P::Square, c: P::Square) -> bool;
}