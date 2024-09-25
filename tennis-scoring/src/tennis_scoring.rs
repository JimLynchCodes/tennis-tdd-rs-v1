const SCORE_NAMES: [&str; 4] = ["Love", "Fifteen", "Thirty", "Forty"];

#[derive(Default)]
pub struct TennisGame {
    pub score1: u8,
    pub score2: u8,
    pub player1_name: String,
    pub player2_name: String,
}

impl TennisGame {
    pub fn new() -> Self {
        TennisGame::default()
    }
}

impl TennisGame {
    pub fn set_names(&mut self, player1_name: String, player2_name: String) {
        self.player1_name = player1_name;
        self.player2_name = player2_name;
    }

    pub fn set_score(&mut self, score1: u8, score2: u8) {
        self.score1 = score1;
        self.score2 = score2;
    }

    pub fn get_score(&self) -> String {
        if let Some(player) = self.check_if_either_player_wins() {
            return format!("Win for {}", player);
        }

        if let Some(player) = self.check_if_either_player_has_advantage() {
            return format!("Advantage {}", player);
        }

        if let Some(score) = self.check_if_tied() {
            return score;
        }

        self.concatonate_scores()
    }

    fn check_if_either_player_wins(&self) -> Option<&str> {
        if (self.score1 == 4 && self.score2 <= 2)
            || (self.score1 > 4 && self.score1 >= (self.score2 + 2))
        {
            return Some(&self.player1_name);
        }

        if (self.score2 == 4 && self.score1 <= 2)
            || (self.score2 > 4 && self.score2 >= (self.score1 + 2))
        {
            return Some(&self.player2_name);
        }

        None
    }

    fn check_if_either_player_has_advantage(&self) -> Option<&str> {
        if self.score1 >= 3 && self.score2 >= 3 {
            if self.score1 == self.score2 + 1 {
                return Some(&self.player1_name);
            } else if self.score2 == self.score1 + 1 {
                return Some(&self.player2_name);
            }
        }

        None
    }

    fn check_if_tied(&self) -> Option<String> {
        if self.score1 == self.score2 {
            if self.score1 > 2 {
                return Some("Deuce".to_string());
            }

            let score1_name = SCORE_NAMES.get(self.score1 as usize).unwrap();
            return Some(format!("{}-All", score1_name));
        }

        None
    }

    fn concatonate_scores(&self) -> String {
        let score1_name = SCORE_NAMES.get(self.score1 as usize).unwrap();
        let score2_name = SCORE_NAMES.get(self.score2 as usize).unwrap();
        format!("{}-{}", score1_name, score2_name)
    }
}
