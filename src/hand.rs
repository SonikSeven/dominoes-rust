use crate::{snake::Snake, Piece, PieceSet};

const NO_DOUBLES_ERR_MSG: &str = "No a single double piece in either hand";
const NO_POSSIBLE_MOVES_ERR_MSG: &str = "No possible moves";

struct Selected {
    index: usize,
    score: usize,
}

pub struct Hand {
    pieces: Vec<Piece>,
}

impl PieceSet for Hand {
    fn get_pieces<'a>(&'a self) -> &'a Vec<Piece> {
        &self.pieces
    }

    fn get_pieces_mut(self: &mut Self) -> &mut Vec<Piece> {
        &mut self.pieces
    }
}

impl Hand {
    pub fn new(pieces: Vec<Piece>) -> Hand {
        Hand { pieces }
    }

    pub fn display(self: &Self) {
        self.get_pieces()
            .iter()
            .enumerate()
            .for_each(|(number, piece)| println!("{:?}:{:?}", number + 1, piece));
    }

    pub fn retrieve_piece(self: &mut Self, index: usize) -> Piece {
        self.get_pieces_mut().swap_remove(index)
    }

    pub fn get_piece(self: &Self, index: usize) -> Piece {
        self.pieces[index]
    }

    pub fn add_piece(self: &mut Self, piece: Piece) {
        self.get_pieces_mut().push(piece);
    }

    pub fn retrieve_highest_double_piece(
        self: &mut Self,
        other: &mut Self,
    ) -> Result<Piece, &'static str> {
        for i in (0..=7).rev() {
            let piece = [i, i];
            if self.get_pieces().contains(&piece) {
                let index = self.get_pieces().iter().position(|&e| e == piece);
                return Ok(self.retrieve_piece(index.unwrap()));
            } else if other.pieces.contains(&piece) {
                let index = other.get_pieces().iter().position(|&e| e == piece);
                return Ok(other.retrieve_piece(index.unwrap()));
            }
        }
        Err(NO_DOUBLES_ERR_MSG)
    }

    pub fn best_move(self: &mut Self, snake: &mut Snake) -> Result<(), &'static str> {
        let mut numbers = snake.numbers();
        numbers.append(&mut self.numbers());
        let mut selected = Selected { index: 0, score: 0 };
        for (index, [left, right]) in self.pieces.iter().enumerate() {
            if *left == snake.left_end()
                || *left == snake.right_end()
                || *right == snake.left_end()
                || *right == snake.right_end()
            {
                let score = numbers.iter().filter(|&x| x == left).count()
                    + numbers.iter().filter(|&x| x == right).count();
                if score > selected.score {
                    selected.index = index;
                    selected.score = score;
                }
            }
        }
        if selected.score == 0 {
            return Err(NO_POSSIBLE_MOVES_ERR_MSG);
        }
        snake.add_piece(self, selected.index)?;
        Ok(())
    }
}
