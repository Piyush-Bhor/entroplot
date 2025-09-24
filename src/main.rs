use std::{env, error::Error, process};

use entropyviz2::{calculate_entropy, draw_entropy_chart, read_file};

const BLOCK_SIZE: usize = 1024;

fn main() -> Result<(), Box<dyn Error>> {
    // Get file path from CLI
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("[Error] Missing positional argument: 'file_path'");
        process::exit(1);
    }

    let file_path = &args[1];
    let output_path = if args.len() > 2 {
        &args[2]
    } else {
        "entropy_chart.png"
    };

    // Convert file contents to buffer
    let mut buf = read_file(file_path)?;

    // Calculate entropy of each block of the buffer
    let entropies = calculate_entropy(&mut buf, BLOCK_SIZE);

    // Draw a chart based on the entropies
    draw_entropy_chart(&entropies, output_path)?;
    println!("Entropy chart successfully written to {output_path}.");

    Ok(())
}
