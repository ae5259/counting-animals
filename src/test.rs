use std::collections::HashMap;
use std::thread;

fn main() {
    let mut map = HashMap::new();

    loop {
        thread::sleep(std::time::Duration::from_secs(1));

        let word = String::from("animal");
        // Get a mutable reference to the entry's value, or insert 0 if vacant
        let count = map.entry(word).or_insert(0);
        *count += 1; // Dereference the mutable reference and increment
        //
        println!("Word counts: {:?}", map);
    }
}
