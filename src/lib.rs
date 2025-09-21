use entropy::shannon_entropy;
use std::fs::File;
use std::io::Read;

pub fn read_file(file_path: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut file = File::open(file_path)?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;

    Ok(buf)
}

pub fn calculate_entropy(buf: &[u8], block_size: usize) -> Vec<f32> {
    buf.chunks(block_size)
        .map(|chunk| shannon_entropy(chunk))
        .collect()
}

pub fn draw_entropy_graph(entropies: &[f32]) {
    for &h in entropies {
        let bars = (h.round() as usize).min(8);
        println!("{:.2} | {}", h, "█".repeat(bars));
    }
}
