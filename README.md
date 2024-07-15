# ⚙ brainfuck 🧠

## Basics

Brainfuck is an esoteric programming language created by [Urban Müller](https://esolangs.org/wiki/Urban_Müller). It was originally developed as a toy language to create interpreters and compilers for and it is an example of a [Turing tarpit](https://esolangs.org/wiki/Turing_tarpit)(a language designed to be Turing complete while using the smallest possible number of commands).

Brainfuck operates on an array of 30,000 cells, each initially set to zero. The language uses a [Turing tape](https://en.wikipedia.org/wiki/Turing_machine) model, where each cell can hold a single byte of data, and a data pointer starts at the beginning of this array. The language consists of only eight commands, minimalist yet Turing complete, meaning it can simulate any Turing machine and therefore perform any computation given enough time and memory.

### Commands

| command | meaning                          | c equivalent                 |
| ------- | -------------------------------- | ---------------------------- |
| `+`     | increase value pointed by MP     | `*ptr++`                     |
| `-`     | decrease value pointed by MP     | `*ptr--`                     |
| `>`     | increase memory pointer          | `ptr++`                      |
| `<`     | decrease memory pointer          | `ptr--`                      |
| `[`     | jump to matching `]` if zero     | `while(*ptr) {`              |
| `]`     | jump to matching `[` if not zero | `}`                          |
| `.`     | print current cell to console    | `printf("%c", (char) *ptr);` |
| `,`     | get input store in current cell  | `scanf("%c", ptr);`          |

anything other than these commands are considered comments and ignored.

## Usage

### Interpreter

to build and run: (don't forget to resize your terminal window)

```bash
$ cargo run --release -- <path-to-file>
```

or

```bash
$ cargo build --release
$ ./target/release/fucker examples/hanoi.b
```

to install the binary

```bash
$ cargo install --path .
$ fucker examples/mandel.b
```

You can enable debug mode with the -d or --debug flag to see how long parsing, optimizing, and executing take.

```bash
$ fucker examples/gold.b -d
```

if you want to disable some of the optimizations, you can use the following flags

```bash
$ fucker -f-no-optmize-scan  ...
$ fucker -f-no-optmize-clear ...
$ fucker -f-no-optmize-loops ...
```

you have always the option to check yourself

```bash
$ fucker --help
```

### Compiler

coming soon ✨

## Abstract Syntax Tree

There are 12 node types, including `Comment` and `NoOp`, which are removed before execution

```rust
pub enum ASTNode {
    Incr(u8),
    Decr(u8),
    Next(usize),
    Prev(usize),
    Loop(Vec<ASTNode>),
    Input,
    Output,
    Set(u8),
    ScanLeft,
    ScanRight,
    Comment(char),
    NoOp,
}
```

## Optimizations

### stacking nodes

consecutive `Incr`, `Decr`, `Next`, `Prev` nodes are optimized into single operand during parsing

```bf
[->>>+++<<]
```

can be thought of as

```rust
ASTNode::Loop(
    [
        ASTNode::Decrement(1),
        ASTNode::Next(3),
        ASTNode::Increment(3),
        ASTNode::Prev(2),
    ]
)
```

this reduces number of operations to be executed

### clear optimization

```bf
[-]
```

the brainfuck snippet above is equivalent of the c code below, we can optimize this into a single operation, `ASTNode::Set(0)`

```c
while(*ptr) --*ptr;
```

### scan optimization

```bf
first  [<]
second [>]
```

these can be transpiled to C like this

```c
// first
while(*ptr) --ptr;

// second
while(*ptr) ++ptr;
```

we can optimize these into `ASTNode::ScanLeft` and `ASTNode::ScanRight`, therefore reducing number of instructions

### loop optimization

unused loops should be removed, such as

```bf
first [[[-]]]
second [[ ]]
```

can be thought of as

```bf
first  [-]
second
```

## Benchmarks

tested on Apple M1 silicon

| script     | parsing      | optimizing | interpretion |
| ---------- | ------------ | ---------- | ------------ |
| mandelbrot | 10.614334ms  | 58.375µs   | 5.570075416s |
| hanoi      | 141.925583ms | 168.459µs  | 596.212459ms |

## TODO

- [ ] ARM compiler
- [ ] JIT compiler
- [ ] x86 compiler
- [ ] more AST optimizations
