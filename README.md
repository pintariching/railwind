# Railwind

Tailwind compiler rewritten in Rust

## Description

The main goal, is to decouple the original Tailwind project from Node and NPM and add warning messages with some recovery in specific situations with the side goal of possibly making it even faster and smaller (Tailwind already has a [standalone version](https://tailwindcss.com/blog/standalone-cli), but the binary size is a *whooping 35 mb*).

# Getting started

### Installation

To install with cargo, run `cargo install railwind_cli` to install the CLI.

### Using railwind_cli

Run `railwind_cli -i index.html` to generate a `railwind.css` file. You can optionally specify a different output file with the `-o` flag.

# Features

### Warning messages

The CLI can write helpfull warning messages if you didn't pass the right value to a class or if you passed not enough or too many arguments. 

All Tailwind classes are now supported.


## Authors

Contributors names and contact info

[@pintariching](https://github.com/pintariching)

## License

This project is licensed under the MIT License - see the LICENSE.md file for details

## Acknowledgments

Inspiration, code snippets, etc.
* [awesome-readme](https://github.com/matiassingers/awesome-readme)
