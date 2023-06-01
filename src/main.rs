use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file_path = "pv_electricity.txt"; // Replace with the actual file path

    // Open the file in read-only mode
    let file = match File::open(file_path) {
        Ok(file) => file,
        Err(error) => {
            eprintln!("Failed to open file: {}", error);
            return;
        }
    };

    // Create a buffered reader to read the file line by line
    let reader = BufReader::new(file);

    // Create a vector to store the lines
    let mut lines: Vec<f32> = Vec::new();

    // Read each line and add it to the vector
    for line in reader.lines() {
        if let Ok(line) = line {
            // Attempt to convert the string to an f32
            let number = match line.parse::<f32>() {
                Ok(parsed_number) => parsed_number,
                Err(_) => {
                    println!("Failed to parse the string as f32");
                    return;
                }
            };
            lines.push(number);
        } else {
            eprintln!("Failed to read line");
        }
    }

    // Round and convert the elements to u16
    let rounded_numbers: Vec<u16> = lines.iter().map(|&num| num.round() as u16).collect();

    /* Calculate the split index used to split the initial vector into 2
    Training data set - first 80% of the vector
    Testing data set  - the following 20% */
    let split_index = (rounded_numbers.len() as f32 * 0.8) as usize;

    // Split the vector into two parts
    let (training_data, testing_data) = rounded_numbers.split_at(split_index);

    // create an empty hash map
    let mut markov_dict: HashMap<Vec<u16>, Vec<(Vec<u16>, u16)>> = HashMap::new();

    println!("Training data:\n");
    println!("    # |      Patterns      | State");

    const CONTEXT_SIZE: u8 = 3;
    for i in 0..training_data.len() - CONTEXT_SIZE as usize {
        let pattern = &training_data[i..i + CONTEXT_SIZE as usize].to_vec();
        let state =
            &training_data[i + CONTEXT_SIZE as usize..i + CONTEXT_SIZE as usize + 1].to_vec();

        print!("{:5} | ", i);
        print!("{:4?} | ", pattern);
        println!("{:?}", state);

        if let Some(value) = markov_dict.get_mut(pattern) {
            let mut new_elements: Vec<(Vec<u16>, u16)> = Vec::new();
            let mut found_match = false;

            for (tuple_state, ref mut count) in value.iter_mut() {
                if tuple_state == state {
                    *count += 1;
                    found_match = true;
                }
            }

            if !found_match {
                new_elements.push((state.clone(), 1));
            }

            if !new_elements.is_empty() {
                value.extend(new_elements);
            }
        } else {
            let vect = vec![(state.clone(), 1 as u16)];
            markov_dict.insert(pattern.clone(), vect);
        }
    }

    println!();
    println!("Testing data:");
    println!("    # |      Patterns      | Real state | Predicted state | Error ");

    let mut previous_state = 0;
    let mut error_vector: Vec<u32> = Vec::new();
    for i in 0..testing_data.len() - CONTEXT_SIZE as usize {
        let mut predicted_state = 255;

        let pattern = &testing_data[i..i + CONTEXT_SIZE as usize];
        let state = &testing_data[i + CONTEXT_SIZE as usize..i + CONTEXT_SIZE as usize + 1];

        let key_to_find = pattern.to_vec();
        if let Some(values) = markov_dict.get(&key_to_find) {
            let mut max_count = 0;
            let mut max_value: Option<&(Vec<u16>, u16)> = None;

            for value in values {
                if value.1 > max_count {
                    max_count = value.1;
                    max_value = Some(value);
                }
            }

            /* If we find the key in the dictionary */
            if let Some(value) = max_value {
                predicted_state = value.0[0];
            }
        } else {
            /* If we don't find the key in the dictionary */
            predicted_state = previous_state;
        }
        let error = ((state[0] as i16 - predicted_state as i16) as i16).abs() as u32;

        print!("{:5} | ", i);
        print!("{:4?} | ", pattern);
        print!("{:4?}       | ", state[0]);
        print!("{:4?}            | ", predicted_state);
        println!("{:4?}             ", error);
        error_vector.push(error);

        previous_state = state[0];
    }

    let sum: u32 = error_vector.iter().sum();
    let count = error_vector.len();
    let mean = sum as f32 / count as f32;

    println!();
    println!("Mean average: {}", mean);
}
