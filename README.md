# Info Utils

Utilities for logging and error-handling in Rust.

## Features

- `log!`, `warn!`, and `error!` macros for convenient and informational data display

- `eval()` as a drop in for `unwrap()`

- `should()` as a drop in for `expect()`

- Info Utils also provides `eval_or()`, `eval_or_default()`, and `eval_or_else()` functions as drop-ins for the corresponding `unwrap_*()` functions.

## Usage

Add the following to your `cargo.toml`:

```toml
[dependencies]
info_utils = "2.0.0"
```

Or run

```bash
cargo add info_utils
```

### Use in Rust Code

#### Macros

##### Basic Use

```rust
use info_utils::macros::*;

fn main() {
    let x = 10;
    log!("Value is {}", x);
    warn!("{} is greater than 3", x);
    error!("x is equal to {}, exiting..", x); // Exits with error code
}
```

###### Output

```text
❯ cargo run
   Compiling demo v0.1.0 (/Path/To/Project)
    Finished dev [unoptimized + debuginfo] target(s) in 0.15s
     Running `target/debug/demo`
info  ["main"]: Value is 10
warn  ["main"]: 10 is greater than 3
error ["main"]: x is equal to 10, exiting..
```

> Terminal output is color formatted

##### With Named Threads

```rust
use info_utils::macros::*;
use std::thread;

fn main() {
    log!("Hello from main thread!");

    let named_thread = thread::Builder::new()
        .name("named thread".into())
        .spawn(||{
            log!("Hello from inside a named thread!");
        }).unwrap();

    let unnamed_thread = thread::Builder::new()
        .spawn(|| {
            log!("Hello from inside an unnamed thread!");
        }).unwrap();

    named_thread.join().unwrap();
    unnamed_thread.join().unwrap();
}
```

###### Output

```text
❯ cargo run
   Compiling demo v0.1.0 (/Path/To/Project)
    Finished dev [unoptimized + debuginfo] target(s) in 0.15s
     Running `target/debug/demo`
info  ["main"]: Hello from main thread!
info  ["named thread"]: Hello from inside a named thread!
info  ["<unknown>"]: Hello from inside an unnamed thread!
```

#### Eval

##### Basic Use

```rust
use info_utils::prelude::*;

fn main() {
    let option: Option<&str> = Some("valid value");
    let mut result: Result<&str, &str> = Ok("everything's fine");

    log!("Option: {}\nValue: {}", option.eval(), result.eval());

    result = Err("oh no something happened!");
    log!("Result: {}", result.eval());
}
```

###### Output

```text
❯ cargo run
   Compiling demo v0.1.0 (/Path/To/Project)
    Finished dev [unoptimized + debuginfo] target(s) in 0.15s
     Running `target/debug/demo`
info  ["main"]: Option: valid value - Value: everything's fine
error ["main"]: "oh no something happened!"
```

##### Other Functions

```rust
use info_utils::prelude::*;

fn main() {
    let mut option: Option<&str> = Some("valid value");
    let mut result: Result<&str, &str> = Ok("everything's fine");

    log!("Option: {}\nValue: {}", option.eval(), result.eval());

    result = Err("oh no something happened!");
    option = None;

    log!("Result: {}", result.eval_or("it's alright we can handle this"));
    warn!("Option: {}", option.eval_or("option was None"));

    log!("Result: {}", result.eval_or_default()); // Logs "" since that's the str default value

    log!("Result: {}", result.eval_or_else(|e| {
        warn!("error was: {:?}, but we're all good anyways", e);
        "error handled"
    }));
}
```

###### Output

```text
❯ cargo run
   Compiling demo v0.1.0 (/Path/To/Project)
    Finished dev [unoptimized + debuginfo] target(s) in 0.15s
     Running `target/debug/demo`
info  ["main"]: Option: valid value - Value: everything's fine
info  ["main"]: Result: it's alright we can handle this
warn  ["main"]: Option: option was None
info  ["main"]: Result: 
warn  ["main"]: error was: "oh no something happened!", but we're all good anyways
info  ["main"]: Result: error handled
```

## Contributing

If you notice any issues or bugs in the package or docs please create an issue or PR addressing it.

Also feel free to file issues for feature requests.
