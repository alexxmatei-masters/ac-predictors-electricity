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

    // Print the lines stored in the vector
    for line in &lines {
        // println!("{}", line);
    }

    // Get the biggest value from the vector
    // let max_value = lines;

    let max_value = lines.iter().max_by(|&a, &b| a.partial_cmp(b).unwrap());
    // println!("{:?}", max_value);

    // Round and convert the elements to u16
    let rounded_numbers: Vec<u16> = lines.iter().map(|&num| num.round() as u16).collect();

    // Print the rounded numbers
    for num in &rounded_numbers {
        // println!("{}", num);
    }

    let max_value_i = rounded_numbers.iter().max();
    // println!("{:?}", max_value_i);

    // Calculate the split index
    let split_index = (rounded_numbers.len() as f32 * 0.8) as usize;

    // Split the vector into two parts
    let (training_data, testing_data) = rounded_numbers.split_at(split_index);

    // println!("Part 1: {:?}", training_data);
    // println!();
    // println!();
    // println!("Part 2: {:?}", testing_data);

    // create an empty hash map
    // can u8 overflow?
    let mut markov_dict: HashMap<Vec<u16>, Vec<(Vec<u16>, u16)>> = HashMap::new();

    println!("Training data:\n");
    // println!(" # | Patterns | States");
    println!("    # |      Patterns      | State");

    const CONTEXT_SIZE: u8 = 3;
    for i in 0..training_data.len() - CONTEXT_SIZE as usize {
        print!("{:5} | ", i);
        let pattern = &training_data[i..i + CONTEXT_SIZE as usize].to_vec();
        let state =
            &training_data[i + CONTEXT_SIZE as usize..i + CONTEXT_SIZE as usize + 1].to_vec();
        print!("{:4?} | ", pattern);
        println!("{:?}", state);

        if let Some(value) = markov_dict.get_mut(pattern) {
            // println!("\nval {:?}", value);
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

    // println!("\nResult:");

    // // for element in markov_dict {
    // //     println!("{:?}", element);
    // // }
    // for values in markov_dict.values() {
    //     for element in values {
    //         println!("{:?}", element);
    //     }
    // }
    println!("\nResult:");

    for (key, values) in markov_dict.iter() {
        println!("Key: {:?}", key);
        for element in values {
            println!("Value: {:?}", element);
        }
    }

    // let key_to_find = vec![0, 0, 0];

    // println!("zeroz");
    // if let Some(values) = markov_dict.get(&key_to_find) {
    //     println!("Values for key {:?}:", key_to_find);

    //     let mut max_count = 0;
    //     let mut max_value: Option<&(Vec<u16>, u16)> = None;

    //     for value in values {
    //         if value.1 > max_count {
    //             max_count = value.1;
    //             max_value = Some(value);
    //         }
    //     }

    //     if let Some(value) = max_value {
    //         println!(
    //             "Key with highest occurrence: {:?}, count: {}",
    //             value.0[0], value.1
    //         );
    //     } else {
    //         println!("No values found for the key!");
    //     }
    // } else {
    //     println!("Key not found!");
    // }

    // if let Some(values) = markov_dict.get(&key_to_find) {
    //     println!("Values for key {:?}:", key_to_find);
    //     for value in values {
    //         println!("{:?}", value);
    //     }
    // } else {
    //     println!("Key not found!");
    // }

    println!();
    println!("Testing data:");
    println!("    # |      Patterns      | Real state | Predicted state | Error ");

    let mut previous_state = 0;
    let mut error_vector: Vec<u32> = Vec::new();
    for i in 0..testing_data.len() - CONTEXT_SIZE as usize {
        let mut predicted_state = 255;

        // let key_to_find: Vec<u16> = vec![0, 0, 0];

        // if markov_dict.contains_key(&key_to_find) {
        //     println!("Key found!");
        // } else {
        //     println!("Key not found!");
        // }

        // markov_dict.contains_key(k);
        // let markov_dict_keys: Vec<&[u16]> = markov_dict.keys().cloned().collect();

        // Print the keys
        // println!("Markov Dictionary Keys:");
        // for key in markov_dict {
        //     println!("{:?}", key);
        // }

        let pattern = &testing_data[i..i + CONTEXT_SIZE as usize];
        let state = &testing_data[i + CONTEXT_SIZE as usize..i + CONTEXT_SIZE as usize + 1];

        let key_to_find = pattern.to_vec();
        if let Some(values) = markov_dict.get(&key_to_find) {
            // println!("Values for key {:?}:", key_to_find);

            let mut max_count = 0;
            let mut max_value: Option<&(Vec<u16>, u16)> = None;

            for value in values {
                if value.1 > max_count {
                    max_count = value.1;
                    max_value = Some(value);
                }
            }

            if let Some(value) = max_value {
                // println!(
                //     "Key with highest occurrence: {:?}, count: {}",
                //     value.0[0], value.1
                // );
                // println!("\nHit for {:?}, predicted: {:?}", key_to_find, value);
                predicted_state = value.0[0];
            } else {
                // println!("No values found for the key!");
            }
        } else {
            // println!("Key not found!");
            predicted_state = previous_state;
        }
        print!("{:5} | ", i);
        print!("{:4?} | ", pattern);
        print!("{:4?}       | ", state[0]);
        print!("{:4?}            | ", predicted_state);
        let error = ((state[0] as i16 - predicted_state as i16) as i16).abs() as u32;
        println!("{:4?}             ", error);
        error_vector.push(error);

        // if let Some(value) = markov_dict.get_mut(pattern) {
        //     let mut new_elements: Vec<(&[u16], u16)> = Vec::new();
        //     let mut found_match = false;

        //     for &mut (tuple_state, ref mut count) in value.iter_mut() {
        //         if tuple_state == state {
        //             *count += 1;
        //             found_match = true;
        //         }
        //     }

        //     if !found_match {
        //         new_elements.push((state, 1));
        //     }

        //     if !new_elements.is_empty() {
        //         value.extend(new_elements);
        //     }
        // } else {
        //     let vect = vec![(state, 1 as u16)];
        //     markov_dict.insert(pattern, vect);
        // }
        previous_state = state[0];
    }

    let sum: u32 = error_vector.iter().sum();
    let count = error_vector.len();
    let mean = sum as f32 / count as f32;

    println!("Mean average: {}", mean);
}
