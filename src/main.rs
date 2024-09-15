use cozy_chess::{Board, Move};
use nnue::position::Position;

mod nnue;

fn main () {
    let mut board = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1".parse::<Board>().unwrap();

    let mut pos = Position::new(board);
    pos.make_move("d2e3".parse::<Move>().unwrap());
    
    let e = pos.get_eval();
    
    pos.unmake_move();

    println!("{:?}", e);
}
