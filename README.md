![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/xerrion/shr_parser_py/CI.yml) ![GitHub License](https://img.shields.io/github/license/xerrion/shr_parser_py) ![Crates.io Version](https://img.shields.io/crates/v/shr_parser_py?link=https%3A%2F%2Fcrates.io%2Fcrates%2Fshr_parser_py)

# SHR Parser Python Bindings

This project provides Python bindings for the Rust-based `shr_parser` library, which is designed to parse and handle SHR files. The SHR file format includes a file header and multiple sweeps, each with its own header and data. This library uses memory mapping for efficient file reading and Rayon for parallel processing of sweeps.

## Features

- **Parse SHR Files:** Read and interpret SHR files, including headers and sweeps.
- **Validate Files:** Ensure the integrity of SHR files by validating signatures and versions.
- **Sweep Metrics Calculation:** Compute key metrics such as peak, mean, and low values from sweep data.
- **CSV Export:** Export parsed SHR data to CSV format for easy analysis and reporting.

## Installation

To install the Python bindings, you need to build the Rust library and install it as a Python module using `maturin`. Make sure you have Rust and Python installed on your system.

1. Install `maturin`:
    ```sh
    pip install maturin
    ```

2. Build and install the module:
    ```sh
    maturin develop
    ```

## Usage

Here's an example of how to use the SHR file parser from Python:

```python
import shr_parser

# Define the file path and parsing type
file_path = "path/to/your/shrfile.shr"
parsing_type = 0  # SHRParsingType::Peak

# Create a SHRParser instance and use its methods
parser = shr_parser.SHRParser(file_path, parsing_type)
print(parser.get_file_path())
print(parser.get_file_header())
for sweep in parser.get_sweeps():
    print(sweep)
parser.to_csv("output.csv")
```

## Module Structure

### `SHRParser` Class

A class representing a parser for SHR files.

#### Methods

- `__init__(self, file_path: str, parsing_type: int) -> SHRParser`
- `to_str(self) -> str`
- `to_csv(self, path: str) -> None`
- `get_file_path(self) -> str`
- `get_file_header(self) -> str`
- `get_sweeps(self) -> List[Tuple[int, int, float, float]]`

## Example

Here is an example usage of the SHRParser class:

```python
from shr_parser import SHRParser, SHRParsingType

# Define the file path and parsing type
file_path = "path/to/your/shrfile.shr"
parsing_type = SHRParsingType.PEAK

# Create a parser instance
parser = SHRParser(file_path, parsing_type)

# Get file path
print(parser.get_file_path())

# Get file header
print(parser.get_file_header())

# Get sweeps
for sweep in parser.get_sweeps():
    print(sweep)

# Export to CSV
parser.to_csv("output.csv")
```

## Documentation

For detailed documentation on the `shr_parser` library, refer to the [Rust documentation](https://docs.rs/shr_parser/1.0.2/shr_parser/). To generate the documentation locally, run:

```sh
cargo doc --open
```

## Contributing

Contributions are welcome! If you encounter any issues or have suggestions for improvements, please open an issue or submit a pull request.

## License

This project is licensed under the GPLv3 License. See the `LICENSE` file for details.
