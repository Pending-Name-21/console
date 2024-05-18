# By Luiggy Mamani Condori
## Interopability

Interopability helps two programs work together through a bridge.

## Foreign Function Interface (FFI)

It helps us communicate two programs that may be written in different programming languages.

A language that helps us achieve this is Rust, as you can call functions from C, C++, Java and use them in Rust or vice versa.

![](https://d3i71xaburhd42.cloudfront.net/278bd70d0d53a4f512e8519d30e99e0acfebd3ec/1-Figure1-1.png)

This is achieved by declaring external functions which are functions that were created in another program.

To do this we need to declare the function with the same parameters and output based on the language it comes from, something similar to declaring a function in a C header.

There are two ways to link libraries, the first is dynamic (at runtime) and the other is static.

In Rust there are tools like rustc and cargo that help us compile and manage projects, but they also have support for working with FFI.