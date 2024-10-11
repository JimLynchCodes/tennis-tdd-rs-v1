#[cfg(test)]
mod tennis_scoring_tests {

    use crate::tennis_scoring::TennisGame;
    use rstest::*;

    #[test]
    fn test_defaults() {
        let game = TennisGame::new();
        assert_eq!(game.score1, 0);
        assert_eq!(game.score2, 0);
        assert_eq!(game.player1_name, "".to_string());
        assert_eq!(game.player2_name, "".to_string());
    }

    #[test]
    fn test_sets_score() {
        let mut game = TennisGame::new();
        game.set_score(1, 2);
        assert_eq!(game.score1, 1);
        assert_eq!(game.score2, 2);
    }

    #[test]
    fn test_sets_names() {
        let mut game = TennisGame::new();
        game.set_names("bob".to_string(), "jim".to_string());
        assert_eq!(game.player1_name, "bob");
        assert_eq!(game.player2_name, "jim");
    }

    #[rstest]
    #[case(0, 0, "Love-All")]
    #[case(1, 1, "Fifteen-All")]
    #[case(2, 2, "Thirty-All")]
    #[case(4, 0, "Win for player1")]
    #[case(0, 4, "Win for player2")]
    #[case(4, 1, "Win for player1")]
    #[case(1, 4, "Win for player2")]
    #[case(4, 2, "Win for player1")]
    #[case(2, 4, "Win for player2")]
    #[case(6, 4, "Win for player1")]
    #[case(4, 6, "Win for player2")]
    #[case(8, 6, "Win for player1")]
    #[case(18, 16, "Win for player1")]
    #[case(6, 8, "Win for player2")]
    #[case(16, 18, "Win for player2")]
    #[case(4, 3, "Advantage player1")]
    #[case(3, 4, "Advantage player2")]
    #[case(5, 4, "Advantage player1")]
    #[case(4, 5, "Advantage player2")]
    #[case(6, 5, "Advantage player1")]
    #[case(5, 6, "Advantage player2")]
    #[case(16, 15, "Advantage player1")]
    #[case(15, 16, "Advantage player2")]
    #[case(1, 0, "Fifteen-Love")]
    #[case(0, 1, "Love-Fifteen")]
    #[case(2, 0, "Thirty-Love")]
    #[case(0, 2, "Love-Thirty")]
    #[case(3, 0, "Forty-Love")]
    #[case(0, 3, "Love-Forty")]
    #[case(1, 2, "Fifteen-Thirty")]
    #[case(2, 1, "Thirty-Fifteen")]
    #[case(1, 3, "Fifteen-Forty")]
    #[case(3, 1, "Forty-Fifteen")]
    #[case(2, 3, "Thirty-Forty")]
    #[case(3, 2, "Forty-Thirty")]
    #[case(4, 4, "Deuce")]
    #[case(5, 5, "Deuce")]
    #[case(6, 6, "Deuce")]
    #[case(8, 8, "Deuce")]
    #[case(42, 42, "Deuce")]
    fn check_score(#[case] score1: u8, #[case] score2: u8, #[case] expected_result: &str) {
        let mut game = TennisGame::new();

        game.set_names("player1".to_string(), "player2".to_string());
        game.set_score(score1, score2);

        assert_eq!(game.get_score(), expected_result);
    }
}
