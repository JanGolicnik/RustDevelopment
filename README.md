# Rust Compiler

An experimental compiler written in Rust.

## Table of Contents
- [Introduction](#introduction)
- [Features](#features)
- [TODO](#todo)
- [Examples](#examples)

## Introduction

This is a simple compiler written in Rust. It is designed for educational purposes and supports a limited set of features.

## Features

- **Variables**: Declare variables using `let`.
- **Operators**: Support for `+`, `-`, `/`, `*`, `<`, `>`, `=`, and parentheses for operator precedence.
- **Exit Code**: Return values using the `return` statement.
- **Functions**: Define functions using `fn`.
- **Console Output**: Print characters to the console using the `print` function.
- **If Statements**: Use `if` statements for conditional code.
- **Loops**: Implement `while` loops.
- **Break Statement**: Break out of loops using the `break` statement.
- **Scopes**: Enclose code in `{}` to create scopes.
- **String Literals**: Assign string literals using double quotes `"`.

## TODO

- Accepting user input
- ??? (Please provide more details on future plans)

## Examples

- Showing off features

```cpp
let x = 4 * (5 + 1);
while x < 10 {
    if x > 5 {
        break;
    }
    x = x + 1;
}
return x;
```

```cpp
fn Print(ptr, len){
    let i = 1;
    while i < len {
        print i + ptr;
        i = i + 1;
    }
    return i;
}

let hello_string = "Hello World!";

return Print(hello_string, 12);
```