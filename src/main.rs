use clap::Parser;
use std::error::Error;

use entropyviz2::{Cli, calculate_entropy, draw_entropy_chart, read_file};

const BLOCK_SIZE: usize = 1024;

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    // Convert file contents to buffer
    let mut buf = read_file(&cli.input_file)?;

    // Calculate entropy of each block of the buffer
    let entropies = calculate_entropy(&mut buf, BLOCK_SIZE);

    // Draw a chart based on the entropies
    draw_entropy_chart(&entropies, &cli.output_file)?;
    println!(
        "Entropy chart successfully written to {}.",
        &cli.output_file
    );

    Ok(())
}
