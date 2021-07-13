use std::path::Path;

use xtralib::types::{BlockBody, BlockHash, BlockHeader, Receipts, TotalDifficulty};
use xtralib::Freezer;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 5 || ["--help", "-help", "-h", "h"].contains(&args[1].as_str()) {
        print_info();
        return;
    }

    let ancient_folder = Path::new(&args[1]);
    let block_part = parse_block_part(&args[2]);
    let block_numbers = parse_block_numbers(&args[3]);
    let mut write_target: Box<dyn std::io::Write> = match args[4].as_str() {
        "-" => Box::new(std::io::stdout()),
        file => Box::new(std::fs::File::create(file).expect("Cannot create file")),
    };

    if block_part.is_none() || block_numbers.is_none() {
        println!("Invalid input. Please supply a valid combination of arguments");
        print_info();
        return;
    }
    let block_part = block_part.unwrap();
    let block_numbers = block_numbers.unwrap();

    let output = load_data(&ancient_folder, block_part, block_numbers);

    write_target.write_all(output.as_bytes()).unwrap();
}

fn parse_block_numbers(block_numbers: &str) -> Option<(u64, u64)> {
    let block_numbers = block_numbers
        .split('-')
        .map(|x| x.parse::<u64>())
        .collect::<Result<Vec<u64>, std::num::ParseIntError>>();

    if let Ok(block_numbers) = block_numbers {
        return match block_numbers[..] {
            [a, b] => Some((a, b)),
            [a] => Some((a, a + 1)),
            _ => None,
        };
    }
    None
}

fn parse_block_part(block_part: &str) -> Option<Freezer> {
    match block_part {
        "b" | "body" => Some(Freezer::Bodies),
        "h" | "header" => Some(Freezer::Headers),
        "d" | "difficulty" => Some(Freezer::Difficulty),
        "hash" => Some(Freezer::Hashes),
        "r" | "receipt" => Some(Freezer::Receipts),
        _ => None,
    }
}

fn print_info() {
    println!("Help Text");
}

// Yep, that is ugly...
fn load_data(ancient_folder: &Path, block_part: Freezer, block_numbers: (u64, u64)) -> String {
    match block_part {
        Freezer::Bodies => Freezer::Bodies
            .load::<BlockBody>(ancient_folder, block_numbers.0, block_numbers.1)
            .unwrap()
            .to_string(),
        Freezer::Headers => Freezer::Headers
            .load::<BlockHeader>(ancient_folder, block_numbers.0, block_numbers.1)
            .unwrap()
            .to_string(),
        Freezer::Hashes => Freezer::Hashes
            .load::<BlockHash>(ancient_folder, block_numbers.0, block_numbers.1)
            .unwrap()
            .to_string(),
        Freezer::Difficulty => Freezer::Difficulty
            .load::<TotalDifficulty>(ancient_folder, block_numbers.0, block_numbers.1)
            .unwrap()
            .to_string(),
        Freezer::Receipts => Freezer::Receipts
            .load::<Receipts>(ancient_folder, block_numbers.0, block_numbers.1)
            .unwrap()
            .to_string(),
    }
}
