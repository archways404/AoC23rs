use std::fs;
use std::io;

fn main() -> io::Result<()> {
    // Read the engine schematic from a file (you can replace "input.txt" with your actual file)
    let contents = fs::read_to_string("test.txt")?;
    
    // Parse the contents into a 2D vector
    let mut schematic: Vec<Vec<char>> = Vec::new();
    for line in contents.lines() {
        schematic.push(line.chars().collect());
    }

    let rows = schematic.len();
    let cols = schematic[0].len();
    let mut part_numbers = vec![vec![false; cols]; rows];

    // Define the directions for 8 possible adjacent cells
    let directions = [
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1),         (0, 1),
        (1, -1), (1, 0), (1, 1),
    ];

    // Check for each symbol and mark adjacent numbers
    for i in 0..rows {
        for j in 0..cols {
            if !schematic[i][j].is_alphanumeric() && schematic[i][j] != '.' {
                // It's a symbol
                for &(dx, dy) in &directions {
                    let ni = i as isize + dx;
                    let nj = j as isize + dy;
                    if ni >= 0 && ni < rows as isize && nj >= 0 && nj < cols as isize {
                        let ni = ni as usize;
                        let nj = nj as usize;
                        if schematic[ni][nj].is_digit(10) {
                            part_numbers[ni][nj] = true;
                        }
                    }
                }
            }
        }
    }

    // Sum up the part numbers
    let mut sum = 0;
    for i in 0..rows {
        for j in 0..cols {
            if part_numbers[i][j] {
                if let Some(number) = schematic[i][j].to_digit(10) {
                    sum += number;
                }
            }
        }
    }

    println!("Sum of part numbers: {}", sum);

    Ok(())
}
