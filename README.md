# Try Out Rust & WebAssembly

This is a repo that lets you try out Rust and WebAssembly in
[VisualStudio Code](https://code.visualstudio.com/). This requires the
[VS Code Remote - Containers](https://aka.ms/vscode-remote/containers)
extension to work.

## Setting up the development container

Open the root directory of this repository in VS Code. It should ask you if you
want to open the repo in a remote container. Choose yes.

## Things to try

Once you have this sample opened in a container, you'll be able to work with it like you would locally.

Some things to try:

1. **Edit:**
   - Open `src/lib.rs`
   - Try adding some code and check out the language features.
1. **Terminal:** Press <kbd>ctrl</kbd>+<kbd>shift</kbd>+<kbd>\`</kbd> and type
  `uname` and other Linux commands from the terminal window.
1. **Build and Run:**
   - Open a terminal and type `wasm-pack build -t web`
   - Once the project is compiled, run `simple-http-server` and open
     [http://localhost:8000/index.html](http://localhost:8000/index.html) in
     your browser.
 
## License

The project is based off of the
[Try Out Development Containers: Rust](https://github.com/microsoft/vscode-remote-try-rust)
created by Microsoft. The container configuration was adjusted to include
libraries and tools for the WebAssembly development with Rust.

Copyright © Microsoft Corporation All rights reserved.<br />
Licensed under the MIT License. See LICENSE in the project root for license information.
