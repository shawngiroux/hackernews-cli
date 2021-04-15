# hackernews-tui

![Screenshot from 2021-04-02 09-52-15](https://user-images.githubusercontent.com/29616627/113421471-28e04d80-9399-11eb-8bbc-f8db9846756c.png)

This project is certainly a work in process to familiarize myself with Rust and creating terminal user interface.

## Running
I recommend compiling this project by using [Cargo](https://github.com/rust-lang/cargo/):
```
$ cargo run
```

## Controls

### Stories panel:
```
q: quit
k: traverse up
j: traverse down
enter: open story in default browser
c: open comments for story
```

### Comments panel:
```
q: go back
k: traverse up
K: traverse to previous parent comment
j: traverse down
J: traverse to next parent comment
g: go to top of comments
G: go to bottom of comments
y: yank comment text to clipboard
```
