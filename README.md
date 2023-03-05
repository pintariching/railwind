# Railwind

Tailwind compiler rewritten in Rust

## Description

The main goal, is to decouple the original Tailwind project from Node and NPM and add warning messages with some recovery in specific situations with the side goal of possibly making it even faster and smaller (Tailwind already has a [standalone version](https://tailwindcss.com/blog/standalone-cli), but the binary size is a *whooping 35 mb*).

# Getting started

### Installation

To install with cargo, run `cargo install railwind_cli` to install the CLI.

### Using railwind

To first start, generate a default `railwind.config.ron` file using `railwind -g` or `railwind --generate`. At the moment, the config supports only two values:

#### **content**
Similar to `tailwind`s option, configure a path to all your HTML templates, Rust or JS files.
#### **extend_collection_options**
The compiler reads the file extension and selects an apropriate `regex` or way to parse that file. For example, files ending with `.html` will be parsed with a `regex`: `(?:class|className)=(?:["]\W+\s*(?:\w+)\()?["]([^"]+)["]` to extract the class names. Similarly, you can specify your own `regex` to parse custom files:

```
extend_collection_options: Some({
    "rs": Regex(r#"(?:class)=(?:["]\W+\s*(?:\w+)\()?["]([^"]+)["]"#)
})
```
or give hints to the compiler, for example to parse a `rs` file as a `html` file:

```
extend_collection_options: Some({
    "rs": Html
})
```

To check out what other options are available, check out the documentation or the `railwind::CollectionOptions` enum which can be expaned.

After setting up the config file, you can run `railwind` to read the `railwind.config.ron` and generate a `railwind.css` file in the same directory. You can optionally specify a different config file with the `-c` flag and a different output file using the `-o` flag. 

## Authors

Contributors names and contact info

[@pintariching](https://github.com/pintariching)

## License

This project is licensed under the MIT License - see the LICENSE.md file for details

## Acknowledgments

Inspiration, code snippets, etc.
* [awesome-readme](https://github.com/matiassingers/awesome-readme)
