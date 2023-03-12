# a

CLI tool to generate code from ChatGPT that pretty-prints the output by language

![tty](https://user-images.githubusercontent.com/6937171/221947920-9cf8ed65-9ead-490b-a3c3-e606012ee8ce.gif)

## Installation

```
cargo install a-gpt

# or to enable clipboard functionality:
# cargo install a-gpt --features clipboard

export OPENAI_API_KEY=sk-WEz... # from https://platform.openai.com/account/api-keys
```

For local development:
```
make release
sudo make install
```

If the `clipboard` feature is enabled and you are running Ubuntu/Debian, you may need to install the following packages:
```
sudo apt install xorg-dev libxcb-composite0-dev xclip
```

## Usage
You will need an OpenAI API key, and to set the environment variable `OPENAI_API_KEY`.

Invoke the `a` command followed by a prompt. If the first word in the prompt is a programming language or file-format the pretty-printer recognizes, it will syntax highlight the output.

```bash
a python script that fetches a url
a rust program that showcases its various features
a yaml manifest describing a kubernetes deployment
```

Invoking the command with no arguments will read from stdin, accepting input interactively or from a pipe. You can choose to include or not include the word "a" at the beginning your input string.

```bash
echo "python script that fetches a url" | a
echo "a python script that fetches a url" | a
```

If installed with the `clipboard` feature enabled, the output will be copied to the clipboard.
