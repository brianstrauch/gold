# Gold 🥇

[![Gold](https://img.shields.io/badge/code%20style-gold-yellow)](https://github.com/brianstrauch/gold)

A fast linter for Go, written in Rust.

## Usage

    gold [path] [--fix]

## Rules

| Rule                 | Description                | Fix |
| -------------------- | -------------------------- | --- |
| [F0000](tests/F0000) | Redundant parameter types  | ✅  |
| [F0001](tests/F0001) | Unsorted imports           | ✅  |

## Configuration

* Gold searches the root directory of your Go modules for a .gold.yml file
* Gold can also understand .golangci.yml configuration files, if they exist
* The following is an example of a .gold.yml configuration file:

```yaml
# rules to enable, default: [] (all rules)
enable:
    - F0000
    - F0001

# rule-specific settings
settings:
    # order to sort imports by, default: [standard, default]
    F0001:
        - standard
        - default
        - prefix(github.com/brianstrauch/gold/tests)

# directories to ignore, default: []
ignore:
    - mock
```
