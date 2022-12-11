use std::{fs, collections::HashSet};

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Waaahhh... 😭");


    let lines = contents.split("\r\n");

    let mut grid = Vec::new();

    for line in lines {
        let mut row = Vec::new();

        for this_char in line.chars() {
            row.push(this_char.to_digit(10).expect("That ain't no number..."));
        }

        grid.push(row);
    }

    let num_row = grid.len();
    let num_col = grid[0].len();

    let mut visible_trees: Vec<Position> = Vec::new();

    // First for rows
    for i in 1..(num_row-1) {
        visible_trees.append(&mut get_visible_tree_row(i, Direction::LeftToRight, &grid));
        visible_trees.append(&mut get_visible_tree_row(i, Direction::RightToLeft, &grid));
    }


    // Then for columns 
    for i in 1..(num_col-1) {
        visible_trees.append(&mut get_visible_tree_col(i, Direction::Down, &grid));
        visible_trees.append(&mut get_visible_tree_col(i, Direction::Up, &grid));
    }

    let unique_trees: HashSet<Position> = HashSet::from_iter(visible_trees);

    let num_external = 2 * num_row + (2 * (num_col -2));
    let num_internal = unique_trees.len();
    println!("Internal: {}, External: {}, Total: {}", num_internal, num_external, num_internal+num_external);

    let mut max_score = 0;

    // for tree in unique_trees {
       
    //     let this_score = get_visible_tree(&tree, Direction::Down, &grid) *
    //             get_visible_tree(&tree, Direction::Up, &grid) *
    //             get_visible_tree(&tree, Direction::LeftToRight, &grid) *
    //             get_visible_tree(&tree, Direction::RightToLeft, &grid);
                
    //     if this_score > max_score {
    //         println!("Score for {:?} is {}", tree, this_score);
    //         max_score = this_score;
    //     }
    // }

    for i in 1..num_row-1 {
        for j in 1..num_col-1 {

                let tree = Position{row: i, col: j};
                let this_score = get_visible_tree(&tree, Direction::Down, &grid) *
                    get_visible_tree(&tree, Direction::Up, &grid) *
                    get_visible_tree(&tree, Direction::LeftToRight, &grid) *
                    get_visible_tree(&tree, Direction::RightToLeft, &grid);
            
            println!("Score for {:?} is {}", tree, this_score);

            if this_score > max_score {
                max_score = this_score;
            }
        }
    }


    println!("Max Score: {}", max_score);

}

fn get_visible_tree(position: &Position, direction: Direction, grid : &Vec<Vec<u32>>) -> usize {
    

    if (position.row == 14 && position.col == 52) {
        println!("TOOOY");
    }

    let num_col = grid[0].len();
    let num_row = grid.len();
        
    let array: Box<dyn Iterator<Item=usize>> = match direction {
        Direction::Down => Box::new(position.row+1..num_row),
        Direction::Up => Box::new((0..position.row).rev()),
        Direction::LeftToRight => Box::new(position.col+1..num_col),
        Direction::RightToLeft => Box::new((0..position.col).rev()),
    };

    let mut count = 0;
    let my_size = grid[position.row][position.col];
    let is_up_down = direction == Direction::Up || direction == Direction::Down;

    for i in array {

        let this_value = if is_up_down { grid[i][position.col]} else { grid[position.row][i]};
        count = count + 1;
        
        if this_value >= my_size {
            break;
        }
    }

    //println!("Visible trees for {:?} in Direction {:?} is {}", position, direction, count);
    count

}
 
fn get_visible_tree_row(row: usize, direction: Direction, grid : &Vec<Vec<u32>>) -> Vec<Position> {

    let num_col = grid[0].len();
    
    let array:  Box<dyn Iterator<Item=usize>>;

    if direction == Direction::LeftToRight {
        array = Box::new(0..num_col);
    }
    else {
        array = Box::new((0..num_col).rev());
    }

    let mut res = Vec::new();
    let mut max_size = 0;

    for i in array {
        if grid[row][i] > max_size {
            let position = Position{row: row, col: i};
            res.push(position);
            max_size = grid[row][i];
        }
    }   

    // Remove edges
    let res_filt = res.iter().filter(|x| !x.is_edge(grid.len(), num_col)).cloned().collect();

    res_filt
}

fn get_visible_tree_col(col: usize, direction: Direction, grid : &Vec<Vec<u32>>) -> Vec<Position> {

    let num_row = grid.len();
        
    let array:  Box<dyn Iterator<Item=usize>>;

    if direction == Direction::Down {
        array = Box::new(0..num_row);
    }
    else {
        array = Box::new((0..num_row).rev());
    }

    let mut res = Vec::new();
    let mut max_size = 0;

    for i in array {
        if grid[i][col] > max_size {
            let position = Position{row: i, col: col};
            res.push(position);
            max_size = grid[i][col];
        }
    } 
    
    let res_filt = res.iter().filter(|x| !x.is_edge(num_row, grid[0].len())).cloned().collect();
    res_filt

}

#[derive(PartialEq, Eq, Hash, Debug)]
enum Direction {
    LeftToRight,
    RightToLeft,
    Up,
    Down,
}


#[derive(PartialEq, Eq, Hash, Debug, Clone)]
struct Position {
    row : usize,
    col : usize
}

impl  Position {
    
    fn is_edge(&self, max_row: usize, max_col: usize) -> bool {
        self.row == max_row-1 ||
        self.row == 0 ||
        self.col == max_col-1 ||
        self.col == 0
    }
}