<div align="center">

# nym

A modern alias manager for your shell

</div>

**nym** (as in _pseudonym_) is an alias manager that automatically creates and manages shell aliases for you.

## Why?

Traditional shell aliases have some limitations:

- **Hard to remember**
- **No organization**
- **Difficult to search**
- **No descriptions**
- **Manual setup**

nym solves these problems by automatically creating shell aliases organized with categories and descriptions, making them easy to discover and manage.

## Usage

### Add a command (creates an alias)

```bash
nym add
```

Prompts for alias name, category, command, and description. Once added, nym automatically creates a shell alias for you.

You can also use flags:

```bash
nym add rdocs -c rust -C "rustup doc --std" -d "Open rust standard library documentation in the browser."
```

This creates an alias `rdocs` that you can immediately use in your shell:

```bash
$ rdocs  # Opens Rust standard library documentation
```

Commands are stored in `~/.config/nym/config.toml` as TOML entries, and corresponding aliases are automatically created in your shell configuration:

```toml
[[rdocs]]
category = "rust"
cmd = "rustup doc --std"
description = "Open rust standard library documentation in the browser."
```

### List all commands

```bash
$ nym list

┌──────────┬──────────┬──────────────────────────────────────────────────────┬──────────────────────┐
│ Name     │ Category │ Description                                          │ Command              │
├──────────┼──────────┼──────────────────────────────────────────────────────┼──────────────────────┤
│ rdocs    │ rust     │ Open rust standard library documentation in browser  │ rustup doc --std     │
└──────────┴──────────┴──────────────────────────────────────────────────────┴──────────────────────┘
```

Lists all commands saved in nym, organized by category.

### Delete a command (removes the alias)

```bash
nym delete rdocs
```

Deletes a command from nym and automatically removes the corresponding shell alias.
