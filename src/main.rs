use std::{fs,collections::HashMap};
use std::collections::hash_map::Entry::{Occupied, Vacant};

fn main() {
    
    // Initial State
    let path = "initial_state.txt";

    let contents = fs::read_to_string(path)
        .expect("Should have been able to read the state file");

    let mut state = parse_initial_state(&contents);

    // Instructions
    let path = "instructions.txt";

    let contents = fs::read_to_string(path)
        .expect("Should have been able to read the instructions file");
        
    let instructions = parse_instructions(&contents);

    for instruction in instructions {
        println!("Handling: Move {} from {} to {}", instruction.num, instruction.from, instruction.to);
        state.move_crates(instruction.from, instruction.to, instruction.num);
    }

    state.print_top();
}

fn parse_initial_state(contents :&str) -> CargoState {
    
    let mut state = CargoState{cargo_state : HashMap::new()};

    let lines = contents.split("\n");

    for line in lines {
        state.add_row(line);
    }

    state
}

fn parse_instructions(contents :&str) -> Vec<Instruction> {
    let mut instructions = Vec::new();
    
    let lines = contents.split("\n");

    for line in lines {
        let bits :Vec<&str> = line.split(" ").collect();
        instructions.push(Instruction { 
            num: bits[1].parse::<usize>().unwrap(), 
            from: bits[3].parse::<u16>().unwrap(), 
            to: bits[5].parse::<u16>().unwrap()});
    }

    instructions
}

struct Instruction {
    from: u16,
    to: u16,
    num: usize
}


struct CargoState {
    cargo_state :HashMap<u16, CargoColumn>,
}

impl CargoState {

    pub fn add_row(&mut self, new_row :&str) {

        let bits = new_row.split("\t");

        let mut i = 1;

        for bit in bits {

            if bit.len() > 0 {
                
                let cargo_col = match self.cargo_state.entry(i) {
                    Vacant(entry) => entry.insert(CargoColumn{cargo_column: Vec::new()}),
                    Occupied(entry) => entry.into_mut(),
                };

                cargo_col.add(bit.chars().next().unwrap());
            }

            i = i+1;
        }
    }

    fn print_top(&self) {

            for i in 1..=self.cargo_state.len() {
                let col = self.cargo_state.get(&(i as u16)).unwrap();
                print!("{} ", col.get_top());

            }
        }

    pub fn move_crates(&mut self, from : u16, to: u16, size: usize) {

        let removed = self.cargo_state.get_mut(&from).unwrap().remove(size);

        for to_add in removed.iter().rev() {
            self.cargo_state.get_mut(&to).unwrap().add_top(*to_add);
        }
    }

}

struct CargoColumn {
    cargo_column :Vec<char>,
}

impl CargoColumn {
    fn add(&mut self, entry: char) {
        self.cargo_column.push(entry);
    }

    fn add_top(&mut self, entry: char) {
        self.cargo_column.insert(0, entry);
    }

    fn remove(&mut self, size: usize) -> Vec<char> {

        dbg!(self.cargo_column.len());
        if self.cargo_column.len() > size {
            let mut removed = vec![' '; size];
            removed.clone_from_slice(&self.cargo_column[0..size]);

            for i in 0..size {
                self.cargo_column.remove(0);
            }

            return removed;
        }

        let cloned = self.cargo_column.clone();
        self.cargo_column = Vec::new();

        cloned
    }

    fn get_top(&self) -> &char {
        self.cargo_column.first().unwrap()
    }
 }