use anyhow::Context;
use clap::Parser;
use entropy::shannon_entropy;
use plotters::prelude::*;
use std::fs::File;
use std::io::Read;

#[derive(Parser)]
#[command(
    name = "entroplot",
    version = "0.1.0",
    about = "Generate entropy plots"
)]
pub struct Cli {
    /// The path to the input file whose entropy will be calculated.
    pub input_file: String,

    /// The path where the generated entropy chart will be saved.
    #[arg(short, long, default_value = "entropy_chart.png")]
    pub output_file: String,
}

/// Reads a file and returns it buffer.
pub fn read_file(file_path: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut file =
        File::open(file_path).with_context(|| format!("Failed to open file: `{}`.", file_path))?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)
        .with_context(|| format!("Failed to read contents of file: `{}`", file_path))?;

    Ok(buf)
}

/// Calaculates the Shannon Entropy of a buffer in chunks.
pub fn calculate_entropy(buf: &[u8], block_size: usize) -> Vec<f32> {
    buf.chunks(block_size)
        .map(|chunk| shannon_entropy(chunk))
        .collect()
}

/// Draws a line chart based on the Shannon entropies of each chunk of a file buffer.
pub fn draw_entropy_chart(
    entropies: &[f32],
    output: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Create drawing area
    let root = BitMapBackend::new(output, (800, 600)).into_drawing_area();
    root.fill(&BLACK)?;

    // Find min and max values for scaling
    let max_entropy = entropies.iter().cloned().fold(f32::MIN, f32::max).max(0.1); // avoid division by zero
    let min_entropy = entropies.iter().cloned().fold(f32::MAX, f32::min).min(0.0);

    // Build the chart
    let mut chart = ChartBuilder::on(&root)
        .caption("Entropy Chart", ("monospace", 30).into_font().color(&GREEN))
        .margin(20)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(
            0..entropies.len(),             // x = chunk index
            min_entropy..max_entropy + 0.5, // y = entropy range
        )?;

    chart
        .configure_mesh()
        .label_style(("monospace", 15).into_font().color(&GREEN))
        .axis_style(&GREEN)
        .light_line_style(&GREEN.mix(0.2))
        .x_desc("Block Index")
        .y_desc("Entropy")
        .draw()?;

    // Draw line graph
    chart.draw_series(LineSeries::new(
        entropies.iter().enumerate().map(|(i, &h)| (i, h)),
        &GREEN,
    ))?;

    // Save chart
    root.present()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculate_entropy_single_block() {
        // Buffer with low entropy
        let buf = vec![0u8; 16]; // all zeros
        let entropies = calculate_entropy(&buf, 1024);

        assert_eq!(entropies.len(), 1); // single block
        assert!((entropies[0] - 0.0).abs() < 1e-6); // entropy should be ~0
    }

    #[test]
    fn test_calculate_entropy_empty_buffer() {
        let buf: Vec<u8> = Vec::new();
        let entropies = calculate_entropy(&buf, 1024);
        assert!(entropies.is_empty());
    }
}
