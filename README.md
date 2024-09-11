# Mastermind - A Second Brain for Spymasters

Mastermind is a CLI tool that generates clue words for spymasters in the game of **Codenames**. Powered by LLMs of your
choice. Written in Rust because why not?

![GitHub License](https://img.shields.io/github/license/theoforger/mastermind)

<img src="demo.gif" width="600"/>

## Usage

You need to prepare two text files.

1. **Words to Link Together**: This file contains the words from your own team.
2. **Words to Avoid**: This file contains:

- Your opponent's words
- Neutral words
- The assassin word

Each file should contain one word per line. See the [examples](examples) directory for sample files.

To use the tool, run:

```bash
mastermind [TO_LINK] [TO_AVOID]
```

Feel free to run the program multiple times to get the best result!

### Options

- `-g` or `--get-models` : Get all available language models.
- `-m` or `--set-model` : Choose a language model.
- `-V` or `--version` : Display the version number.
- `-h` or `--help` : Display help information.

## Building

### Prerequisites

- `rust`
- `cargo`

Using [`rustup`](https://www.rust-lang.org/tools/install) will install them both. Alternatively, you can install them
from your package manager. Keep in mind that some distros package them separately.

### Clone the Repository

```bash
git clone https://github.com/theoforger/mastermind.git
cd mastermind
```

### Configure Environment Variables

Edit the [example.env](example.env) file to include or modify:

- Your API key
- The base URL of an OpenAI Compatible API
- A default language model

Once you're done, rename it to `.env`:

```bash
mv example.env .env
```

### Build the Project

Simply run:

```bash
cargo build --release
```
