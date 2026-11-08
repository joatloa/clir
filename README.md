# `clir`

## Contents

- [Getting started](#getting-started)
- [Metadata](#metadata)
- [Commands](#commands)
  - [Defining a Command](#defining-a-command)
  - [The cli command](#the-cli-command)
- [Arguments](#arguments)
  - [Defining an Argument](#defining-an-argument)
  - [Optional arguments](#optional-arguments)
  - [Variadic arguments](#variadic-arguments)
- [Options](#options)
  - [Defining an Option](#defining-an-option)
  - [Required options](#required-options)
  - [Option arguments](#option-arguments)
- [Auto Help](#auto-help)
  - [Help Message](#help-message)
  - [Error Handling](#error-handling)
- [License](#license)

---

### Getting started

Add the dependency in your `Cargo.toml`:

```toml
[dependencies]
clir = "0.1.0"
```

Or using cargo:

```bash
cargo add clir
```

Then define your CLI using the `cli!` macro:

```rust
use clir::cli;

// The macro expands to items (types + functions), so it can be used at module scope.
cli! {
    name "string_utils",
    version "0.1.0",
    description "A simple CLI for string utilities",

    cmd split "Split a string by a separator" {
        arg string "The string to split",
        req_opt "-s, --separator" "The separator to use" {
            arg string
        },
    },
}

fn main() {
    let parsed = parse_env(std::env::args().skip(1).collect());

    match parsed {
        Command::Split(args, opts) => {
            println!(
                "{}",
                args.string
                    .split(&opts.separator)
                    .collect::<Vec<&str>>()
                    .join(" ")
            );
        }
    }
}
```

Notes:

- `help` is a built-in command: run `<exe> help` or `<exe> help <command>` (trying to override won't lead to anything).
- `cli` is a special command [see here](#the-cli-command).
- The macro generates `pub` items, so you can import them from other modules (e.g. `use crate::{Command, parse_env, SplitArgs, SplitOpts};`).

---

### `Metadata`

- `name` - The name of the CLI
- `version` - The version of the CLI
- `description` - The description of the CLI

---

### `Commands`

#### Defining a Command

- We define a command by using the `cmd` keyword.
- It is required for each command to have a name as this will be used when identifying which command was used.
- A command can have a description which is displayed below the usage of the command in the help message.

```
cmd <name> ["description"] {
    ...
}
```

#### The cli command

A special kind of command is the `cli` command which is used to define arguments and options that are used when no command is provided (top-level arguments and options). It's important to note that top-level arguments/options are not global, so we can't define an option in the `cli` command and use it in another command.

If the cli command has positional arguments and the user supplies the first one and it happens to be the name of a command, the command will be executed instead of the cli command.

In other words: command names always take priority over `cli` when matching the first token. Avoid using a first positional argument that can collide with your subcommand names.

---

### `Arguments`

#### Defining an Argument

- We define an argument by using the `arg` keyword.
- Arguments are required to have a name due to the macro auto-generating structs for the arguments and options of a command.
- A description is optional and can be provided to describe the argument.
- If we want to provide our own type for the argument, we can do so by specifying the type after the `:` character. Obviously, we can only supply types that can be parsed from a string.

```
arg <name> ["description"] [: <type>] [= <default>],
```

#### Optional arguments

We can make an argument optional by supplying an `Option<T>` type. In this case, we can also provide a default value for the argument by using the `=` character. Defaults are only allowed for arguments that can be absent; if provided, the generated field type becomes `T` (not `Option<T>`).

Absence is dictated by the type and the context in which the argument is defined. For example:

- If the argument's type is `Option<T>`, it can be absent.
- For an argument defined inside an **optional option**, it doesn't matter wether the argument is optional or required, the resulting type will be `Option<T>`.

#### Variadic arguments

We can make an argument variadic by supplying a `Vec<T>` type. If we want an optional variadic argument, we can supply an `Option<Vec<T>>` type **AND NOT** `Vec<Option<T>>`.

---

### `Options`

#### Defining an Option

- We define an option by using the `opt` keyword.
- Options are required to have flags. Flags can be either short (`-f`) or long (`--flag`) or both (`-f, --flag`). Flags must be provided as a string literal, e.g. `"-f, --flag"`.
- A description is optional and can be provided to describe the option.

```
opt "<flags>" ["description"] [{
    ...args
}],
```

#### Required options

We can make an option required by supplying the `req_opt` keyword instead of `opt`. This will make the option required to be provided when the command is used.

#### Option arguments

The syntax is the same as defining a positional argument, but here we don't allow a description. If you want to convey meaning about something in the option, do it directly in the option description.

---

### Auto Help

#### Help Message

The CLI will automatically generate a help message for the commands, arguments and options. The help message will be displayed when the user runs `help` with or without a command name.

#### Error Handling

Whenever the CLI encounters an error, it will display what the user did wrong, how to fix it, and suggest running the help command for more information.

---

## License

MIT License - Copyright (c) 2026 [Covei Rares](https://github.com/rarescovei5)
