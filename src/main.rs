use std::collections::HashMap;

fn main() {
    const CONTEXT_SIZE: u8 = 3;
    const TRAINING_DATA: &str = "ABCDABCDABCEABC";
    let mut markov_dict: HashMap<&str, Vec<(&str, u8)>> = HashMap::new(); // create an empty hash map
    println!("Training data:\n");
    println!(" # | Patterns | States");
    for i in 0..TRAINING_DATA.len() - CONTEXT_SIZE as usize {
        print!("{:2} | ", i);
        let pattern: &str = &TRAINING_DATA[i..i + CONTEXT_SIZE as usize];
        let state: &str = &TRAINING_DATA[i + 3..i + 4];
        print!("{:?}    | ", pattern);
        println!("{:?}", state);
        if let Some(value) = markov_dict.get_mut(pattern) {
            let mut new_elements = Vec::new();

            for &mut (tuple_state, ref mut count) in value.iter_mut() {
                if tuple_state == state {
                    *count += 1;
                } else {
                    new_elements.push((state, 1));
                }
            }

            value.extend(new_elements);
        } else {
            let vect = vec![(state, 1 as u8)];
            markov_dict.insert(pattern, vect);
        }
    }

    println!("\nResult:");
    println!("{:?}", markov_dict);
}
