use std::fs;
use std::io::Read;
use std::{thread, time::Duration, io::Write};

fn main() {
    let story = read_file("story.txt").unwrap();
    typewriter(story.to_string(), 50)
}


fn read_file(filename: &str) -> Result<String, std::io::Error> {
    // Open the file in read-only mode
    let mut file = fs::File::open(filename)?;

    // Read the contents of the file into a string
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

fn typewriter(text: String, delay: u64) {
    let mut total_delay = 0;
    for word in text.split_whitespace() {
        for c in word.chars() {
            print!("{}", c);
            std::io::stdout().flush().unwrap();
            total_delay += delay;
            thread::sleep(Duration::from_millis(delay));
        }
        print!(" ");
        std::io::stdout().flush().unwrap();
        total_delay += delay;
        thread::sleep(Duration::from_millis(delay));
    }
    println!();
}
