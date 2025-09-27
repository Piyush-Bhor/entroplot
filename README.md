# Entroplot

![Rust](https://img.shields.io/badge/language-Rust-orange)
![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)

> **`entroplot` is a Rust-based command-line tool for analyzing and visualizing file entropy.**  
> It helps reverse engineers, security researchers, and developers uncover patterns, randomness, or structure in binary data.

## ✨ Features

- 📂 Analyze the entropy of any file
- 📈 Generate entropy plots to visualize randomness and structure
- 🛠️ Simple CLI interface

## 🚀 Installation

Clone the repository and build with Cargo:

```bash
git clone https://github.com/Piyush-Bhor/entroplot.git
cd entroplot
cargo build --release
```

The compiled binary will be available at `target/release/entroplot`.

## 🖥️ Usage

Basic usage:

```bash
entroplot <file>
```

Example:

```bash
entroplot ./samples/malware.bin
```

This generates a plot (`entropy_chart.png` by default) showing the entropy distribution of the file.

### Output file customization

```bash
entroplot ./samples/malware.bin --output-file /path/to/output.png
# or shorthand
entroplot ./samples/malware.bin -o /path/to/output.png
```

### Help

```bash
entroplot --help
```

```
Generate entropy plots

Usage: entroplot [OPTIONS] <INPUT_FILE>

Arguments:
  <INPUT_FILE>  The path to the input file whose entropy will be calculated

Options:
  -o, --output-file <OUTPUT_FILE>  Path to save the entropy chart [default: entropy_chart.png]
  -h, --help                       Print help
  -V, --version                    Print version
```

## 📊 Example Output

![Entropy Plot](./assets/entropy_chart.png)

## 🧪 Development

Run tests with:

```bash
cargo test
```

## 🛣️ Roadmap

- [ ] Add interactive entropy visualization
- [ ] Support additional file formats
- [ ] Export results to JSON/CSV
- [ ] Include more statistical measures beyond Shannon entropy

## 🤝 Contributing

Contributions, bug reports, and feature requests are welcome!
Please open an [issue](https://github.com/Piyush-Bhor/entroplot/issues) or submit a pull request.

## 📜 License

This project is licensed under the **GNU GPL v3**.
See the [LICENSE](./LICENSE) file for details.

---

**Made with ❤️ by [Piyush Bhor](https://github.com/Piyush-Bhor)**
