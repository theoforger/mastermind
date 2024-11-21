# 🔮 Mastermind - A Second Brain for Spymasters

Mastermind is a CLI tool designed to generate clue words for spymasters in the game of **Codenames**, leveraging large language models (LLMs) of your choice!

Written in Rust 🦀, because why not?

![GitHub License](https://img.shields.io/github/license/theoforger/mastermind)
![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/theoforger/mastermind/.github%2Fworkflows%2Frust.yml)

<img src="images/demo.gif" alt="A gif demo of the basic functions of this program."/>

## 📖 Prepare
To get started, prepare two text files:

1. **Words to Link Together** - Contains the words from your own team.
2. **Words to Avoid** - Contains:
    - Your opponent's words
    - Neutral words
    - The assassin word

One word per line. Refer to the [`examples`](examples) directory for sample files.

## 🛠️ Configure
Here are what you need to configure before running mastermind:
- API key
- The base URL of an OpenAI-compatible API
- A default language model

There are two ways to configure this program:
### Config File
During the first run, a config file will be created at your system's preferred location. The specific location will be given in the output. Generally, it is located at:
- For Linux: `$HOME/.config/mastermind/config.toml`
- For macOS: `$HOME/Library/Application Support/mastermind/config.toml`
- For Windows: `C:\Users\[your username]\AppData\Roaming\mastermind\config.toml`

### Environment Variables
Make a copy of [`example.env`](example.env) and name it `.env`

```bash
cp example.env .env
```

Then edit `.env` and provide you preferred configuration.

Alternatively, you can simply pass these environment variables during run time.


## 🏃 Run

```bash
mastermind [TO_LINK] [TO_AVOID]
```

Feel free to run the program multiple times to get the best result!

### Options

- `-g`, `--get-models` : Print all available language models
- `-m`, `--set-models` : Select language model(s)
- `-o`, `--output` : Specify an output file
- `-t`, `--token-usage` : Print token usage
- `-h`, `--help` : Print help
- `-V`, `--version` : Print version
