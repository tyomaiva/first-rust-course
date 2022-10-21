# Setup

> *Note*: this Section is optional for the first part of the course. You need it only whenever you want to run the code examples *locally* on your system. Otherwise, use the online sandbox [Rust Playground](https://play.rust-lang.org/).
> However, for the embedded part, this Section is mandatory, since we want to interface our physical microcontroller boards.

## Setting up the environment
If something below goes wrong, go for more details to <https://doc.rust-lang.org/book/ch01-01-installation.html>

### Installing Rust on Windows
Before installing Rust, it also requires the C++ build tools. In case you do not have those, you can get them via this link: <https://visualstudio.microsoft.com/visual-cpp-build-tools/>

To install Rust itself, go to the following link and follow the instructions for installation: <https://www.rust-lang.org/tools/install>

### Installing Rust on Linux or macOS
The following command downloads and installs the latest stable version of Rust:
```shell
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```
Rust also needs a C linker. In case you do not have one, do the following:
+ For **Linux**
```shell
sudo apt-get update
sudo apt install build-essential
```
+ For **macOS**
```shell
xcode-select install
```

### What IDEs can be used?
Mainly for syntax highlighting, code completion, etc. Both have Rust plugins:
+ Visual Studio Code ([rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer))
+ IntelliJ IDEA

## Setting up a basic project
We will use Cargo, which comes with the Rust installation. What does it do?
+ Package manager (e.g., it fetches and builds external dependencies)
+ Build system (it compiles your packages)

Cargo can be used to generate a new Rust project template, by running the following in your shell:
```shell
cargo new my_rust_sandbox
cd my_rust_sandbox
```

Inside the new directory,
+ _src_ sub-directory is generated with a file inside called _main.rs_ which contains a "Hello World!" program. In order to run examples from the next Sections, just copy the code from an example and replace the contents of this file with that.
+ _Cargo.toml_ file is generated which basically configures your program. We don't need any external dependencies for the first part of the course, so this file can be kept untouched.

To (re-)build and run the executable, use
```shell
cargo run
```
