# 🔮 Mastermind - A Second Brain for Spymasters

Mastermind is a CLI tool designed to generate clue words for spymasters in the game of **Codenames**, leveraging large language models (LLMs) of your choice!

Written in Rust 🦀, because why not?

![GitHub License](https://img.shields.io/github/license/theoforger/mastermind)
![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/theoforger/mastermind/.github%2Fworkflows%2Frust.yml)

<img src="images/demo.gif" alt="A gif demo of the basic functions of this program."/>

## 💻 Usage

To get started, prepare two text files:

1. **Words to Link Together** - Contains the words from your own team.
2. **Words to Avoid** - Contains:
    - Your opponent's words
    - Neutral words
    - The assassin word

One word per line. Refer to the [`examples`](examples) directory for sample files.

Run the tool with:

```bash
mastermind [TO_LINK] [TO_AVOID]
```

Feel free to run the program multiple times to get the best result!

### Options

- `-g`, `--get-models` : Print all available language models
- `-m`, `--set-model` : Select a language model
- `-o`, `--output` : Specify an output file
- `-h`, `--help` : Print help
- `-V`, `--version` : Print version

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

### Configure Environment Variables

Make a copy of [`example.env`](example.env) and name it `.env`

```bash
cp example.env .env
```

Edit `.env` to add or modify:
- API key
- The base URL of an OpenAI-compatible API
- A default language model

### Build the Project

Run the following command:

```bash
cargo build --release
```
