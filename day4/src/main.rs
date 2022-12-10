use std::fs;

fn main() {

    let contents = fs::read_to_string("input.txt")
        .expect("Should have been able to read the input file");

    let lines = contents.split("\r\n");

    let mut counter=0;

    for line in lines {
        let mut bits = line.split(",").into_iter();
        let job_1 = job::new(bits.next().unwrap());
        let job_2 = job::new(bits.next().unwrap());

        if job_1.overlaps(&job_2) {
            counter = counter+1;
        }
    }

    println!("Total counter = {}", counter);
}

struct job {
    lower: u32,
    upper: u32,
}

impl job {

    fn new(details :&str) -> job {
        
        let mut bits = details.split("-").into_iter();
        job {
            lower: bits.next().unwrap().parse::<u32>().unwrap(),
            upper: bits.next().unwrap().parse::<u32>().unwrap(),
        }
    }

    fn contains(&self, other : &job) -> bool {

        self.upper >= other.upper && self.lower <= other.lower ||
        other.upper >= self.upper && other.lower <= self.lower
    }

    fn overlaps(&self, other : &job) -> bool {
        
        (self.upper <= other.upper && self.upper >= other.lower) ||
        (self.lower <= other.upper && self.lower >= other.lower) ||
        (other.upper <= self.upper && other.upper >= self.lower) ||
        (other.lower <= self.upper && other.lower >= self.lower) 
    }
}
