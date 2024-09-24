
#[derive(Default)]
pub struct TennisGame {
    pub score1: u8,
    pub score2: u8,
    pub player1_name: String,
    pub player2_name: String
}

impl TennisGame {
    pub fn new() -> Self {
        TennisGame::default()
    }
}

// Functions to implement
impl TennisGame {

    pub fn clear(&mut self) {

    }

    pub fn won_point(&mut self, player_name: &str) {
        
    }

    pub fn set_score(&mut self, score1: u8, score2: u8) -> &mut Self {
        self.score1 = score1;
        self.score2 = score2;

        return self;
    }

    pub fn get_score(&self) -> String {

        if self.score1 == 0 && self.score2 == 1 {
            return "Love-Fifteen".to_string()
        }

        "Love-All".to_string()
    }

}