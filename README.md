# Lazy Assembler (laasm)

Lazy Assembler (laasm) is a straightforward, command-line assembler that supports multiple instruction sets. It is designed for quick assembly tasks, offering an easy-to-use interface for converting assembly code snippets into their hexadecimal machine code equivalents. This tool is perfect for developers, educators, and hobbyists who require a fast way to assemble code without the overhead of setting up a full development environment.

## Usage

To use Lazy Assembler, you simply need to specify the instruction set architecture (ARCH) you're targeting and the assembly code (CODE) you want to assemble. Here's the basic syntax:

```bash
laasm <ARCH> <CODE>
```

This will output the hexadecimal machine code equivalent of the input assembly code.

## Arguments

**\<ARCH\>**: The instruction set of the assembly. Choose from the supported architectures listed below.

**\<CODE\>**: The assembly code to be assembled, as a string.

**Options**

`-h, --help`: Print help information.

## Example

To assemble a piece of x86-32 assembly code, you can use the following command:

```bash
$ laasm x86-32 "mov eax, 5; add eax, 10; ret;"
B8 05 00 00 00 83 C0 10 C3
```



## Features

- Supports a wide range of instruction sets, making it versatile for different architectures.
- Simple and concise command-line interface.
- Ideal for quick assembly tasks, testing, and educational purposes.

## Supported Architectures

Lazy Assembler supports the following instruction sets:

- ARM
- ARM-Thumb
- ARM64
- MIPS32
- MIPS64
- PPC32
- PPC64
- SPARC32
- SPARC64
- SystemZ
- x86-16
- x86-32
- x86-64
