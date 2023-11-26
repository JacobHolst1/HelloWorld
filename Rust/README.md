# Overview

This program allows a user to write, store, and read passwords using text files. It also has a password generator, which will provide the user with a password of a specified length and complexity.

I thought this project would be good for learning the basics of using primitive types in Rust. After my last project in C++, I wanted to keep things a bit more simple to make sure I could effectively build the program one step at a time.

[Software Demo Video](https://youtu.be/BX0CZIkc50E)

# Development Environment

This project was all written using VS Code. I mainly used the official website to learn, but also started referring to ChatGPT for help towards the end.

The code is all written in Rust, with one .txt file for storing passwords.

# Useful Websites

- [Rust Official Website](https://doc.rust-lang.org/book/ch01-02-hello-world.html)
- [Documentation on Dictionaries](https://docs.rs/dict/latest/dict/)
- [Packages and Crates](https://doc.rust-lang.org/book/ch07-01-packages-and-crates.html)
- [Files](https://doc.rust-lang.org/std/fs/struct.File.html)

# Future Work

- Implement file reader/writer to save and load passwords, not just print them on the console
- Convert the password generator to a vector so that it can change in length
- Learn more about Option<> so I can save generated passwords directly
