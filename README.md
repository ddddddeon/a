# a

CLI tool to generate code from GPT3 that pretty-prints the output by language, and copies to your clipboard

![tty](https://user-images.githubusercontent.com/6937171/221947920-9cf8ed65-9ead-490b-a3c3-e606012ee8ce.gif)

## Installation 
```
cargo install a-gpt
```

For local development:
```
make release
sudo make install
```

If the `clipboard` feature is enabled and you are running Ubuntu/Debian, you may need to install the following packages:
```
sudo apt install xorg-dev libxcb-composite0-dev
```

## Usage
You will need an OpenAI API key, and to set the environment variable `OPENAI_API_KEY`.

Invoke the `a` command followed by a prompt. If the first word in the prompt is a programming language or file-format the pretty-printer recognizes, it will syntax highlight the output.

```
a python script that fetches a url
a rust program that showcases its various features
a yaml manifest describing a kubernetes deployment
```

If installed with the `clipboard` feature (enabled by default), the output will be copied to the clipboard.
