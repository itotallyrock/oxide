
use interface::game::Square;
use oxide_interface::game::{OxideSquare, OxideBitboard};

fn main() {
    const SQUARES: &[OxideSquare] = &[OxideSquare::A2, OxideSquare::B3, OxideSquare::H4, OxideSquare::E5];
    let mask = SQUARES.iter().copied().collect::<OxideBitboard>();
    for k in mask.into_iter() {
        dbg!(k);
    }
    // let t = oxide_interface::game::OxideSquare::SQUARES;
    for x in <OxideSquare as Square<OxideBitboard, 64>>::SQUARES.iter().copied().filter(|s| SQUARES.contains(s)) {
        println!("{}", x);
    }
}
