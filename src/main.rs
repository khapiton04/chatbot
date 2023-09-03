use markov::Chain;

use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

fn process(input: String, chain: &mut Chain<String>) -> Result<(), Box<dyn Error>> {
    if let Some(filename) = input.strip_prefix("save ") {
        let ser = bincode::serialize(&chain)?;
        let mut fileser = OpenOptions::new()
            .truncate(true)
            .create(true)
            .write(true)
            .read(true)
            .open(filename)?;
        fileser.write_all(&ser)?;
    } else if let Some(filename) = input.strip_prefix("load ") {
        let mut ser = Vec::new();
        File::open(filename)?.read_to_end(&mut ser)?;
        *chain = bincode::deserialize(&ser)?;
    } else if input == "?" {
        println!("\nchatbot > {}", chain.generate_str());
    } else if let Some(input) = input.strip_prefix('?') {
        println!(
            "\nchatbot > {}\n",
            chain.generate_str_from_token(input.trim())
        );
    } else if let Some(input) = input.strip_prefix('/') {
        chain.feed_str(input);
    } else if input == "commands" {
        println!("\n/(текст..) - Накормить ИИ\n? - Сгенерировать\n?(текст..) - Сгенерировать из текста\nload (filename) - Загрузить ИИ\nsave (filename) - Сохранить ИИ\n");
    } else if input == "hi" {
        println!("\nchatbot > Hello!\n");
    } else if input == "hello" {
        println!("\nchatbot > Hello!\n");
    } else if input == "how are you?" {
        println!("\nchatbot > Fine, and you?\n");
    } else if input == "good" {
        println!("\nchatbot > :)\n");
    } else {
        println!("\nchatbot > I'm sorry! For now this command is incorrect for me!\n");
    }
    Ok(())
}



fn main() {
    let mut chain: Chain<String> = Chain::of_order(1);
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        input.pop();
        process(input, &mut chain).unwrap();
    }
}
