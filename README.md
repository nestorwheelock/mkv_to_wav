
# MKV to WAV Converter

**MKV to WAV Converter** is a Rust-based command-line tool that converts `.mkv` files to `.wav` format. The tool uses `ffmpeg` for conversion, preserves the original timestamps of the files, and supports parallel processing for faster execution using the `rayon` crate.

## Features

- Converts `.mkv` files to `.wav` audio format.
- Uses `ffmpeg` for audio extraction.
- Preserves file timestamps (accessed and modified times).
- Supports parallel processing of files in a directory using `rayon`.
- Optionally removes the original `.mkv` files after processing.

## Requirements

- Rust (for building the tool)
- `ffmpeg` (for converting `.mkv` to `.wav`)
- `rayon`, `chrono`, and `filetime` Rust crates.

## Installation

1. **Clone the repository:**
   ```bash
   git clone https://github.com/your-username/mkv-to-wav.git
   cd mkv-to-wav
   ```

2. **Build the project using Cargo:**
   ```bash
   cargo build --release
   ```

3. **Ensure that `ffmpeg` is installed on your system:**
   - On macOS: `brew install ffmpeg`
   - On Ubuntu: `sudo apt install ffmpeg`
   - On Windows: [Download FFmpeg](https://ffmpeg.org/download.html)

## Usage

1. **Convert all `.mkv` files in the current directory:**
   Run the program without arguments to convert `.mkv` files in the current directory:
   ```bash
   ./target/release/mkv-to-wav
   ```

2. **Convert `.mkv` files in a specific directory:**
   You can also specify a directory:
   ```bash
   ./target/release/mkv-to-wav /path/to/directory
   ```

3. **Output Directory:**
   The converted `.wav` files will be stored in an `audio/` directory created in the current working directory.

4. **Removing Original Files:**
   The tool is currently configured to convert `.mkv` files but does **not** remove the original files automatically. You can uncomment the `remove_file` function call in the code if you'd like to remove the original `.mkv` files after conversion.

## Example

Assume you have a directory with the following `.mkv` files:

```text
video1.mkv
video2.mkv
video3.mkv
```

Run the following command:

```bash
./target/release/mkv-to-wav
```

The tool will convert these files and create corresponding `.wav` files in the `audio/` directory:

```text
audio/video1.wav
audio/video2.wav
audio/video3.wav
```

## Project Structure

- **`main.rs`**: The core logic that handles conversion, timestamp preservation, and parallel processing.
- **`Cargo.toml`**: Lists the required dependencies for the project, including `ffmpeg`, `rayon`, `chrono`, and `filetime`.

## Dependencies

This project uses the following Rust crates:

- [ffmpeg](https://ffmpeg.org): To convert `.mkv` files to `.wav`.
- [rayon](https://crates.io/crates/rayon): For parallel processing of files.
- [chrono](https://crates.io/crates/chrono): For managing and formatting timestamps.
- [filetime](https://crates.io/crates/filetime): For preserving file access and modification times.

## License

This project is licensed under the GNU GPLv3 License. See the [LICENSE](LICENSE) file for more details.
