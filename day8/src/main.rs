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
    print!("Internal: {}, External: {}, Total: {}", num_internal, num_external, num_internal+num_external);

}

fn get_visible_tree_row(row: usize, direction: Direction, grid : &Vec<Vec<u32>>) -> Vec<Position> {

    let num_col = grid[0].len();
    
    let array:  Box<dyn Iterator<Item=usize>>;

    if direction == Direction::LeftToRight {
        array = Box::new(0..=(num_col-1));
    }
    else {
        array = Box::new((0..=(num_col-1)).rev());
    }

    let mut res = Vec::new();
    let mut max_size = 0;

    for i in array {
        if grid[row][i] > max_size {
            let position = Position{x: row, y: i};
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
        array = Box::new(0..=(num_row-1));
    }
    else {
        array = Box::new((0..=(num_row-1)).rev());
    }

    let mut res = Vec::new();
    let mut max_size = 0;

    for i in array {
        if grid[i][col] > max_size {
            let position = Position{x: i, y: col};
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
    x : usize,
    y : usize
}

impl  Position {
    
    fn is_edge(&self, max_x: usize, max_y: usize) -> bool {
        self.x == max_x-1 ||
        self.x == 0 ||
        self.y == max_y-1 ||
        self.y == 0
    }
}