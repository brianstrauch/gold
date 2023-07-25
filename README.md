# Gold 🥇

[![Gold](https://img.shields.io/badge/code%20style-gold-yellow)](https://github.com/)

A fast linter for Go, written in Rust.

## Usage

    gold <path> [--fix]

## Rules

| Rule                        | Description                | Fix |
| --------------------------- | -------------------------- | --- |
| [G0000](tests/G0000/1.go)   | Redundant parameter types  | ✅  |
| [G0001](tests/G0001)        | Unsorted imports           | ✅  |
