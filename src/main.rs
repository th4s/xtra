use log::info;
use simplelog::{ColorChoice, Config, LevelFilter, TermLogger, TerminalMode};
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

    let _logger = if args[4].as_str() != "-" {
        Some(TermLogger::init(
            LevelFilter::Info,
            Config::default(),
            TerminalMode::Stdout,
            ColorChoice::Auto,
        ))
    } else {
        None
    };

    if block_part.is_none() || block_numbers.is_none() {
        println!("Invalid input. Please supply a valid combination of arguments\n");
        print_info();
        return;
    }
    let block_part = block_part.unwrap();
    let block_numbers = block_numbers.unwrap();

    // Build index
    let index = block_part
        .init(ancient_folder, block_numbers.0, block_numbers.1)
        .expect("Failed to build index");

    // Load all data files into RAM
    let _ = write_target.write_all(b"[\n");
    for job in index.offsets {
        let data = block_part
            .load_data(ancient_folder, job.0, &job.1)
            .expect("Unable to load data files");
        let output = match block_part {
            Freezer::Bodies => block_part.export_json::<BlockBody>(&job.1, &data),
            Freezer::Headers => block_part.export_json::<BlockHeader>(&job.1, &data),
            Freezer::Hashes => block_part.export_json::<BlockHash>(&job.1, &data),
            Freezer::Difficulty => block_part.export_json::<TotalDifficulty>(&job.1, &data),
            Freezer::Receipts => block_part.export_json::<Receipts>(&job.1, &data),
        };
        info!("Writing to hard disk...");
        let _ = write_target.write_all(output.expect("Unable to export data").as_bytes());
        info!("Done!");
    }
    let _ = write_target.write_all(b"]");
    info!("Finished successfully!");
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
    println!(
        r#"
Usage: xtra FOLDER MODE BLOCK_RANGE OUTPUT

FOLDER              the geth freezer folder, usually chaindata/ancient

MODE
    b, body         export block bodies
    h, header       export block headers
    d, difficulty   export total difficulty
    hash            export block hashes
    r, receipt      export transaction receipts

BLOCK_RANGE
    number          export the single block with this number
    number-number   export the block range

OUTPUT
    -               print to stdout
    file            write to file
    "#
    );
}
