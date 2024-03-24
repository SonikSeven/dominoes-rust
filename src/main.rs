use dominoes::{hand::Hand, input, score, snake::Snake, stock::Stock, PieceSet};

const INVALID_INPUT_ERR_MSG: &str = "Invalid input. Please try again.\n";
const UNKNOWN_COMMAND_ERR_MSG: &str = "\nUnknown command";

fn main() {
    let mut error_message = "";
    loop {
        dominoes::clear_terminal();
        println!(
            "D O M I N O E S\n\
            \nYou won: {} times.\n\
            You lost: {} times.\n\
            {}",
            score::get_score("player"),
            score::get_score("computer"),
            error_message
        );
        match dominoes::input("\nType \"play\" to play the game and \"exit\" to quit.\n").as_str() {
            "play" => play(),
            "exit" => return,
            _ => error_message = UNKNOWN_COMMAND_ERR_MSG,
        }
    }
}

fn play() {
    let (mut stock, mut player_hand, mut computer_hand, mut snake);
    loop {
        stock = Stock::new();
        player_hand = stock.retrieve_hand();
        computer_hand = stock.retrieve_hand();
        snake = match Snake::new(&mut player_hand, &mut computer_hand) {
            Ok(s) => s,
            Err(_) => continue,
        };
        break;
    }
    let mut turn = player_hand.size() > computer_hand.size();
    loop {
        display_interface(&snake, &stock, &player_hand, &computer_hand);
        if check_game_over(&snake, &player_hand, &computer_hand) {
            return;
        }
        if turn {
            loop {
                match player_move(&mut snake, &mut stock, &mut player_hand) {
                    Ok(_) => break,
                    Err(err_msg) => println!("{}", err_msg),
                }
            }
        } else {
            let _ = computer_move(&mut snake, &mut stock, &mut computer_hand);
        }
        turn = !turn
    }
}

fn display_interface(snake: &Snake, stock: &Stock, player_hand: &Hand, computer_hand: &Hand) {
    dominoes::clear_terminal();
    let line: String = std::iter::repeat('=').take(70).collect();
    println!(
        "{}\n\
        Stock size: {}\n\
        Computer pieces: {}\n",
        line,
        stock.size(),
        computer_hand.size()
    );
    snake.display();
    println!("\nYour pieces:");
    player_hand.display();
}

fn check_game_over(snake: &Snake, player_hand: &Hand, computer_hand: &Hand) -> bool {
    if computer_hand.size() == 0 {
        score::increment_score("computer");
        input("\nStatus: The game is over. The computer won! Press Enter to continue...\n");
    } else if player_hand.size() == 0 {
        score::increment_score("player");
        input("\nStatus: The game is over. You won! Press Enter to continue...\n");
    } else if snake.check_draw() {
        input("\nStatus: The game is over. It's a draw! Press Enter to continue...\n");
    } else {
        return false;
    }
    true
}

fn player_move(
    snake: &mut Snake,
    stock: &mut Stock,
    player_hand: &mut Hand,
) -> Result<(), &'static str> {
    let mut number: i8 =
        match input("\nIt's your turn to make a move. Enter your command.\n").parse() {
            Ok(n) => n,
            Err(_) => return Err(INVALID_INPUT_ERR_MSG),
        };
    if !(0 <= number.abs() && number.abs() <= player_hand.size().try_into().unwrap()) {
        return Err(INVALID_INPUT_ERR_MSG);
    }
    if number > 0 {
        number -= 1;
        snake.add_piece_right(player_hand, number as usize)?
    } else if number < 0 {
        number = number.abs() - 1;
        snake.add_piece_left(player_hand, number as usize)?
    } else {
        player_hand.add_piece(stock.retrieve_piece()?);
    }
    Ok(())
}

fn computer_move(
    snake: &mut Snake,
    stock: &mut Stock,
    computer_hand: &mut Hand,
) -> Result<(), &'static str> {
    input("\nComputer is about to make a move. Press Enter to continue...\n");
    if computer_hand.best_move(snake).is_err() {
        computer_hand.add_piece(stock.retrieve_piece()?);
    }
    Ok(())
}
