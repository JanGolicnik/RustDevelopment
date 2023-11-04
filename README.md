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
- **Arrays**: Declaring and indexing into arrays with `[]`.
- **References**: Using references to get addresses of variables with `&`.
- **Dereferences**: We have pointers at home with `*`.
- **Reading**: Read from console input with `read`.

## TODO

- Better assembly generation
- General usability

## Prerequisites

- `rustup + cargo`
- `nasm`

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

```cpp
fn neki(x){
    return x[1];
}
let b[2] = 0;
b[1] = 10;
return neki(b);
```

```cpp
let x[2] = 65;
x[1] = 66;
x[2] = 67;
print &x + 8;
return &x;
```

```cpp
let x = 65;
let y = &x;
return *y;
```

```cpp
let opening_string = "Enter two numbers please      ";
let first_input_prompt = "First number: ";
let second_input_prompt = "Second number: ";

let new_line = 10;

let read_buf = 0;

print opening_string, 39;
print &new_line, 1;

while true{
    print first_input_prompt, 14;
    read &read_buf, 1;
    let x = read_buf;
    read &read_buf, 1;

    print second_input_prompt, 15;
    read &read_buf, 1;
    let y = read_buf;
    read &read_buf, 1;

    x = x - 48;
    y = y - 48;  

    if x < 4 {
        if y < 4 {
            let res = x + y + 48; 
            print &res, 1;
            print &new_line, 1;
            return x + y + 48;
        }
    }
}
```