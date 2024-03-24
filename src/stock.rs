use crate::{hand::Hand, Piece, PieceSet};
use rand::seq::SliceRandom;

const EMPTY_STOCK_ERR_MSG: &str = "The stock is empty";

pub struct Stock {
    pieces: Vec<Piece>,
}

impl PieceSet for Stock {
    fn get_pieces<'a>(&'a self) -> &'a Vec<Piece> {
        &self.pieces
    }

    fn get_pieces_mut(self: &mut Self) -> &mut Vec<Piece> {
        &mut self.pieces
    }
}

impl Stock {
    pub fn new() -> Stock {
        let mut pieces: Vec<[u8; 2]> = (0..7).flat_map(|y| (0..=y).map(move |x| [x, y])).collect();
        pieces.shuffle(&mut rand::thread_rng());
        Stock { pieces }
    }

    pub fn retrieve_hand(self: &mut Self) -> Hand {
        Hand::new(self.pieces.drain(0..7).collect())
    }

    pub fn retrieve_piece(self: &mut Self) -> Result<Piece, &'static str> {
        if self.size() > 0 {
            Ok(self.pieces.pop().unwrap())
        } else {
            Err(EMPTY_STOCK_ERR_MSG)
        }
    }
}
