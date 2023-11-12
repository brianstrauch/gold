# Gold 🥇

[![Gold](https://img.shields.io/badge/code%20style-gold-yellow)](https://github.com/brianstrauch/gold)

A fast linter for Go, written in Rust.

## Usage

    gold [path] [--fix]

## Rules

| Rule               | Description                  | Fix |
| ------------------ | ---------------------------- | --- |
| [F001](tests/F001) | No redundant parameter types | ✅  |
| [F002](tests/F002) | No unsorted imports          | ✅  |

## Configuration

* Gold searches the root directory of your Go modules for a .gold.yml file
* Gold can also read .golangci.yml configuration files, if they exist
* The following is an example of a .gold.yml configuration file:

```yaml
# rules to enable, default: [] (all rules)
enable:
    - F001
    - F002

# rule-specific settings
settings:
    # order to sort imports by, default: [standard, default]
    F002:
        - standard
        - default
        - prefix(github.com/brianstrauch/gold/tests)

# directories to ignore, default: []
ignore:
    - mock
```
