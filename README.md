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
- **Real-Time Speed Display**: Live speed measurements with animated progress indicators
- **Configurable Duration**: Set test duration from 1 to any number of seconds (default: 30s)
- **Multiple Units**: Display results in bps, Kbps, Mbps, or Gbps
- **Verbose Logging**: Debug mode for detailed test information
- **Colorful Output**: Easy-to-read colored terminal output with animated spinners

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

#### Test Duration

Use the `--duration` or `-t` flag to specify how long each test should run (in seconds):

```bash
speedtest-cli start --duration 60   # Run each test for 60 seconds
speedtest-cli download -t 15        # Run download test for 15 seconds
speedtest-cli start                 # Default: 30 seconds per test
```

- Default: 30 seconds
- Longer durations provide more accurate results
- Each test (download and upload) runs for the specified duration

#### Verbose Mode

Enable detailed logging with the `--verbose` or `-v` flag:

```bash
speedtest-cli start --verbose
speedtest-cli download -v
```

### Examples

```bash
# Quick download test with default settings (30 seconds)
speedtest-cli download

# Full test showing results in Kbps
speedtest-cli start -u kbps

# Upload test with verbose logging
speedtest-cli upload -v

# Download test in Gbps with debug info
speedtest-cli download --unit gbps --verbose

# 60-second test for more accurate results
speedtest-cli start --duration 60

# Quick 10-second download test in Mbps
speedtest-cli download -t 10 -u mbps

# Combined options: 45-second test with verbose logging
speedtest-cli start --duration 45 --unit kbps --verbose
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
⠹ Testing download speed - 45.23 Mbps
✔ Testing download speed
⠹ Testing upload speed - 24.87 Mbps
✔ Testing upload speed

Speed Test Results
===================
Download Speed: 48.42 Mbps
Upload Speed:   25.08 Mbps
```

The spinner (⠹) animates while testing and shows real-time speed updates every 100ms. When complete, it displays a checkmark (✔).

## Requirements

- Rust 1.70 or higher
- Active internet connection

## Dependencies

- `clap` - Command-line argument parsing
- `tokio` - Async runtime
- `reqwest` - HTTP client
- `env_logger` & `log` - Logging
- `colored` - Terminal colors
- `indicatif` - Progress bars and spinners
- `thiserror` - Error handling
- `futures-util` - Async utilities

## How It Works

The tool performs speed tests by:
- **Download**: Downloading test files from reliable servers and measuring transfer rate in real-time
- **Upload**: Uploading data to test endpoints and measuring transfer rate
- **Real-Time Updates**: Speed is calculated and displayed every 100ms during the test
- **Multiple Fallbacks**: Automatically tries alternative servers if one fails

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

- [x] Real-time progress indicators with animated spinners
- [ ] Add server selection options
- [ ] Support for latency/ping tests
- [ ] JSON output format
- [ ] Configuration file support
- [ ] Historical test results tracking

---

Made with ❤️ and Rust
