[![Crates.io](https://img.shields.io/crates/v/robe)](https://crates.io/crates/robe)
[![Crates.io](https://img.shields.io/crates/d/robe)](https://crates.io/crates/robe)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue)](LICENSE)
[![Build Status](https://img.shields.io/github/actions/workflow/status/sawsent/robe/rust.yml?branch=main)](https://github.com/sawsent/robe/actions/workflows/rust.yml?query=branch%3Amain)
[![Coverage Status](https://codecov.io/gh/sawsent/robe/branch/main/graph/badge.svg)](https://app.codecov.io/github/sawsent/robe)

**Disclaimer: robe touches your filesystem and is in early development. Use at your own risk**

# robe

**Wear your configs.**

`robe` is a tiny CLI for saving and switching between named configuration files or directories.

Keep versions.  
Switch instantly.  
Nothing else.

---

## Install

```bash
cargo install robe
```

---

## Quickstart

Register and save:

```bash
robe add tmux/work -r .config/tmux
```

Create another profile:

```bash
robe add tmux/minimal
```

Switch:

```bash
robe use tmux/work
```

Inspect:

```bash
robe view tmux/work
robe edit tmux/work
```

---

## What it does

- stores named config profiles
- switches between them quickly
- prints to stdout or edits with `$EDITOR`
- keeps everything as plain files or directories

---

## Docs

Full behavior and spec:  
See [docs/spec.md](docs/spec.md)

---

## Roadmap
What is planned before v0.1.0 (first stable release)
- [ ] `robe status`
    - [ ] Show the current activated profile for each tool
    - [ ] If changed, show the last activated profile for each tool
- [ ] `robe status <target>`
    - [ ] Show the current active profile for a given tool
    - [ ] If changed, show the last activated profile for a tool
- [ ] `robe use` should require `-f` if the current file is different than the last activated profile.
- [ ] Quality of life improvements:
    - [ ] Profiles should be stored with their respective file endings to allow better editing with `robe edit <target>`
    - [x] Better readability in the `dispatch` module.
    - [ ] `--config <file>` allow setting a config file
    - [ ] better errors (especially io)
- [ ] Coverage:
    - [ ] Coverage should be over 90%

---

## License

Apache License 2.0

