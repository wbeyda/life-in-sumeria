use std::fs;

pub fn read_file(filename: &str) -> Result<String, std::io::Error> {
    let mut file = fs::File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

