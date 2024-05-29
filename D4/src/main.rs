/* 
use std::fs;
use std::io;

fn main() -> io::Result<()> {
    // Read the file contents
    //let contents = fs::read_to_string("input.txt")?;
    let contents = fs::read_to_string("temp.txt")?;
    
    for line in contents.lines() {
        let mut parts = line.split('|');
        let left_side = parts.next().unwrap().trim();
        let right_side = parts.next().unwrap().trim();
        
        let winning_numbers: Vec<u32> = left_side
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();
        
        let lot_numbers: Vec<u32> = right_side
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();
        
        println!("Winning numbers: {:?}", winning_numbers);
        println!("Lot numbers: {:?}", lot_numbers);
    }

    Ok(())
}
*/

use std::fs;
use std::io;

fn main() -> io::Result<()> {
    // Read the file contents (you can replace "temp.txt" with your actual file)
    let contents = fs::read_to_string("input.txt")?;
    
    // Vector to store points for each line
    let mut points: Vec<u32> = Vec::new();
    
    for line in contents.lines() {
        let mut parts = line.split('|');
        let left_side = parts.next().unwrap().trim();
        let right_side = parts.next().unwrap().trim();
        
        let winning_numbers: Vec<u32> = left_side
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();
        
        let lot_numbers: Vec<u32> = right_side
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();
        
        println!("Winning numbers: {:?}", winning_numbers);
        println!("Lot numbers: {:?}", lot_numbers);
        
        // Counter for the number of matches
        let mut match_count = 0;
        
        // Check for matches and update the counter
        for &number in &winning_numbers {
            if lot_numbers.contains(&number) {
                if match_count == 0 {
                    match_count += 1;
                } else {
                    match_count *= 2;
                }
            }
        }
        
        // Store the match count in the points vector
        points.push(match_count);
        
        // Print the match count for this line
        println!("Matches found: {}", match_count);
    }

    // Print the total points for all lines
    println!("Points: {:?}", points);

    let mut total_points = 0;

    for point in points {
        total_points += point;
    }

    println!("Total points: {}", total_points);

    Ok(())
}


