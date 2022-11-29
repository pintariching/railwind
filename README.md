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

Currently, not all Tailwind features are supported. At the moment, the following features are working:

### Layout
- [x] Aspect Ratio
- [ ] Container
- [x] Columns
- [x] Break After
- [x] Break Before
- [x] Break Inside
- [x] Box Decoration Break
- [x] Box Sizing
- [x] Display
- [x] Floats
- [x] Clear
- [x] Isolation
- [x] Object Fit
- [x] Object Position
- [x] Overflow
- [x] Overscroll Behavior 
- [x] Position
- [x] Top / Right / Bottom / Left
- [x] Z-Index

### Spacing
- [x] Padding
- [x] Margin
- [x] Space Between

### Flexbox & Grid
- [x] Flex Basis
- [x] Flex Direction
- [x] Flex Wrap
- [x] Flex
- [x] Flex Grow
- [x] Flex Shrink
- [x] Order
- [x] Grid Template Columns
- [x] Grid Columns Start / End
- [x] Grid Template Rows
- [x] Grid Row Start / End
- [x] Grid Auto Flow
- [x] Grid Auto Columns
- [x] Grid Auto Rows
- [x] Gap
- [x] Justify Content
- [x] Justify Items
- [x] Justify Self
- [x] Align Content
- [x] Align Items
- [x] Align Self
- [x] Place Content
- [x] Place Items
- [x] Place Self

### Sizing
- [x] Width
- [x] Min-Width
- [x] Max-Width
- [x] Height
- [x] Min-Height
- [x] Max-Height

### Background
- [ ] Attachment
- [ ] Clip
- [ ] Color
- [ ] Origin
- [ ] Position
- [ ] Repeat
- [ ] Size
- [ ] Image
- [ ] Gradient Color Stops

### Border
- [ ] Border Radius
- [ ] Border Width
- [ ] Border Color
- [ ] Border Style
- [ ] Divide Width
- [ ] Divide Color
- [ ] Divide Style
- [ ] Outline Width
- [ ] Outline Color
- [ ] Outline Style
- [ ] Outline Offset
- [ ] Ring Width
- [ ] Ring Color
- [ ] Ring Offset Width
- [ ] Ring Offset Color

### Other
- [x] Pseudo-Classes
- [x] Pseudo-Elements
- [x] Media Queries

## Authors

Contributors names and contact info

[@pintariching](https://github.com/pintariching)

## License

This project is licensed under the MIT License - see the LICENSE.md file for details

## Acknowledgments

Inspiration, code snippets, etc.
* [awesome-readme](https://github.com/matiassingers/awesome-readme)
