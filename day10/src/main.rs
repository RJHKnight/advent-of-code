use std::fs;

fn main() {
    
    let contents = fs::read_to_string("input.txt")
        .expect("Noooo way...");
    
    let lines = contents.split("\r\n");

    let mut i=0;
    let mut register: i32 = 1;
    let mut signal_strength = 0;

    let mut instructions = Vec::new();

    for line in lines {

        let this_inst = Instruction::from_string(line);
        instructions.push(this_inst);
    }

    let mut visible = [false; 40*6];
    visible[0] = true;

    for this_inst in instructions {

        println!("Cycle: {}, Handling {:?}, register = {}", i, this_inst, register);

        match this_inst {
            Instruction::NoOp => {                 
                i += 1;
                if (i%40 - register).abs() < 2 {
                    visible[i as usize] = true;
                }

                if (i+20) % 40 == 0 {
                    let this_strength = i * register;
                    signal_strength += this_strength;
                    println!("At cycle {} - register = {}, strength = {}", i, register, this_strength);
                }

            },
            Instruction::AddX{to_add} => {

                for counter in 0..2 {
                    
                    i += 1;
                    
                    if counter == 1 {
                        register += to_add;
                    }                    

                    if (i%40 - register).abs() < 2 {
                        visible[i as usize] = true;
                    }
                    
                    if (i+20) % 40 == 0 {
                        let this_strength = i * register;
                        signal_strength += this_strength;
                        println!("At cycle {} - register = {}, strength = {}", i, register, this_strength);
                    }
                }
            }
        }
    }

    println!("Signal strength = {}", signal_strength);

    for i in 0..6 {
        for j in 0..40 {
            if visible[(i * 40) + j] {
                print!("#");
            }
            else {
                print!(".");
            }
        }
        print!("\n");
    }
    
}

#[derive(Debug)]
enum Instruction {
    AddX{to_add: i32},
    NoOp,
}

impl Instruction  {

    fn from_string(the_string :&str ) -> Instruction {
        if the_string.contains("noop") {
            return Instruction::NoOp;
        }
        else if the_string.contains("addx") {

            let bits: Vec<&str> = the_string.split(' ').collect();
            let to_add = bits.get(1).unwrap().parse::<i32>().unwrap();
            return Instruction::AddX {to_add};
        }

        panic!("Invalid instruction");
    }
}

