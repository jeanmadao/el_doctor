# el_doctor

## What is el_doctor
A terminal-based written in Rust that allows you to monitor your computer
processes.

## Why use Rust and Ratatui
There were multiple choices offered to me in order to make this program.

1. Write it in C and use the curses library. While this was my initial preferred
option, I found it quite hard to use the curses library due to the lack (imo) of
examples available in the internet of its usage. I also found the curses manual
to be quite complex to understand. So probabably a skill issues from my part.

2. Write in Bash and use the [Bash Simple Curses](https://github.com/metal3d/bashsimplecurses)
library. This one has the advantage of being able to use linux commands directly
and display it via the library. However, this project doesn't seem to be actively
maintained.

3. Write it in Python and use the curses library. I could have actually use
this stack honestly.

And there probably were many more options offered to me. Instead, I chose to use
Rust and the [Ratatui](https://github.com/ratatui-org/ratatui) library.

First of all because I've been wanting to learn and practice using Rust, and 
ratatui is more modern way to write TUI than curses.
