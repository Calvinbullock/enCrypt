# Overview
This program will encrypt files for security and privacy. It will allow both command-line interaction 
as well as a more guided console experience.

This program aims to help me learn the Rust programing language and learn more about
using libraries. In this case, it is a cryptographic one.

install:
    navagate to your choosen directory.
    git clone https://github.com/Calvinbullock/enCrypt

Useing / running:
    cd into enCrypt directory

    cargo run

[Software Demo Video](https://www.youtube.com/watch?v=rsYSzaz9zSQ&list=PLoyljU3FIZC0Tj9_WPmjfL5vGZI_t2S0s&index=3)

# Development Environment

I am using the rustc package from the Ubuntu apt repositories.
this includes Rust version 1.71.1

My editer is vim/neovim and vs code.

library used sodiumoxide, made to be an easy to use rust cryptography library.

# Useful Websites

{Make a list of websites that you found helpful in this project}
* [Rust naming conventions](https://rust-lang.github.io/api-guidelines/naming.html#:~:text=Basic%20Rust%20naming%20conventions%20are%20described%20in%20RFC%20430.&text=snake_case!&text=In%20UpperCamelCase%20%2C%20acronyms%20and%20contractions,are%20lower%2Dcased:%20is_xid_start%20.)

* [sodiumoxide rust docs](https://docs.rs/sodiumoxide/latest/sodiumoxide/crypto/box_/index.html)

* [sodiumoxide github](https://github.com/sodiumoxide/sodiumoxide)

* [Rust Variables and Mutability](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html#variables-and-mutability)

# Future Work

- get file paths to work
- get std in to function corectly
- handel incorect input
- implement std in help arg
