# Gold 🥇

[![Gold](https://img.shields.io/badge/code%20style-gold-yellow)](https://github.com/)

A fast linter for Go, written in Rust.

## Usage

    gold <path> [--fix]

## Rules

| Rule                        | Description                | Fix |
| --------------------------- | -------------------------- | --- |
| [F0000](tests/F0000/1.go)   | Redundant parameter types  | ✅  |
| [F0001](tests/F0001)        | Unsorted imports           | ✅  |
