use entropy::shannon_entropy;
use plotters::prelude::*;
use std::fs::File;
use std::io::Read;

/// Reads a file and returns it buffer.
pub fn read_file(file_path: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut file = File::open(file_path)?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;

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
