use std::fs;
use array_tool::vec::Intersect;

fn main() {
    
    let contents = fs::read_to_string("input.txt")
        .expect("Should have been able to read the state file");

    let lines = contents.split("\n");
    let mut score = 0;

    let mut elf_group = Vec::new();
    let mut i=0;

    for line in lines {

        let new_line = line.replace("\r", "");
        let mut this_chars = Vec::new();

        for char in new_line.chars() {
            this_chars.push(char);
        }

        elf_group.push(this_chars);
        i = i+1;

        if i == 3 {

            let intersect = elf_group[0].intersect(elf_group[1].to_owned()).intersect(elf_group[2].to_owned());  
            score += get_score(&intersect);
            i = 0;
            elf_group = Vec::new();
        }

    }
    
    println!("Total score = {}", score);

}

fn get_score(chars : &Vec<char>) -> u32 {
    chars.into_iter()
        .map(|c| c.to_owned())
        .map(|c| (if c.is_ascii_lowercase() {c as u32 - 96} else {c as u32 - 38}))
        .sum()
}

#[test]
fn test_score() {
    assert_eq!(get_score(&vec!['a']), 1);
    assert_eq!(get_score(&vec!['b']), 2);
    assert_eq!(get_score(&vec!['c']), 3);
    assert_eq!(get_score(&vec!['d']), 4);

    assert_eq!(get_score(&vec!['A']), 27);
    assert_eq!(get_score(&vec!['B']), 28);
    assert_eq!(get_score(&vec!['C']), 29);
    assert_eq!(get_score(&vec!['D']), 30);
}