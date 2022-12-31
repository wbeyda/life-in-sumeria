use std::{thread, time::{Duration, Instant}, io::Write, panic};

pub fn typewriter(text: String, delay: u64) {
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

#[warn(unused_imports)]
mod test {
    use super::*;

    #[test]
    //#[ignore]
//    fn test_typewriter() {
//        let mut output = Vec::new();
//        {
//            use std::io::Write;
//            let mut handle = std::io::Cursor::new(&mut output);
//            let mut stdout = std::io::stdout();
//            panic::set_hook(Box::new(move |info| handle.write_all(info.message().as_bytes()).unwrap()));
//
//            typewriter("Hello, world!".to_string(), 50);
//
//            panic::set_hook(Box::new(move |info| stdout.write_all(info.message().as_bytes()).unwrap()));
//        }
//        assert_eq!(output, b"Hello, world!\n");
//    }

    fn test_typewriter_elapsed_time() {
        let start = Instant::now();
        typewriter("Hello, world!".to_string(), 50);
        let elapsed = start.elapsed();
        assert!(elapsed < Duration::from_millis(1000));
    }



}

