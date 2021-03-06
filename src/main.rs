use std::io;

mod pinyin_to_normal;

fn main() {
    loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(len) => if len == 0 {
                return;
            } else {
                let result = pinyin_to_normal::pinyin_to_normal(input);
                print!("{}", result);
            } 
            Err(error) => {
                eprintln!("error: {}", error);
                return;
            }
        }
    }
}
