## Demo

![Demo](./demo.gif)


## Installation 

```
cargo install todo-cli-app
```

## Usage

```
todo-cli-app --help
```
```
Yet another todo CLI app written in Rust

Usage: todo-cli-app.exe [OPTIONS] <COMMAND>

Commands:
  add   add tasks
  rm    remove tasks
  list  list tasks
  done  complete tasks
  help  Print this message or the help of the given subcommand(s)

Options:
  -f, --file <FILE>  The path to the file to read/write!
  -h, --help         Print help

```


## Inspired by 

- [Simple todo cli program written in rust](https://github.com/sioodmy/todo)
- [A simple command-line todo list written in Rust.](https://github.com/thekuwayama/todo)

## Difference

```
# todo-cli-app -f PATH [COMMAND] to have more than one todo list. 

todo-cli-app -f C:\Users\jackwill\Desktop\todo.txt add "Buy milk"

todo-cli-app -f C:\Users\jackwill\Desktop\todo.txt list
```
