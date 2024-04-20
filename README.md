# Scanner
[![Release](https://github.com/Velka-DEV/scanner/workflows/Release/badge.svg)](https://github.com/Velka-DEV/scanner/actions?query=workflow:"Release")
[![GitHub release](https://img.shields.io/github/release/Velka-DEV/scanner?include_prereleases=&sort=semver&color=blue)](https://github.com/Velka-DEV/scanner/releases/)
[![License](https://img.shields.io/badge/License-GPLv3-blue)](#license)
[![issues - scanner](https://img.shields.io/github/issues/Velka-DEV/scanner)](https://github.com/Velka-DEV/scanner/issues)

Scanner is a command-line tool written in Rust that scans a directory and its subdirectories to generate a project tree and display the contents of files with specified extensions. It provides a convenient way to visualize the structure of your project and view the code snippets of selected files.

## Features

- Generates a project tree showing the directory structure
- Displays the contents of files with specified extensions
- Supports filtering files by extension
- Allows excluding specific files and folders from the output
- Provides syntax highlighting for various programming languages in the output

## Installation

You can download the latest release of Scanner from the [Releases](https://github.com/Velka-DEV/scanner/releases) page on GitHub. The releases are available for Windows, macOS, and Linux.

## Build

To build Scanner, you need to have Rust installed on your system. If you don't have Rust installed, you can download and install it from the official Rust website: [https://www.rust-lang.org/](https://www.rust-lang.org/)

Once Rust is installed, you can clone this repository and build the project using Cargo, the Rust package manager:

```bash
git clone https://github.com/yourusername/scanner.git
cd scanner
cargo build --release
```

After the build process is complete, you can find the `scanner` executable in the `target/release` directory.

## Usage

To run Scanner, navigate to the directory you want to scan and execute the `scanner` executable with the desired options:

```bash
./scanner [OPTIONS]
```

### Options

- `-p, --path <PATH>`: Specify the path of the directory to scan. If not provided, the current directory will be scanned.
- `-e, --extensions <EXTENSIONS>`: Specify the file extensions to include in the output (comma-separated). If not provided, all files will be included.
- `-x, --exclude <PATTERNS>`: Specify the file/folder patterns to exclude from the output (comma-separated). If not provided, no files/folders will be excluded.
- `-h, --help`: Display the help message.
- `-V, --version`: Display the version of Scanner.

### Examples

Scan the current directory and include only files with extensions ".go", ".rs", and ".py":

```bash
./scanner -e go,rs,py
```

Scan the current directory, include all files, and exclude files/folders matching the patterns "target" and "node_modules":

```bash
./scanner -x target,node_modules
```

Scan the current directory, include only files with extensions ".java" and ".kt", and exclude files/folders matching the pattern "build":

```bash
./scanner -e java,kt -x build
```

## Output

Scanner generates the output in two sections:

1. **Project Tree**: Displays the directory structure of the scanned project. Each directory is represented by its name followed by a forward slash (/), and each file is represented by its name.

2. **Files**: Displays the contents of files with the specified extensions. Each file is presented with its name, followed by the code snippet enclosed in a code block with syntax highlighting based on the file extension.

The syntax highlighting is provided for various programming languages based on the file extension. Scanner uses a predefined mapping of file extensions to their corresponding language identifiers for code blocks.

## Contributing

Contributions to Scanner are welcome! If you find any issues or have suggestions for improvements, please open an issue or submit a pull request on the GitHub repository.

## License

Scanner is open-source software licensed under the [GNU GPLv3 License](LICENSE).

