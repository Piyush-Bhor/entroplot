use std::fs::File;
use std::io::Read;

// Algorithm to calculate Shannon entropy
fn entropy(data: &[u8]) -> f64 {
    let mut counts = [0usize; 256];
    for &b in data {
        counts[b as usize] += 1;
    }
    let len = data.len() as f64;
    counts
        .iter()
        .filter(|&&c| c > 0)
        .map(|&c| {
            let p = c as f64 / len;
            -p * p.log2()
        })
        .sum()
}

pub fn read_file(file_path: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut file = File::open(file_path)?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;

    Ok(buf)
}

pub fn calculate_entropy(buf: &[u8], block_size: usize) -> Vec<f64> {
    buf.chunks(block_size).map(|chunk| entropy(chunk)).collect()
}

pub fn draw_entropy_graph(entropies: &[f64]) {
    for &h in entropies {
        let bars = (h.round() as usize).min(8);
        println!("{:.2} | {}", h, "█".repeat(bars));
    }
}
