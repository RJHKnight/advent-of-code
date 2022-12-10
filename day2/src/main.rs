use std::fs;

fn main() {
    // Initial State
    let path = "input.txt";

    let contents = fs::read_to_string(path)
        .expect("Should have been able to read the input file 😴");

    let lines = contents.split("\n");

    let mut total_score = 0;

    for line in lines {

        let new_line = line.replace("\r", "");
        let mut bits = new_line.split(" ");
        let their_move = move_from_string(bits.next().unwrap());
        let result  = result_from_string(bits.next().unwrap());
        let our_move = get_required_move(&their_move, &result);
        let this_score = get_score(our_move, &their_move);

        print!(" Score = {} \n", &this_score);
        total_score += this_score;
    }

    println!("Total score is {}.", total_score);

}

#[derive(PartialEq, Debug, Clone)]
enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

fn move_from_string(input :&str) -> Move {

    if input == "A" || input == "X" {
        return Move::Rock;
    }
    else if input == "B" || input == "Y" {
        return Move::Paper;
    }
    else if input == "C" || input == "Z" {
        return Move::Scissors;
    }

    panic!("Invalid character - {}!!", input);
}

fn result_from_string(input :&str) -> Result {
    
    if input == "X" {
        return Result::Loss;
    }
    else if input == "Y" {
        return Result::Draw;
    }
    else if input == "Z" {
        return Result::Win;
    }

    panic!("Invalid character - {}!!", input);
}

#[derive(PartialEq, Debug)]
enum Result {
    Win,
    Loss,
    Draw,
}

fn get_result(our_move :&Move, their_move :&Move) -> Result {
    
    if our_move == their_move {
        return Result::Draw;
    }

    if (our_move == &Move::Rock && their_move == &Move::Scissors) || 
       (our_move == &Move::Scissors && their_move == &Move::Paper) ||
       (our_move == &Move::Paper && their_move == &Move::Rock) {
        return Result::Win;
    }

    Result::Loss

}

fn get_required_move<'a>(their_move :&'a Move, result :&'a Result) -> &'a Move {

    if result == &Result::Draw {
        return their_move;
    }
    else if result == &Result::Win {
        let our_move = match their_move {
            Move::Paper => &Move::Scissors,
            Move::Rock => &Move::Paper,
            Move::Scissors => &Move::Rock,
        };

        return our_move;
    }
    else {
        let our_move = match their_move {
            Move::Paper => &Move::Rock,
            Move::Rock => &Move::Scissors,
            Move::Scissors => &Move::Paper,
        };

        return our_move;
    }

}

fn get_score(our_move :&Move, their_move :&Move) -> u32 {

    let result = get_result(our_move, their_move);

    print!("{:?} vs. {:?} - result is {:?}", our_move, their_move, &result);

    let move_score = our_move.clone() as u32;
    match result {
        Result::Win => 6 + move_score,
        Result::Draw => 3 + move_score,
        Result::Loss => 0 + move_score,
    }
}