# Contributing to Mastermind 🔮

Thank you for your interest in contributing to the Mastermind project! This documentation file will walk you through the setup process and proper linter configurations.

## ⚖️ License Disclaimer
The Mastermind project is licensed under The GNU General Public License v3.0. By contributing to this project, you are agreeing to provide your content under this license as well. In addition, you are agreeing to explicitly grant your patent rights to all users of this project.

For more information, refer to the [license](https://github.com/theoforger/mastermind/blob/main/LICENSE)

---

## 🛠️ Building

### Prerequisites

- `rust`
- `cargo`

Installing via [`rustup`](https://www.rust-lang.org/tools/install) is recommended.

You may also get them from your package manager (note: some distributions may package them separately).

### Clone the Repository

```bash
git clone https://github.com/theoforger/mastermind.git
cd mastermind
```

### Build the Project

Run the following command:

```bash
cargo build
```

### Configure Environment Variables

See: https://github.com/theoforger/mastermind?tab=readme-ov-file#%EF%B8%8F-configure

Alternatively, you can make use of the `.env` file. Simply make a copy of example.env and name it .env

```bash
cp example.env .env
```

Then edit .env and provide you preferred configuration.

## ✒️ Linting / Code Formatting
Before you commit, make sure the following linting/formatting tools using the options below. Commit only when your code is error/warning free.

### rustfmt
Run the following command:
```bash
cargo fmt
```

### Clippy
If you don't have `clippy` installed already. Add it through `rustup`:
```bash
rustup component add clippy
```

Then run the following command:

```bash
cargo clippy --all-features --all-targets -- -W clippy::all
```

### IDE Integration
We recommend [RustRover](https://www.jetbrains.com/rust/) for contributions to this project. It comes with first-party support for both `rustfmt` and `clippy`. To enable them:

- `rustfmt`: Open "Settings -> Rust -> Rustfmt". Enable "Use Rustfmt instead of the built-in formatter"
- `clippy`: Open "Settings -> Rust -> External Linters". Set "External tool" to `Clippy`. Set "Additional arguments" to `--all-features --all-targets -- -W clippy::all`