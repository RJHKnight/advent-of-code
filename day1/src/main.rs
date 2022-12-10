use std::fs;

fn main() {
    
    let contents = fs::read_to_string("input.txt")
        .expect("Cannot read input file 😫");

    let lines = contents.split("\n");

    let mut elfs = Vec::new();

    let mut id = 1;
    let mut elf_food = ElfFood{id: id, food: Vec::new()};

    for line in lines {
        if line.is_empty() || line.eq("\r") {
            elfs.push(elf_food);
            id = id + 1;
            elf_food = ElfFood{id: id, food: Vec::new()};
        }
        else {
            elf_food.food.push(line.replace("\r", "").parse::<u32>().unwrap());
        }
    }

    let mut max_size = 0;
    let mut max_size_id = 0;

    for elf in &elfs {
        let this_elf_food = elf.total_food();
        if this_elf_food > max_size {
            max_size = this_elf_food;
            max_size_id = elf.id;
        }
    }

    println!("Max size is {} for elf {}", max_size, max_size_id);

    elfs.sort_by_key(|e| e.total_food());
    elfs.reverse();

    let mut top_three_calories = 0;

    for i in 0..3 {
        let this_elf = &elfs[i];
        println!("Position {} elf is {} and has {} food.", i, this_elf.id, this_elf.total_food());
        top_three_calories += this_elf.total_food();
    }

    println!("Top 3 total food = {}", top_three_calories);
}

struct ElfFood {
    id: usize,
    food :Vec<u32>,
}

impl ElfFood {
    fn total_food(&self) -> u32 {
        self.food.iter().sum()
    }
}