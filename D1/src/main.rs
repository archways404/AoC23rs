use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let contents = fs::read_to_string("input.txt")?;
    //let contents = fs::read_to_string("test.txt")?;
    //println!("File contents:\n{}", contents);
    let mut my_vector = Vec::new();
    for line in contents.lines() {
        println!("{}", line);


        let numbers: Vec<u32> = line.chars()
        .filter(|c| c.is_numeric())
        .filter_map(|c| c.to_digit(10))
        .collect();

        println!("{:?}", numbers);
        let numlen = numbers.len();

        if numlen < 2 {
            if numlen == 1 {
              println!("Only one number found");
              let numfirst = numbers[0];
              println!("numfirst: {}", numfirst);
              let concatenated_string: String = numfirst.to_string() + &numfirst.to_string();
              // If you need the result as an integer
              let concatenated_number: i32 = concatenated_string.parse().unwrap();
              println!("Concatenated number: {}", concatenated_number);
              my_vector.push(concatenated_number);
              continue;
            } else {
              println!("More than one number found or no numbers found");
              continue;
            }
        } else {

          println!("numlen: {}", numlen);
          let numlast = numbers[numlen-1];
          let numfirst = numbers[0];
          println!("numfirst: {}", numfirst);
          println!("numlast: {}", numlast);

          let concatenated_string: String = numfirst.to_string() + &numlast.to_string();
          // If you need the result as an integer
          let concatenated_number: i32 = concatenated_string.parse().unwrap();
          println!("Concatenated number: {}", concatenated_number);
          my_vector.push(concatenated_number);

        }
    }

    println!("Vector: {:?}", my_vector);
    let total_sum: i32 = my_vector.iter().sum();
    
    println!("Total Sum: {}", total_sum);

    Ok(())
    // 12, 38, 15, and 77. Adding these together produces 142.
}
