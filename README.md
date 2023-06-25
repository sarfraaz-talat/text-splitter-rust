# TextSplitter - Rust

TextSplitter is a Rust library for splitting a text into chunks of specified size with an option to overlap.

This library replicates the functionality of the `CharacterTextSplitter` found in the Python Langchain library, giving Rust users access to the same text splitting capabilities. It's designed to be straightforward to use and requires zero dependencies, making it a lightweight addition to any project.

## Features

- Split text into chunks of a specific size
- Option to overlap chunks
- Option to provide custom delimiter for splitting

## Installation

Add the following to your `Cargo.toml` file:

```toml
[dependencies]
character_text_splitter = "0.1.2"
```

## Usage

Import the library and use the `CharacterTextSplitter` struct to split your text.

```rust
use character_text_splitter::CharacterTextSplitter;

let text = "your text here...";

let splitter = CharacterTextSplitter::new();
let chunks = splitter.split_text(text);

for chunk in chunks {
    println!("{}", chunk);
}
```

You can also specify the chunk_size, chunk_overlap size or the separator you want to use for the library, like this

```rust
    let splitter = CharacterTextSplitter::new()
        .with_chunk_size(300)
        .with_chunk_overlap(50)
        .with_separator(". ");
```

Default value for `chunk_size` is `200`, `chunk_overlap` is `40` and default `separator` is `\n\n`

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
