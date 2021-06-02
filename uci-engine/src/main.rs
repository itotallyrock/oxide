
use interface::game::{Square, ChessMove, SimpleChessMove};
use oxide_interface::game::{OxideSquare, OxideBitboard, OxideMove, OxideSquare::*, OxidePiece};

fn main() {
    const SQUARES: &[OxideSquare] = &[OxideSquare::A2, OxideSquare::B3, OxideSquare::H4, OxideSquare::E5];
    let mask = SQUARES.iter().copied().collect::<OxideBitboard>();
    for k in mask.into_iter() {
        dbg!(k);
    }
    // let t = oxide_interface::game::OxideSquare::SQUARES;
    for x in OxideSquare::SQUARES.iter().copied().filter(|s| SQUARES.contains(s)) {
        println!("{}", x);
    }

    let mut t = OxideMove::new_promoting_capture(E7, F8, OxidePiece::Queen);
    println!("{} {} {}", OxideMove::WHITE_KING_CASTLE, OxideMove::new(E7, F8), t);
}
