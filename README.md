# Guess with Ferris

A fun and interactive number guessing game written in Rust, featuring Ferris the Crab!

## 🦀 Tech Stack

- **Language**: Rust (1.85.0)
- **Build Tool**: Cargo
- **Dependency Management**: Cargo
- **Toolchain Management**: Mise-en-place (mise)
- **Code Formatting**: rustfmt
- **Linting**: clippy

## 🚀 Getting Started

### Prerequisites

- Install [mise](https://mise.jdx.dev/) for toolchain management
- Git for version control

### Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/Brisinger/guess-with-ferris.git
   cd guess-with-ferris
   ```

2. Install project dependencies using mise:
   ```bash
   mise install
   ```

## 🛠️ Build & Run

### Development

1. Ensure you're using the correct Rust version:
   ```bash
   mise use
   ```

2. Build the project:
   ```bash
   cargo build
   ```

3. Run the game:
   ```bash
   cargo run
   ```

### Using Mise Tasks

The project includes several predefined tasks in `mise.toml`:

- `mise run format`: Format the code using rustfmt
- `mise run lint`: Run clippy for linting
- `mise run build`: Build the release binary
- `mise run ci`: Run all CI checks (format, lint, build)

## 📦 Release with Mise

### Building a Release

1. Ensure all dependencies are installed:
   ```bash
   mise install
   ```

2. Build the optimized release binary:
   ```bash
   mise run build
   ```

3. The binary will be available at `target/release/guessing_game`

### Version Management

Mise ensures consistent development environments by:
- Managing Rust toolchain versions
- Enforcing consistent tooling versions across the team
- Providing reproducible builds

## 📄 Project Structure

```
guess-with-ferris/
├── src/                # Source code
├── scripts/            # Utility scripts
├── .github/workflows/  # CI/CD configuration
├── Cargo.toml          # Project manifest
└── mise.toml           # Toolchain configuration
```

## 🔧 Cargo.toml

The `Cargo.toml` file is the manifest for your Rust project:

```toml
[package]
name = "guessing_game"
version = "0.1.0"
edition = "2024"

dependencies = [
    "rand = \"0.8.5\""
]
```

### Key Sections:

1. **`[package]`**:
   - `name`: The name of your package
   - `version`: Current version (follows semantic versioning)
   - `edition`: Rust edition (2024 in this case)

2. **`[dependencies]`**:
   - Lists all external crates your project depends on
   - Version requirements can be specified using semantic versioning

## 🔄 Toolchain Management with Mise

Mise is used to manage the Rust toolchain and other development tools. The configuration is in `mise.toml`:

```toml
[tools]
"rust" = { version = "1.85.0", profile = "default" }
```

### Key Features:

- **Automatic Tool Installation**: Mise will automatically install the specified Rust version
- **Per-project Toolchains**: Different projects can use different Rust versions
- **Reproducible Builds**: Ensures everyone uses the same tool versions

### Common Mise Commands:

```bash
# Install all tools specified in mise.toml
mise install

# Use the tools specified in the current directory
mise use

# Show current tool versions
mise which rustc
mise which cargo
```

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Commit your changes
4. Push to the branch
5. Create a new Pull Request

## 📜 License

This project is licensed under the [Apache License 2.0](LICENSE).

```
   Copyright 2025 Shubhojit Dasgupta

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
```

---

Made with ❤️ and 🦀
