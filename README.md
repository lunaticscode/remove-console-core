# remove-console-core

A simple Rust command-line tool to remove console log statements from source code.

## 🚀 Features

- Removes specified `console.log`, `console.warn`, `console.error`, etc.
- Supports multiple log levels
- Cleans up unnecessary backslash-sequences like `\n`, `\t`, etc.
- Defaults to removing console.log if no log levels are specified.

## 🛠 Usage

Run the tool with the source code and the log levels to remove:

```sh
cargo run -- "console.log('Hello'); console.warn('Warning!');" "log,warn,error"
```

If no log levels are provided, it defaults to removing `console.log`:

```sh
cargo run -- "console.log('Hello'); console.warn('Warning!');"
```

### Example

#### Input

```js
console.log("Debugging...");
console.warn("Warning message");
console.error("An error occurred");
```

#### Command

```sh
cargo run -- "console.log(\"Debugging...\"); console.warn(\"Warning message\"); console.error(\"An error occurred\");" "log,warn"
```

#### Output

```js
console.error("An error occurred");
```

## 📜 License

This project is licensed under the MIT License.

## 📫 Contributing

Feel free to submit issues or pull requests on [GitHub](https://github.com/lunaticscode/vite-remove-console).
