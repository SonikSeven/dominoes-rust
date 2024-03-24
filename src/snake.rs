use crate::{hand::Hand, Piece, PieceSet};

const ILLEGAL_MOVE_ERR_MSG: &str = "Illegal move. Please try again.\n";

pub struct Snake {
    pieces: Vec<Piece>,
}

impl PieceSet for Snake {
    fn get_pieces<'a>(&'a self) -> &'a Vec<Piece> {
        &self.pieces
    }

    fn get_pieces_mut(self: &mut Self) -> &mut Vec<Piece> {
        &mut self.pieces
    }
}

impl Snake {
    pub fn new(hand_1: &mut Hand, hand_2: &mut Hand) -> Result<Snake, &'static str> {
        match hand_1.retrieve_highest_double_piece(hand_2) {
            Ok(highest_double) => Ok(Snake {
                pieces: vec![highest_double],
            }),
            Err(err_msg) => Err(err_msg),
        }
    }

    pub fn display(&self) {
        if self.size() < 6 {
            println!(
                "{}",
                self.pieces
                    .iter()
                    .map(|&[x, y]| format!("[{}, {}]", x, y))
                    .collect::<String>()
            );
        } else {
            let first_part = &self.pieces[..3];
            let last_part = &self.pieces[self.pieces.len() - 3..];
            println!(
                "{}...{}",
                first_part
                    .iter()
                    .map(|&[x, y]| format!("[{}, {}]", x, y))
                    .collect::<String>(),
                last_part
                    .iter()
                    .map(|&[x, y]| format!("[{}, {}]", x, y))
                    .collect::<String>()
            );
        }
    }

    pub fn right_end(self: &Self) -> u8 {
        self.pieces[self.size() - 1][1]
    }

    pub fn left_end(self: &Self) -> u8 {
        self.pieces[0][0]
    }

    pub fn add_piece_right(
        self: &mut Self,
        hand: &mut Hand,
        index: usize,
    ) -> Result<(), &'static str> {
        if self.right_end() == hand.get_piece(index)[0] {
            let piece = hand.retrieve_piece(index);
            self.pieces.push(piece);
            Ok(())
        } else if self.right_end() == hand.get_piece(index)[1] {
            let mut piece = hand.retrieve_piece(index);
            piece.reverse();
            self.pieces.push(piece);
            Ok(())
        } else {
            Err(ILLEGAL_MOVE_ERR_MSG)
        }
    }

    pub fn add_piece_left(
        self: &mut Self,
        hand: &mut Hand,
        index: usize,
    ) -> Result<(), &'static str> {
        if self.left_end() == hand.get_piece(index)[1] {
            let piece = hand.retrieve_piece(index);
            self.pieces.insert(0, piece);
            Ok(())
        } else if self.left_end() == hand.get_piece(index)[0] {
            let mut piece = hand.retrieve_piece(index);
            piece.reverse();
            self.pieces.insert(0, piece);
            Ok(())
        } else {
            Err(ILLEGAL_MOVE_ERR_MSG)
        }
    }

    pub fn add_piece(self: &mut Self, hand: &mut Hand, index: usize) -> Result<(), &'static str> {
        if self.add_piece_right(hand, index).is_err() {
            self.add_piece_left(hand, index)?;
        }
        Ok(())
    }

    pub fn check_draw(self: &Self) -> bool {
        if self.right_end() == self.left_end() {
            for i in 1..7 {
                if self.count_number_occurrances(i) >= 8 {
                    return true;
                }
            }
        }
        false
    }
}
