use std::{fs, collections::{HashSet, VecDeque}};

fn main() {
    let line = fs::read_to_string("input.txt").expect("Boo 👻");

    let mut buffer = VecDeque::new();

    let mut i = 1;
    let mut found = false;

    for entry in line.chars().into_iter() {

        if buffer.len() > 13 {
            buffer.pop_front();
        }

        if buffer.len() > 14 {
            panic!("Buffer is too long!");
        }

        buffer.push_back(entry);
        
        if buffer.len() == 14 {
            let unique_size = (&buffer).into_iter().collect::<HashSet<_>>().len();
            if unique_size == 14 {
                found = true;
                break;
            }
        }
        
        i = i+1;
    }

    if found {
        println!("Found after {} chars", i);
    }
    else {
        println!("Not found.")
    }
}
