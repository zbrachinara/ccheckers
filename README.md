# Chinese Checkers???????????
Small chinese checkers implementation with nannou

![image](https://user-images.githubusercontent.com/43701041/218815879-d0630e9f-9ce8-4130-8cef-953277a39b87.png)

## Building and deploying

Both platforms assume that you have cargo installed.

For native (Linux/Mac/Windows assuming no cross-compilation): Run `cargo build --release, then get the executable
from `./target/release/ccheckers(.exe)`

For web (a `.vscode/tasks.json` task is provided for convenience for steps 2-3):
- Install wasm-bindgen either with your system's package manager or with `cargo install wasm-bindgen-cli`
- Run `cargo build --release --target wasm32-unknown-unknown`
- Run `wasm-bindgen --no-typescript --out-dir static --target web ./target/wasm32-unknown-unknown/debug/ccheckers_wasm.wasm`
- Serve `./static`

Note that for web, the `.static` directory cannot be wiped, as the html file is created manually.
