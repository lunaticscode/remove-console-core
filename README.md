# 🚀 Remove Console from Levels (WASM)

This Rust project provides a WebAssembly (WASM) function to remove console logging statements of specified log levels from a given JavaScript string. It is designed for use with `wasm-pack` to integrate with JavaScript applications.

## ✨ Features

- Removes `console.log`, `console.warn`, `console.error`, etc., from a given string based on specified log levels.
- Utilizes `regex` for efficient pattern matching.
- Returns a cleaned-up JavaScript string without unwanted logging statements.

## 🔧 Installation

Ensure you have Rust and `wasm-pack` installed:

```sh
cargo install wasm-pack
```

Then, build the WebAssembly package:

### 🌍 For Web

```sh
wasm-pack build --target web
```

### 🖥️ For Node.js

```sh
wasm-pack build --target nodejs
```

## 📌 Usage

### 🌐 Importing in JavaScript (Web)

After building with `wasm-pack`, you can import and use the function in JavaScript:

```javascript
import init, { remove_console_from_levels } from "./pkg/your_wasm_package.js";

async function run() {
  await init();

  const jsCode = `
        console.log("Debug message");
        console.warn("Warning message");
        console.error("Error message");
    `;

  const cleanedCode = remove_console_from_levels(jsCode, ["log", "warn"]);
  console.log(cleanedCode); // This will remove console.log and console.warn, but keep console.error.
}

run();
```

### 🖥️ Importing in JavaScript (Node.js)

For Node.js, use the CommonJS module system:

```javascript
const wasm = require("./pkg/your_wasm_package.js");

async function run() {
  await wasm.default();

  const jsCode = `
        console.log("Debug message");
        console.warn("Warning message");
        console.error("Error message");
    `;

  const cleanedCode = wasm.remove_console_from_levels(jsCode, ["log", "warn"]);
  console.log(cleanedCode); // This will remove console.log and console.warn, but keep console.error.
}

run();
```

## 📜 Function Signature

```rust
#[wasm_bindgen]
pub fn remove_console_from_levels(str: &str, log_levels: Vec<String>) -> String;
```

### 🔹 Parameters

- `str`: The input JavaScript code as a string.
- `log_levels`: A vector of log levels to remove (e.g., `log`, `warn`, `error`).

### 🔄 Returns

- A modified string with specified console statements removed.

## 📄 License

This project is licensed under the MIT License.

## 🤝 Contributions

Contributions are welcome! Feel free to submit issues or pull requests to enhance functionality or improve performance.

## Author

- [Insoo Park](https://www.linkedin.com/in/insoo-park-437496138/)
- lunatics384@gmail.com
