# Intro

Minrusty is a minor side project of mine, to try to rewrite some parts
of Mindustry using Rust, while experimenting with the possibility of
parallel computing. An entity component system (ECS) library will be
used, which leverages Rust's type system, and generally makes very
efficient use of ballencing ergonomics and system resources. 

Mindustry is developed in Java and that means a lot of fundamental
concepts just don't apply in Rust, but that doesn't matter because ECS
is usually more then powerful enough to make up for Rust's lack of
inheritance.

Having ideas is fine, but going ahead and accomplishing them is
another entirely. I'll be using this book to describe by ideas as they
grow (or shrink) over time, and to build potential models.

## Tools

- [Specs](https://github.com/amethyst/specs) -- Entity Component
  Systems library
