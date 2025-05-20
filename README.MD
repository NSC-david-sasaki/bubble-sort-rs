# Bubble Sort Implementation in Rust

## Setup Instructions

1. Install Rust (if not already installed):
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
Then follow the on-screen instructions and restart your terminal.

2. Install mdBook:
```bash
cargo install mdbook
```

3. Clone the repository:
```bash
git clone https://github.com/yourusername/bubble-sort-rs
cd bubble-sort-rs
```

4. View the documentation:
The report is best enjoyed as html. Use your favorite browser to open `book/book/index.html` for the web version of the report.

If you prefer not to install any tools, you may view the markdown source at `book/book/src/` `SUMMARY.MD` is the table of contents to navigate to other pages.


   - For local development (with live reload):
   ```bash
   mdbook serve --open
   ```
   The documentation will open in your browser at `http://localhost:3000`

   - To generate static files:
   ```bash
   mdbook build
   ```
   The static site will be available in the `book/book` directory. 




## Project Overview

- Multiple sorting implementations with different optimization strategies
- Comprehensive test suite with edge cases
- Performance benchmarks and complexity analysis
- Cross-language comparison with Python

All code examples are tested and can be run directly from the repository.