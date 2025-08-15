# fsio

`fsio` is a lightweight, low-level Rust library for advanced file I/O operations on Unix-like systems.  
It provides safe and idiomatic Rust abstractions over system calls, supporting file opening, reading, writing, metadata
handling, and fine-grained control over file flags and permissions.

## Features

- Open files with detailed control over flags (read/write/create/truncate, etc.)
- Read, write, insert, replace, truncate, and seek within files
- Retrieve detailed file metadata including file type, size, permissions, timestamps, UID/GID
- Unix permission flags and file type constants
- Clear modular structure with reusable flag sets and traits
- Minimal dependencies; uses `nix` crate for system calls

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
fsio = { git = "https://github.com/bkoshik/fsio.git" }
```

## Usage

Basic example:

```rust
use fsio::fileio::FileBuilder;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = FileBuilder::new()
        .read_write()
        .create()
        .truncate()
        .permissions(0o755)
        .open("test.txt");
    file.write("Hello, fsio!")?;

    let content = file.read_from_start()?;
    println!("File content: {}", content);

    Ok(())
}
```

## File Metadata

Access file metadata:

```rust
let metadata = file.metadata() ?;

println!("Size: {}", metadata.size());
println!("UID: {}", metadata.uid());
println!("GID: {}", metadata.gid());
```

## Contributing

Contributions are welcome! Feel free to open issues or pull requests.

## License

Apache License 2.0 © 2025 bkoshik
Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

https://www.apache.org/licenses/LICENSE-2.0
