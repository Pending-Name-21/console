# JNI Spike

**Author:** Fabián Romero Claros

This project serves as a spike to explore the integration of Java Native Interface (JNI) in a Java project, focusing on the interaction with native code from Rust.

## Overview

JNI allows Java code to interact with native code written in languages like C, C++ or `Rust`. This can be useful when performance-critical tasks need to be executed in native code or when leveraging existing native libraries.

### Subscribers (jni_events)

- The `Handler` class manages subscribers for various events.
- Subscribers implement the `ISubscriber` interface, which defines the `notify` method.
- Native methods are used to trigger events and communicate with native code.

### Shapes (jni_shape)

The shapes aspect demonstrates polymorphism and inheritance in the context of JNI. Different shapes such as circles, squares, and triangles are represented as Java classes. Native methods are used to calculate areas and perimeters for these shapes.

## Features

- Integration of JNI in Java project.
- Demonstration of polymorphism and inheritance.
- Usage of native methods to interact with native code.
- Implementation of the way to notify all the subscribers given an event type.

## Prerequisites

- Java Development Kit (JDK) installed.
- Rust programming language installed for native code implementation and Cargo.

## Getting Started

It is possible to run each example through the Makefile:

```bash
cd {folder_name}
make
```

To implement a code like the examples:

1. Create a rust lib project:

```bash
cargo new project_name --lib
```

2. In the `Cargo.toml` file, add the following:

```toml
[dependencies]
jni = "0.21.1"

[lib]
crate_type = ["cdylib"]
```

3. Create the Java implementation with the necessary native methods.
4. Implement the native methods in the `lib.rs` file and the static lib load:

```java
static {
    System.loadLibrary("shape");
}
``` 

5. Create a Makefile with:

   1. Compile the java code:

   ```
   javac ClassName.java && java -Djava.library.path=project_name/target/debug/ ClassName
   ```

    If needed to specify the version of java:

    ```
    javac -source 1.8 -target 1.8 ClassName.java && ...
    ```

   2. The lib name:

   ```
   .PHONY: project_name
   ```

   3. Get the C header code:

   ```
   javac -h . ClassName.java
   ```

   4. Run the rust code:

   ```
   cd projectname && cargo build
   ```

## Further Exploration

This spike provides a basic foundation for integrating JNI into a Java project. Further exploration and enhancements could include:

- Handling more complex events and event hierarchies.
- Implementing additional shape types and calculations.
- Optimizing native code performance.
- Error handling and memory management in JNI interactions.

Certainly! Here's the updated section in the Notes:

## Conclusions

- JNI can be utilized for various purposes in Java projects, including performance optimization, integration with existing native libraries, and accessing platform-specific functionality.
- By defining all the necessary native methods to be implemented, JNI allows seamless communication between Java code and native code, enabling developers to leverage the power of both worlds.
- Careful attention should be given to the declaration and implementation of native methods to ensure compatibility and proper functionality.
- Proper error handling and memory management are crucial when working with JNI to prevent memory leaks, crashes, and undefined behavior.
- JNI can be used effectively in any project as long as the native methods are properly defined and implemented according to the project's requirements.