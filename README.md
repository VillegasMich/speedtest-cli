# speedtest-cli

[![Crates.io](https://img.shields.io/crates/v/speedtest-cli.svg)](https://crates.io/crates/speedtest-cli)
[![Downloads](https://img.shields.io/crates/d/speedtest-cli.svg)](https://crates.io/crates/speedtest-cli)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![CI](https://github.com/VillegasMich/speedtest-cli/workflows/CI/badge.svg)](https://github.com/VillegasMich/speedtest-cli/actions)

A fast and simple command-line tool to test your internet connection speed, written in Rust.

📦 [View on crates.io](https://crates.io/crates/speedtest-cli)

## Features

- **Download Speed Test**: Measure your download speed
- **Upload Speed Test**: Measure your upload speed
- **Full Speed Test**: Run both download and upload tests
- **Multiple Units**: Display results in bps, Kbps, Mbps, or Gbps
- **Verbose Logging**: Debug mode for detailed test information
- **Colorful Output**: Easy-to-read colored terminal output

## Installation

### From crates.io

```bash
cargo install speedtest-cli
```

### From source

Clone and build:
```bash
git clone https://github.com/VillegasMich/speedtest-cli.git
cd speedtest-cli
cargo build --release
```

The binary will be available at `target/release/speedtest-cli`

### Install directly to PATH

Build and install in one step:
```bash
git clone https://github.com/VillegasMich/speedtest-cli.git
cd speedtest-cli
cargo install --path .
```

This will compile the binary and install it to `~/.cargo/bin/` (which should be in your PATH). After installation, you can run `speedtest-cli` from anywhere.

To uninstall:
```bash
cargo uninstall speedtest-cli
```

## Usage

### Basic Commands

Run a full speed test (download + upload):
```bash
speedtest-cli start
```

Test download speed only:
```bash
speedtest-cli download
```

Test upload speed only:
```bash
speedtest-cli upload
```

### Options

#### Unit Selection

Use the `--unit` or `-u` flag to specify the output unit:

```bash
speedtest-cli start --unit mbps    # Megabits per second (default)
speedtest-cli download -u kbps     # Kilobits per second
speedtest-cli upload --unit gbps   # Gigabits per second
speedtest-cli start -u bps         # Bits per second
```

Available units:
- `bps` - bits per second
- `kbps` - kilobits per second
- `mbps` - megabits per second (default)
- `gbps` - gigabits per second

#### Verbose Mode

Enable detailed logging with the `--verbose` or `-v` flag:

```bash
speedtest-cli start --verbose
speedtest-cli download -v
```

### Examples

```bash
# Quick download test with default settings
speedtest-cli download

# Full test showing results in Kbps
speedtest-cli start -u kbps

# Upload test with verbose logging
speedtest-cli upload -v

# Download test in Gbps with debug info
speedtest-cli download --unit gbps --verbose
```

### Help

Get help for any command:
```bash
speedtest-cli --help
speedtest-cli start --help
speedtest-cli download --help
speedtest-cli upload --help
```

## Sample Output

```
=> Running full speed test (download + upload)...

Speed Test Results
===================
Download Speed: 48.42 Mbps
Upload Speed:   25.08 Mbps
```

## Requirements

- Rust 1.70 or higher
- Active internet connection

## Dependencies

- `clap` - Command-line argument parsing
- `tokio` - Async runtime
- `reqwest` - HTTP client
- `env_logger` & `log` - Logging
- `colored` - Terminal colors
- `thiserror` - Error handling
- `futures-util` - Async utilities

## How It Works

The tool performs speed tests by:
- **Download**: Downloading test files from reliable servers and measuring transfer rate
- **Upload**: Uploading data to test endpoints and measuring transfer rate

Results are calculated in bytes per second and converted to your chosen unit.

## License

MIT

## Contributing

Contributions are welcome! We appreciate your help in making speedtest-cli better.

### How to Contribute

1. **Fork the repository**
   - Go to https://github.com/VillegasMich/speedtest-cli
   - Click the "Fork" button in the top right
   - This creates a copy of the repository in your GitHub account

2. **Clone your fork**
   ```bash
   git clone https://github.com/YOUR-USERNAME/speedtest-cli.git
   cd speedtest-cli
   ```

3. **Create a new branch**
   ```bash
   git checkout -b feature/your-feature-name
   ```

4. **Make your changes**
   - Write clean, readable code
   - Follow Rust best practices and idioms
   - Add tests for new functionality
   - Update documentation as needed

5. **Test your changes**
   ```bash
   cargo test
   cargo clippy -- -D warnings
   cargo fmt -- --check
   ```

6. **Commit your changes**
   ```bash
   git add .
   git commit -m "feat: add your feature description"
   ```

7. **Push to your fork**
   ```bash
   git push origin feature/your-feature-name
   ```

8. **Open a Pull Request**
   - Go to your fork on GitHub
   - Click "Compare & pull request"
   - Select the base repository: `VillegasMich/speedtest-cli` and base branch: `main`
   - Describe your changes clearly
   - Reference any related issues
   - Submit the pull request

### Guidelines

- **Code Style**: Follow the existing code style and run `cargo fmt` before committing
- **Tests**: Add unit tests for new features and bug fixes
- **Documentation**: Update README and code comments when needed
- **Commit Messages**: Use clear, descriptive commit messages (e.g., `feat:`, `fix:`, `docs:`)
- **CI**: Ensure all CI checks pass before requesting review

### Reporting Issues

Found a bug or have a feature request? Please [open an issue](https://github.com/VillegasMich/speedtest-cli/issues) with:
- A clear description of the problem or feature
- Steps to reproduce (for bugs)
- Expected vs actual behavior
- Your environment (OS, Rust version, etc.)

### Code of Conduct

Be respectful and constructive in all interactions. We're here to build something great together!

## Roadmap

- [ ] Add server selection options
- [ ] Support for latency/ping tests
- [ ] JSON output format
- [ ] Configuration file support
- [ ] Progress bars for long tests

---

Made with ❤️ and Rust
