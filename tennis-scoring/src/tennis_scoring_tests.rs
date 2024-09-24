
use crate::tennis_scoring::TennisGame;

#[test]
fn test_defaults() {
    let game = TennisGame::new();
    assert_eq!(game.score1, 0);
    assert_eq!(game.score2, 0);
    assert_eq!(game.player1_name, "".to_string());
    assert_eq!(game.player2_name, "".to_string());
}

#[test]
fn love_all() {
    let game = TennisGame::new();
    assert_eq!(game.get_score(), "Love-All");
}

#[test]
fn love_fifteen() {
    let mut game = TennisGame::new();

    let new_game = game.set_score(0, 1);

    assert_eq!(new_game.get_score(), "Love-Fifteen".to_string());
}
