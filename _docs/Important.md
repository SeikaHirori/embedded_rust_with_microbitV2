# Important Info

## Apple Silicon and gdb
- As of May 3, 2023, gdb is not supported (both natively and Rosetta 2).
- To workaround the issue, substitute `gdb` with: 
`arm-none-eabi-gdb` 
- To 

- Sources to use `arm-none-eabi-gdb`:
    - [GitHub Issue Comment](https://github.com/rust-embedded/book/issues/325#issuecomment-1229514071)
    - [Installation from "The Embedded Rust Book](https://docs.rust-embedded.org/book/intro/install/macos.html#install-tools-with-homebrew)
