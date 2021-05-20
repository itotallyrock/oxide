use crate::game::Side;

pub trait Position<SideType: Side> {
    fn from_fen(fen: &str) -> Self;
    fn side_to_move(&self) -> SideType;
}