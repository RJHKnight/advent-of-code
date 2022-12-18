use std::{fs, collections::HashSet};
use itertools::Itertools;

fn main() {
    
    let contents = fs::read_to_string("input.txt")
        .expect("Should have been able to read the state file");

    let lines = contents.split("\r\n");

    let mut moves = Vec::new();

    for line in lines {
        let bits : Vec<&str> = line.split(" ").collect();

        moves.push(Move{
            direction: Direction::new(bits[0]),
            steps: bits[1].parse::<u16>().unwrap(),
        });
    }

    let mut tail_visited: HashSet<Position> = HashSet::new();

    let mut knots = Vec::new();

    for _i in 0..10 {
        knots.push(Position{row: 0, col: 0});
    }

    for this_move in moves {

        for _i in 0..this_move.steps {
            
            let mut head = &mut knots[0];
            // Move head
            match &this_move.direction {
                Direction::Up => head.row = head.row+1,
                Direction::Down => head.row = head.row-1,
                Direction::Left => head.col = head.col-1,
                Direction::Right => head.col = head.col+1,
                _ => panic!("Invalid head move."),
            }
            
            for i in 1..10 {

                let (start, end) = knots.split_at_mut(i);    
                let head = &mut start[i-1];
                let mut tail = &mut end[0];

                let this_move = get_move(head, tail);

                if this_move.is_none() {
                    break;
                }
                
                // Update tail
                if i16::abs_diff(head.col, tail.col) > 1 || i16::abs_diff(head.row, tail.row) > 1 {

                    match &this_move.unwrap() {
                        Direction::Up => {
                            tail.row = tail.row + 1;
                        },
                        Direction::Down => {
                            tail.row = tail.row - 1;
                        },
                        Direction::Left => {
                            tail.col = tail.col - 1;
                        },
                        Direction::Right => {
                            tail.col = tail.col + 1;
                        },
                        Direction::NE => {
                            tail.row = tail.row+1; tail.col = tail.col+1;
                        }, 
                        Direction::SE => {
                            tail.row = tail.row-1; tail.col = tail.col+1;
                        },
                        Direction::NW => {
                            tail.row = tail.row+1; tail.col = tail.col-1;
                        },
                        Direction::SW => {
                            tail.row = tail.row-1; tail.col = tail.col-1;
                        },
                    }
                        
                    
                }   
            }

            tail_visited.insert(knots[9].clone());
        }
        
        //println!("After move: {:?}", &this_move);
        //print_state(&knots);
    }
    println!("The tail visited {} locations.", tail_visited.len());

    //print_path(&tail_visited);

}

fn get_move(head: &Position, tail: &Position) -> Option<Direction> {

    if (head.row - tail.row).abs() < 2 && (head.col - tail.col).abs() < 2 {
        return None;
    }

    if head.row == tail.row {
        if head.col < tail.col {
            return Some(Direction::Left);
        }
        else {
            return Some(Direction::Right);
        }
    }
    else if head.col == tail.col {
        if head.row < tail.row {
            return Some(Direction::Down);
        }
        else {
            return Some(Direction::Up);
        }
    }
    else {
        // Up
        if head.row > tail.row {
            if head.col > tail.col {
                return Some(Direction::NE);
            }
            else {
                return Some(Direction::NW);
            }
        }
        // Down
        else {
            if head.col > tail.col {
                return Some(Direction::SE);
            }
            else {
                return Some(Direction::SW);
            }
        }
    }
}

fn print_path(tail_visited : &HashSet<Position>) {

    let min_max_row = tail_visited.iter().map(|x| x.row).minmax().into_option().unwrap();
    let min_max_col = tail_visited.iter().map(|x| x.col).minmax().into_option().unwrap();

    for i in min_max_row.0..min_max_row.1 {
        for j in min_max_col.0..min_max_col.1 {

            if i==0 && j == 0 {
                print!("s");
            }
            else {
                let mut found = false;
                for position in tail_visited {
                    if position.row == i && position.col == j {
                        print!("#");
                        found = true;
                        break;
                    }
                }

                if !found {
                    print!(".")
                }
            }
        }
        print!("\n");
    }

}

fn print_state(knots : &Vec<Position>) {

    // let min_max_row = knots.iter().map(|x| x.row).minmax().into_option().unwrap();
    // let min_max_col = knots.iter().map(|x| x.col).minmax().into_option().unwrap();
    let min_max_row = (-20, 20);
    let min_max_col = (-20, 20);

    for i in (min_max_row.0..min_max_row.1).rev() {
        for j in min_max_col.0..min_max_col.1 {

            if i==0 && j == 0 {
                print!("s");
            }
            else {
                let mut found = false;
                for k in 0..knots.len(){

                    if knots[k].row == i && knots[k].col == j {
                        print!("{}", k);
                        found = true;
                    }
                }

                if !found {
                    print!(".")
                }
            }
        }
        print!("\n");
    }

}

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
struct Position {
    row: i16,
    col: i16,
}

#[derive(Debug)]
struct Move {
    direction : Direction,
    steps : u16,
}

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    NE,
    NW,
    SE,
    SW,
}

impl Direction {

    fn new(from :&str) -> Direction {
        match from {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => panic!("Unknown direction"),
        }
    }
}