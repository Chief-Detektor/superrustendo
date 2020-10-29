# Superrustendo

## What is it?
Right now it is nothing really exiting, yet.
Basically it is some kind of "emulator" which can execute SFC files.
By executing I actually mean it will start executing code starting from the Reset-Interrupt-Vector. Eventually it will break/stop executing due to not implemented Opcodes/Components.

That being said, there are two tools that already dropped out of this project:
- snesdis (A simple dissasembler)
- snesdbg (A step debugger)
Both tools are in a very eary stage and not really ready to be used right now.

## Why?
The SNES is my absolute favorite console.
I really wanted to understand how the system works and how to write software (games/demos) for it.
Also I wanted to improve my coding skills in rust. So I decided to start implementing this.

## How to use?

I assume you have rustup and cargo installed.

To "execute" a SNES Rom (SFC) one need to type:
```
cargo run --bin superrustendo <path-to-sfc-file>
```

To dissasemle a SNES ROM file this:
```
cargo run --bin snesdis <path-to-sfc-file>
```

And to debug a SNES ROM file:
```
cargo run --bin snesdbg <path-to-sfc-file>
```

## snesdbg usage
Once started with a sfc file the following commands are available:

Step one instruction:
`
s|step
`

Step n instructions:
`
s|step n
`

print CPU state:
`
p|print cpu
`
