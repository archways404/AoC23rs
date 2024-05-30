
use std::fs;
use std::io;

fn main() -> io::Result<()> {
    // Read the file contents (you can replace "temp.txt" with your actual file)
    let contents = fs::read_to_string("input.txt")?;
    
    // Vector to store points for each line
    
    //let mut points: Vec<u32> = Vec::new();
    
    for line in contents.lines() {
        let mut parts = line.split(' ');
        let left_side = parts.next().unwrap().trim();
        let right_side = parts.next().unwrap().trim();
        
        println!("Left side: {:?}", left_side);
        println!("Right side: {:?}", right_side);
        
        let hands: Vec<u32> = left_side
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();
            
        
        let bets: Vec<u32> = right_side
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();
        
        println!("Hands: {:?}", hands);
        println!("Bets: {:?}", bets);
    }

    Ok(())
}


