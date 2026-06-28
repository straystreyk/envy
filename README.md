# 🌿 envy

> A blazing fast CLI tool for working with `.env` files — built with Rust 🦀

![Rust](https://img.shields.io/badge/built%20with-Rust-orange?style=flat-square&logo=rust)
![License](https://img.shields.io/badge/license-MIT-blue?style=flat-square)

---

## ✨ Features

- 🔍 **diff** — compare two or more `.env` files side by side
- 📋 **list** — beautifully display all variables from a file
- 🎨 Color-coded output — instantly see what changed
- ⚡ Fast and lightweight — single binary, no dependencies
- 🖥️ Works on macOS, Linux, and Windows

---

## 📦 Installation

### Via cargo

```bash
cargo install envy
```

### Build from source

```bash
git clone https://github.com/MasterSaya/envy
cd envy
cargo build --release
```

---

## 🚀 Usage

### `envy diff` — Compare .env files

Compare two or more `.env` files and see exactly what's different:

```bash
envy diff .env .env.example
```

```
+──────────────┬──────────────────────┬──────────────────────┐
│ key          │ .env                 │ .env.example         │
├──────────────┼──────────────────────┼──────────────────────┤
│ DATABASE_URL │ postgres://localhost  │ postgres://localhost  │  ← same
│ API_KEY      │ secret123            │ changeme             │  ← changed (yellow)
│ OLD_SECRET   │ abc123               │ -                    │  ← missing (red)
│ NEW_VAR      │ -                    │ somevalue            │  ← added (green)
└──────────────┴──────────────────────┴──────────────────────┘
```

You can compare **multiple files** at once:

```bash
envy diff .env .env.example .env.production .env.staging
```

---

### `envy list` — List variables

Display all variables from a `.env` file:

```bash
envy list .env
```

```
+──────────────┬───────────┐
│ key          │ value     │
├──────────────┼───────────┤
│ DATABASE_URL │ ***       │
│ API_KEY      │ ***       │
│ PORT         │ ***       │
└──────────────┴───────────┘
```

> 🔒 Values are hidden by default to protect secrets

#### Show real values

```bash
envy list .env --show-values
```

#### Show only keys

```bash
envy list .env --keys

  1  DATABASE_URL
  2  API_KEY
  3  PORT
```

#### Count variables

```bash
envy list .env --count

3
```

---

## 🗂️ Supported `.env` format

```bash
# Comments are ignored
DATABASE_URL=postgres://localhost/mydb

# Quoted values
API_KEY="my secret key"
SECRET='another secret'

# Spaces around = are trimmed
PORT = 3000

# URLs with multiple = signs work correctly
CALLBACK_URL=https://example.com?foo=bar&baz=qux
```

---

## 🛠️ Built with

| Crate                                     | Purpose                |
|-------------------------------------------|------------------------|
| [clap](https://crates.io/crates/clap)     | CLI argument parsing   |
| [tabled](https://crates.io/crates/tabled) | Beautiful table output |

---

## 🤝 Contributing

PRs are welcome! Feel free to open an issue if you find a bug or want a new feature.

---

## 📄 License

MIT © MasterSaya
