use std::io::{self, Stdin, Stdout, Write};
mod game;

pub const COLUMN_COUNT: u8 = 3;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    let mut input = String::new();

    let mut game_state = game::GameState::new();
    let mut dice = game::Dice::new();
    writeln!(stdout, "🍀📦 Welcome to Luckybox! 📦🍀")?;
    writeln!(stdout, "Please enter your player names:")?;
    stdout.flush()?;

    for player in game_state.players.iter_mut() {
        write!(stdout, "===> ")?;
        stdout.flush()?;
        stdin.read_line(&mut input)?;
        player.add_name(input.trim_end());
    }
    input.clear();
    while !game_state.is_complete() {
        let dice_roll = dice.roll();
        let current_player = game_state.get_current_player().unwrap(); // Live dangerously
        writeln!(
            stdout,
            "It's your turn {} - you've rolled a {dice_roll} 🎲",
            current_player.name
        )?;
        writeln!(stdout, "What column do you want to put it in? ")?;
        stdout.flush()?;
        let non_empty_cols = current_player.valid_cols();
        let col = handle_column_input(&stdin, &mut stdout, &mut input, &non_empty_cols)?;
        current_player
            .add_dice_roll(col, dice_roll)
            .expect("How even");
        game_state.end_turn();
    }
    Ok(())
}

fn handle_column_input(
    stdin: &Stdin,
    stdout: &mut Stdout,
    buffer: &mut String,
    valid_cols: &[usize],
) -> Result<usize, Box<dyn std::error::Error>> {
    loop {
        buffer.clear();
        stdin.read_line(buffer)?;
        match buffer.trim().parse::<usize>() {
            Ok(number) if valid_cols.contains(&number) => return Ok(number),
            Ok(_) | Err(_) => {
                writeln!(
                    stdout,
                    "Please only enter a number between 1 and 3 inclusive"
                )?;
                continue;
            }
        }
    }
}
