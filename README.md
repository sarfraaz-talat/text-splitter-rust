# Rust Text Splitter

This is a Rust library for splitting text into chunks based on a given chunk size and overlap size.

## Features

- Split text into chunks of a specific size
- Option to overlap chunks
- Option to provide custom delimiter for splitting

## Installation

Add the following to your `Cargo.toml` file:

```
[dependencies]
text_splitter = "0.1.0"
```

## Usage

Import the library and use the `CharacterTextSplitter` struct to split your text.

```
use text_splitter::CharacterTextSplitter;

fn main() {
    let text = "your text here...";
    let chunk_size = 200;
    let chunk_overlap = 50;
    let separator = String::from(". ");

    let splitter = CharacterTextSplitter::new(chunk_size, chunk_overlap, separator);
    let chunks = splitter.split_text(text);

    for chunk in chunks {
        println!("{}", chunk);
    }
}
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
