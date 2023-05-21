use std::fs::read_to_string;

fn main() {
    aoc2();
}

fn aoc2() {
    // A, X ROCK        1 point
    // B, Y PAPER       2 points
    // C, Z SCISSORS    3 points
    // WIN 6 points
    // LOSE 0 points
    // DRAW 3 points
    let game_input = read_to_string("input2.txt").expect("Failed to read input file");
    // split by newline into a vector
    let games: Vec<&str> = game_input.lines().collect();
    println!("Games: {:?}", games);
    // calculate the score for each game
    let mut scores: Vec<i32> = Vec::new();
    for game in &games {
        let mut score = 0;
        // get the player moves
        let moves: Vec<&str> = game.split(" ").collect();
        println!("Moves: {:?}", moves);
        let player1_move = moves[0];
        let player2_move = moves[1];
        // calculate the score
        match player2_move {
            "X" => {
                score += 1;
                match player1_move {
                    "A" => score += 3,
                    "B" => score += 0,
                    "C" => score += 6,
                    _ => println!("Invalid move: {}", player1_move),
                }
            }
            "Y" => {
                score += 2;
                match player1_move {
                    "A" => score += 6,
                    "B" => score += 3,
                    "C" => score += 0,
                    _ => println!("Invalid move: {}", player1_move),
                }
            }
            "Z" => {
                score += 3;
                match player1_move {
                    "A" => score += 0,
                    "B" => score += 6,
                    "C" => score += 3,
                    _ => println!("Invalid move: {}", player1_move),
                }
            }
            _ => println!("Invalid move: {}", player2_move),
        }
        scores.push(score);
    }
    println!("Scores: {:?}", scores);
    // calculate the total score
    let mut total_score = 0;
    for score in scores {
        total_score += score;
    }
    println!("Total score: {}", total_score);

    scores = Vec::new();
    for game in games {
        let mut score = 0;
        // get the player move and outcome
        let moves: Vec<&str> = game.split(" ").collect();
        println!("Moves: {:?}", moves);
        let player1_move = moves[0];
        let game_outcome = moves[1];
        // calculate the score
        match game_outcome {
            "X" => {
                score += 0;
                match player1_move {
                    "A" => score += 3,
                    "B" => score += 1,
                    "C" => score += 2,
                    _ => println!("Invalid move: {}", player1_move),
                }
            }
            "Y" => {
                score += 3;
                match player1_move {
                    "A" => score += 1,
                    "B" => score += 2,
                    "C" => score += 3,
                    _ => println!("Invalid move: {}", player1_move),
                }
            }
            "Z" => {
                score += 6;
                match player1_move {
                    "A" => score += 2,
                    "B" => score += 3,
                    "C" => score += 1,
                    _ => println!("Invalid move: {}", player1_move),
                }
            }
            _ => println!("Invalid move: {}", game_outcome),
        }
        scores.push(score);
    }
    println!("Scores: {:?}", scores);
    // calculate the total score
    total_score = 0;
    for score in scores {
        total_score += score;
    }
    println!("Total score: {}", total_score);
}
