![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/xerrion/shr_parser_py/CI.yml) ![GitHub License](https://img.shields.io/github/license/xerrion/shr_parser_py) ![Crates.io Version](https://img.shields.io/crates/v/shr_parser_py?link=https%3A%2F%2Fcrates.io%2Fcrates%2Fshr_parser_py)

# NOTE: No longer actively maintained, if anyone want to take over please contact me. Will probably be archived at some point.

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

Create a parser with a `SHRParsingType` enum member:

```python
import shr_parser

file_path = "path/to/your/shrfile.shr"
parser = shr_parser.SHRParser(file_path, shr_parser.SHRParsingType.PEAK)
print(parser.get_file_path())
print(parser.get_file_header())
for sweep in parser.get_sweeps():
    print(sweep)
    for frequency_hz, amplitude in sweep.get_data():
        print(frequency_hz, amplitude)
parser.to_csv("output.csv")
```

You can also create a parser with `create_parser(file_path: str, parsing_type: SHRParsingType) -> SHRParser`.

## Module Structure

### `SHRParsingType`

Enum members: `PEAK = 0`, `MEAN = 1`, and `LOW = 2`. Pass a member to `SHRParser` or `create_parser`.

### `SHRParser`

- `__init__(self, file_path: str, parsing_type: SHRParsingType) -> SHRParser`
- `__repr__(self) -> str`
- `to_csv(self, path: str) -> None`
- `get_file_path(self) -> str`
- `get_file_header(self) -> str` — returns Rust Debug-formatted text, not a structured header.
- `get_sweeps(self) -> list[SHRSweep]`

Parse and CSV-write failures raise `OSError` (`PyIOError`), with messages prefixed by `Failed to parse SHR file: ` and `Failed to write to CSV: `, respectively.

### `SHRSweep`

`SHRSweep` has a `__repr__` method and these read-only fields: `sweep_number`, `timestamp`, `frequency`, `amplitude`, `latitude`, and `longitude`.

- `get_data(self) -> list[tuple[float, float]]` — returns all bins in file order as
  `(frequency_hz, amplitude)` tuples. Frequencies are in Hz, unlike the summary
  `frequency` field, which is in MHz. Each amplitude is the original file's
  32-bit float value exposed as a Python `float`, without rescaling.

`get_sweeps()` creates summary wrappers without copying their bin data. The tuple
list is created only when `get_data()` is called. A sweep keeps its parser alive,
so its data remains accessible after the original parser variable is deleted.

## Building from source

The binding depends on the Rust parser at `../shr_parser`. Check out both
repositories as siblings:

```text
projects/
├── shr_parser/
└── shr_parser_py/
```

From `projects/`, build with `maturin build --manifest-path shr_parser_py/Cargo.toml`.
The CI workflow checks out both repositories in this layout.

## Documentation

For detailed documentation on the `shr_parser` library, refer to the [Rust documentation](https://docs.rs/shr_parser/latest/shr_parser/). To generate the documentation locally, run:

```sh
cargo doc --open
```

## Contributing

Contributions are welcome! If you encounter any issues or have suggestions for improvements, please open an issue or submit a pull request.

## License

This project is licensed under the GPLv3 License. See the `LICENSE` file for details.
