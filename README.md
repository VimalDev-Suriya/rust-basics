# Workspace setup in Rust using Cargo

Cargo is the component comes along with rustup. For more details please [click here](https://doc.rust-lang.org/cargo/getting-started/index.html)

## Workspace initialisation

Create the workspace.
```
cargo new <workspace-name>
```
Initializes the cargo features in already created workspace
```
cargo init .
```
Compilies and build the executable files, when executed with `--release` flag creates the optimized bundle
```
cargo build
```
Compilies, build and Runs the code
```
cargo run
```
To Run the already existing code
```
git clone <repo>
cd <directory>
cargo build/run
```
