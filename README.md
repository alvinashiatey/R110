# R110

A simple image channel splitter for RISO printing. R110 separates images into CMYK channels and allows you to assign custom RISO ink colors to each channel, making it easy to prepare artwork for Risograph printing.

![R110 Screenshot](https://github.com/alvinashiatey/R110/raw/main/static/screenshot.png)

## Features

- **CMYK Channel Separation** – Automatically splits images into Cyan, Magenta, Yellow, and Black channels
- **RISO Color Mapping** – Assign authentic RISO ink colors to each channel
- **Live Preview** – See your color-mapped channels composited in real-time
- **Multiple Export Options** – Export as individual PNG files or a multi-page PDF
- **Image Effects** – Apply dithering, halftone, and threshold effects

## Installation

### macOS (Homebrew)

```bash
brew install --cask alvinashiatey/tap/r110
```

### Manual Download

Download the latest release from the [Releases page](https://github.com/alvinashiatey/R110/releases).

## Usage

1. **Upload an image** – Click "Upload image" to select a PNG or JPEG file
2. **Select number of colors** – Choose how many RISO colors you want to use (1-4)
3. **Pick your colors** – Select from available RISO ink colors
4. **Apply effects** (optional) – Choose from Dither, HalfTone, or Threshold effects
5. **Process** – Click "Process" to generate the channel separations
6. **Export** – Choose PDF or PNG format and click "Export" to save your files

## Development

### Prerequisites

- [Node.js](https://nodejs.org/) (v18+)
- [Rust](https://www.rust-lang.org/tools/install)
- [pnpm](https://pnpm.io/)

### Setup

```bash
# Clone the repository
git clone https://github.com/alvinashiatey/R110.git
cd R110

# Install dependencies
pnpm install

# Run in development mode
pnpm tauri dev

# Build for production
pnpm tauri build
```

## License

[MIT](LICENSE)

## Author

[Alvin Ashiatey](https://github.com/alvinashiatey)
