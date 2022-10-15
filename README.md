# Railwind

Tailwind compiler rewritten in Rust

## Description

The main goal, is to decouple the original Tailwind project from Node and NPM with the side goal of possibly making it even faster and smaller (Tailwind already has a [standalone version](https://tailwindcss.com/blog/standalone-cli), but the binary size is a *whooping 35 mb*).

# Getting started

### Installation

To install with cargo, run `cargo install railwind_cli` to install the CLI.

### Using railwind_cli

Run `railwind_cli -i index.html` to generate a `railwind.css` file. You can optionally specify a different output file with the `-o` flag.

# Features

Currently, not all Tailwind features are supported. At the moment, the following features are working:

### Layout
- [x] Container
- [x] Aspect ratio

### Spacing
- [x] Padding
- [x] Margin

### Flexbox & Grid
- [x] Flex

### Background
- [x] Color

### Other
- [x] Pseudo-classes
- [x] Pseudo-elements
- [x] Media queries


## Authors

Contributors names and contact info

[@pintariching](https://github.com/pintariching)

## License

This project is licensed under the MIT License - see the LICENSE.md file for details

## Acknowledgments

Inspiration, code snippets, etc.
* [awesome-readme](https://github.com/matiassingers/awesome-readme)
