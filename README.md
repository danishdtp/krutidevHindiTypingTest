# Kruti Hindi Typing Practice

A desktop application for practicing Hindi typing using the Kruti Dev font.

## Features

- **Typing Practice** - Practice Hindi typing with random words
- **Multiple Durations** - Choose from 15 seconds, 30 seconds, or 1 minute tests
- **Real-time Stats** - View WPM (words per minute) and accuracy in real-time
- **Score Tracking** - Automatic saving of test results to SQLite database
- **Best Score** - Track your personal best WPM
- **Custom Word Lists** - Load your own Hindi word files (.txt)
- **Hindi Font** - Built-in support for Kruti Dev 010 font
<img width="618" height="607" alt="image" src="https://github.com/user-attachments/assets/abbe4312-9cd8-4e90-9d7f-5dd716152ca4" />

## Requirements

- Rust 1.56 or higher
- Kruti Dev 010 font file (included as `Kruti Dev 010 Regular.ttf`)

## Build from Source

### Prerequisites

- Rust 1.56 or higher (install via [rustup](https://rustup.rs/))
- Platform-specific dependencies:

#### Linux
- `libudev-dev`, `libssl-dev`, `pkg-config`
- On Fedora: `dnf install udev-devel openssl-devel pkg-config`
- On Ubuntu/Debian: `apt install libudev-dev libssl-dev pkg-config`

#### macOS
- Xcode Command Line Tools: `xcode-select --install`

#### Windows
- MinGW-w64 or Visual Studio Build Tools

### Build Commands

```bash
# Clone the repository
git clone <repository-url>
cd krutiHindiTyping

# Debug build
cargo build

# Release build (optimized, smaller binary)
cargo build --release
```

### Pre-built Binaries

Pre-built executables are available in the `releases/` folder:

| Platform | File | Notes |
|----------|------|-------|
| Linux | `releases/kruti-typing-linux` | x86_64 |
| Windows | `releases/kruti-typing.exe` | Build on Windows |
| macOS | `releases/kruti-typing-macos` | Build on macOS |

For Windows/macOS, clone and build on the target platform for best compatibility.

### Running

After building:
```bash
# Debug
cargo run

# Release
./target/release/kruti-typing
```

## Usage
2. Select a duration (15s, 30s, or 1m)
3. Click on the input field and start typing
4. Type the Hindi words shown in the display area
5. Your WPM and accuracy update in real-time
6. When time expires, your score is saved automatically

### Controls

- **Duration Buttons** - Switch between 15s, 30s, 1m test durations
- **Load File** - Load a custom Hindi word list (.txt file)
- **Stop Test** - End the current test early
- **Reset Test** - Reset and generate new words

## Database

Scores are stored in `typing_stats.db` (SQLite). The database tracks:
- WPM (words per minute)
- Accuracy (percentage)
- Date/time of test

## Project Structure

```
krutiHindiTyping/
├── src/
│   ├── main.rs      # Entry point
│   ├── app.rs       # Main application logic
│   ├── models.rs    # Data models
│   └── db.rs        # Database operations
├── hindi-words.txt  # Default word list
├── Kruti Dev 010 Regular.ttf  # Hindi font
├── Cargo.toml       # Rust dependencies
└── README.md        # This file
```

## Dependencies

- **iced** - GUI framework
- **rusqlite** - SQLite database
- **native-dialog** - Native file dialogs
- **rand** - Random word selection
- **chrono** - Date/time handling
- **serde** - Serialization

## License

MIT License

## Release Notes

### Version 0.1.0
- Initial release
- Hindi typing practice with Kruti Dev font
- Multiple duration options (15s, 30s, 1m)
- Real-time WPM and accuracy tracking
- SQLite-based score history
- Custom word list loading support
