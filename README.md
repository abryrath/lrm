# lrm
## Lorem ipsum generation binary written in Rust
---
## Why?
* Browser plugins don't work the way I want (not enough control)
* I like re-inventing the wheel
* I wanted to brush up on my Rust skills (non-existant)
---
## Installation

### macOS
```
brew tap abryrath/taps
brew install lrm
```

---
## Usage

```
lrm 0.0
Abry Rath <abryrath@gmail.com>
Generate lorem ipsum on the command line

USAGE:
    lrm [FLAGS] [OPTIONS]

FLAGS:
    -b, --breaks     Include line breaks
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -W, --break-width <break_width>    Insert line breaks after this many characters, defaulting to 80.
    -c, --chars <chars>                Specify the number of characters to generate, including whitespace.
    -w, --words <words>                Specify the number of words to generate, defaulting to 50. Setting this overrides
                                       `chars`.

```

---
## Shout Outs
The actual lorem ipsum functionality comes from [lipsum](https://docs.rs/lipsum/0.6.0/lipsum/index.html), a simple and easy-to-use crate. I am not affiliated with the author.

## License
MIT
