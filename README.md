# Info Utils

Utilities for logging and error-handling in Rust.

## Features

- `log!`, `warn!`, `terror!`, and `error!` macros for convenient and informational data display

- `eval()` as a drop in for `unwrap()`

- `should()` as a drop in for `expect()`

- Info Utils also provides `eval_or()`, `eval_or_default()`, and `eval_or_else()` functions as drop-ins for the corresponding `unwrap_*()` functions.

## Usage

Add the following to your `cargo.toml`:

```toml
[dependencies]
info_utils = "1.3.1"
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
    exit!("x is equal to {}, exiting..", x); // Exits with error code
}
```

###### Output

```text
❯ cargo run
   Compiling demo v0.1.0 (/Path/To/Project)
    Finished dev [unoptimized + debuginfo] target(s) in 0.15s
     Running `target/debug/demo`
INFO
["main"]:
Value is 10
WARN
["main"]:
10 is greater than 3
ERR
["main"]:
x is equal to 10, exiting..
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
INFO
["main"]:
Hello from main thread!
WARN
["<unknown>"]:
This thread doesn't have a name!
INFO
["named thread"]:
Hello from inside a named thread!
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
     Running `target/debug/demo`INFO
INFO
["main"]:
Option: valid value
Value: everything's fine
ERR
["main"]:
"oh no something happened!"
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
     Running `target/debug/demo`INFO
INFO
["main"]:
Option: valid value
Value: everything's fine
INFO
["main"]:
Result: it's alright we can handle this
WARN
["main"]:
Option: option was None
INFO
["main"]:
Result: 
WARN
["main"]:
error was: "oh no something happened!", but we're all good anyways
INFO
["main"]:
Result: error handled
```

## Contributing

If you notice any issues or bugs in the package or docs please create an issue or PR addressing it.

Also feel free to file issues for feature requests.
