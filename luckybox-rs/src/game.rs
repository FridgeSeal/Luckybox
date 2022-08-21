use itertools::Itertools;
use rand::{self, rngs::ThreadRng, Rng};

pub struct GameState {
    pub players: [Player; 2],
    player_idx: usize,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            players: std::array::from_fn(|_| Player::new()),
            player_idx: 0,
        }
    }

    pub fn is_complete(&self) -> bool {
        self.players.iter().any(|p| p.array_full())
    }

    pub fn get_current_player(&mut self) -> Option<&mut Player> {
        self.players.get_mut(self.player_idx)
    }

    // pub fn get_non_current_player(&mut self) -> Option<&mut Player> {
    //     self.players.get_mut(self.get_other_player_idx())
    // }

    fn get_other_player_idx(&self) -> usize {
        (self.player_idx + 1) % 2
    }

    fn swap_current_player(&mut self) {
        self.player_idx = self.get_other_player_idx();
    }

    pub fn end_turn(&mut self) {
        self.swap_current_player();
    }
}

#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub score_arrays: [[usize; 3]; 3],
}

impl Player {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            score_arrays: [[0; 3]; 3],
        }
    }

    pub fn add_name(&mut self, name: &str) {
        self.name = name.to_owned();
    }

    pub fn add_dice_roll(&mut self, column: usize, value: usize) -> Result<(), bool> {
        if let Some(first_idx) = self.score_arrays[column].iter().position(|&p| p == 0) {
            self.score_arrays[column][first_idx] = value;
            Ok(())
        } else {
            Err(false)
        }
    }

    fn col_has_space(&self, col: [usize; 3]) -> bool {
        col.contains(&0)
    }

    pub fn valid_cols(&self) -> Vec<usize> {
        self.score_arrays
            .into_iter()
            .enumerate()
            .filter(|(_i, col)| self.col_has_space(*col))
            .map(|(i, _)| i)
            .collect_vec()
    }

    fn array_full(&self) -> bool {
        self.valid_cols().is_empty()
    }

    pub fn pretty_print_score_row(&self, row: usize) -> [usize; 3] {
        [
            self.score_arrays[0][row],
            self.score_arrays[1][row],
            self.score_arrays[2][row],
        ]
    }

    pub fn add_score_array(&mut self, idx: usize, new_score_arr: [usize; 3]) {
        self.score_arrays[idx] = new_score_arr;
    }

    fn calculate_column_score(column: [usize; 3]) -> usize {
        let mut sorted_col = column.clone();
        sorted_col.sort_unstable();
        sorted_col
            .iter()
            .dedup_with_count()
            .map(|(count, item)| item * (count.pow(2)))
            .sum()
    }

    pub fn calculate_all_col_scores(&self) -> [usize; 3] {
        self.score_arrays.map(|s| Player::calculate_column_score(s))
    }
    fn calculate_total_score(&self) -> usize {
        self.calculate_all_col_scores().iter().sum()
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn calculates_column_score_correctly() {
        let column: [usize; 3] = [3, 3, 3];
        let expected_score: usize = 27;
        let actual_score = Player::calculate_column_score(column);
        assert_eq!(expected_score, actual_score)
    }

    #[test]
    fn calculates_column_score_correctly_all_zeroes() {
        let column: [usize; 3] = [0, 0, 0];
        let expected_score: usize = 0;
        let actual_score = Player::calculate_column_score(column);
        assert_eq!(expected_score, actual_score)
    }

    #[test]
    fn calculates_all_scores_correctly() {
        let mut p = Player::new();
        p.add_score_array(0, [1, 0, 0]);
        p.add_score_array(1, [2, 2, 5]);
        p.add_score_array(2, [6, 1, 3]);
        let expected_score_arr = [1, 13, 10];
        let actual_score_arr = p.calculate_all_col_scores();
        assert_eq!(expected_score_arr, actual_score_arr);
    }

    #[test]
    fn calculates_total_scores_correctly() {
        let mut p = Player::new();
        p.add_score_array(0, [1, 0, 0]);
        p.add_score_array(1, [2, 2, 5]);
        p.add_score_array(2, [6, 1, 3]);
        let expected_total_score = 24; // Not sure the score logic is right. Double check
        let actual_total_score = p.calculate_total_score();
        assert_eq!(expected_total_score, actual_total_score);
    }
}

pub struct Dice {
    rng: ThreadRng,
}

impl Dice {
    pub fn new() -> Self {
        Self {
            rng: rand::thread_rng(),
        }
    }

    pub fn roll(&mut self) -> usize {
        self.rng.gen_range(1..=6)
    }
}
