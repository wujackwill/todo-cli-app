## Demo

*Use default todo.txt path*
![Demo](./demo.gif)

*Use custom todo.txt path*
```
# todo-cli-app -f PATH [COMMAND] to add tasks, etc... It will automatically create the txt file
todo-cli-app -f /mnt/c/Users/jackwill/Desktop/todo.txt add "hello world"
```

## Note

Fix: please run `todo-cli-app init` at your home directory to create the todo.txt file, then you can use the default path and run `todo-cli-app list` to list the tasks ,etc...


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

Usage: todo-cli-app [OPTIONS] <COMMAND>

Commands:
  init   Initialize a new todo file
  add    add tasks
  rm     remove tasks
  list   list tasks
  done   complete tasks
  clear  clear all tasks
  sort   sort tasks
  edit   edit a task
  sync   add tags for the tasks sync from other device
  help   Print this message or the help of the given subcommand(s)

Options:
  -f, --file <FILE>  The path to the file to read/write!
  -h, --help         Print help

```


## Inspired by 

- [Simple todo cli program written in rust](https://github.com/sioodmy/todo)
- [A simple command-line todo list written in Rust.](https://github.com/thekuwayama/todo)

## Difference

```
# todo-cli-app -f PATH [COMMAND] to have more lists or sync the txt file between different devices

todo-cli-app -f C:\Users\jackwill\Desktop\todo.txt add "Buy milk"

todo-cli-app -f C:\Users\jackwill\Desktop\todo.txt list
```
