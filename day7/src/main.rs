use std::{fs, collections::HashMap};
use crate::Command::{CD,LS};

fn main() {

    let mut contents = fs::read_to_string("input.txt")
        .expect("Should have been able to read the state file");

    contents = contents.replace("$ ", "");
    let lines = contents.split("\r\n");

    let mut cur_directory = DirectoryDetails{size: 0, dirs: Vec::new()};
    let mut cur_location = vec![""];
    let mut all_directories: HashMap<String, DirectoryDetails> = HashMap::new();

    for line in lines {

        if is_command(&line) {
            
            let command = Command::new(line);

            match command {
                LS => {},
                CD{to} => {
  
                    let path = get_path(&cur_location);
                    all_directories.insert(path, cur_directory);
                    
                    if to.eq("..") {
                        
                        cur_location.pop();
                        cur_directory = all_directories.get(&get_path(&cur_location)).unwrap().clone();
                    }
                    else {
                        cur_location.push(to);
                        cur_directory = DirectoryDetails{size: 0, dirs: Vec::new()};
                    }
                },
            }
        }
        else {
            let bits :Vec<&str> = line.split(" ").collect();
            
            if bits[0].contains("dir") {
                cur_directory.dirs.push(bits[1]);
            }
            else {
                let file_size = bits[0].parse::<usize>().unwrap();
                cur_directory.size += file_size;
            }
        }
    }

    all_directories.insert(get_path(&cur_location), cur_directory);

    // let mut total_folder_size = 0;

    // dbg!(&all_directories);

    // for folder in all_directories.keys() {
        
    //      let this_dir_size = get_recursive_size(folder, &all_directories);
    //      if this_dir_size <= 100000 {
    //         total_folder_size += this_dir_size;
    //      }
    // }

    // println!("Total Folder Size: {}", total_folder_size);

    let total_space_used = get_total_folder_size("/", &all_directories);
    let unused_space = 70000000 - total_space_used;
    let target_size = 30000000 - unused_space;

    // Find the smallest directory that is bigger than unused space.
    let mut dir_name = "";
    let mut dir_size = 9999999999999;

    for folder in all_directories.keys() {

        let this_dir_size = get_total_folder_size(folder, &all_directories);

        if this_dir_size > target_size && this_dir_size < dir_size {
            dir_name = folder;
            dir_size = this_dir_size;
        }
    }

    println!("You can delete dir {}, which has size {}.", dir_name, dir_size);
 

}

fn get_path(dirs: &Vec<&str>) -> String {
    
    let mut path = String::new();
    
    for this_string in dirs {
        path = format!("{}/{}", path, this_string);
    }   

    path
}

fn get_total_folder_size(this_dir :&str, dirs: &HashMap<String, DirectoryDetails>) -> usize {

    let mut size = 0;
    for folder in dirs.keys() {
        if folder.contains(this_dir) {
            size += dirs.get(folder).unwrap().size;
        }
    }

    size
}

#[derive(Eq, Hash, PartialEq, Debug, Clone)]
struct DirectoryDetails<'a> {
    size: usize,
    dirs: Vec<&'a str>,
}

fn is_command(the_line: &str) -> bool {
    the_line.starts_with("ls") || the_line.starts_with("cd")
}

enum Command<'a> {
    LS,
    CD{to: &'a str},
}

impl <'a> Command<'a> {
    
    fn new(the_string :&str) -> Command {
        
        let bits : Vec<&str> = (*the_string).split(" ").collect();

        if bits[0].starts_with("ls") {
            return Command::LS;
        }
        else if bits[0].starts_with("cd") {
            return  Command::CD { to: bits[1]};
        }

        panic!("Invalid command {}", the_string);
    }
}
