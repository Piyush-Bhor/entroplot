use std::env;
use std::error::Error;

use entropyviz2::{calculate_entropy, draw_entropy_graph, read_file};

const BLOCK_SIZE: usize = 1024;

fn main() -> Result<(), Box<dyn Error>> {
    // Get file path from CLI
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    // Convert file contents to buffer
    let mut buf = read_file(file_path)?;

    // Calculate entropy of each block of the buffer
    let entropies = calculate_entropy(&mut buf, BLOCK_SIZE);

    // Draw a graph based on the entropies
    draw_entropy_graph(&entropies);

    Ok(())
}
