pub mod hand;
pub mod score;
pub mod snake;
pub mod stock;

use std::{io, process::Command};

#[cfg(target_os = "windows")]
pub fn clear_terminal() {
    let _ = Command::new("cmd").arg("/c").arg("cls").status();
}

#[cfg(not(target_os = "windows"))]
pub fn clear_terminal() {
    let _ = Command::new("clear").status();
}

pub fn input(msg: &str) -> String {
    println!("{}", msg);

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

pub type Piece = [u8; 2];

pub trait PieceSet {
    fn get_pieces<'a>(&'a self) -> &'a Vec<Piece>;

    fn get_pieces_mut(self: &mut Self) -> &mut Vec<Piece>;

    fn size(self: &Self) -> usize {
        self.get_pieces().len()
    }

    fn numbers(self: &Self) -> Vec<u8> {
        self.get_pieces().clone().into_iter().flatten().collect()
    }

    fn count_number_occurrances(self: &Self, number: u8) -> usize {
        self.get_pieces()
            .iter()
            .flatten()
            .filter(|&&e| e == number)
            .count()
    }
}
