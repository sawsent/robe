# suit

**Wear your configs.**

`suit` is a tiny CLI for saving and switching between dotfile configurations.

Keep multiple versions.  
Switch instantly.  
Nothing else.

---

## Install

```bash
cargo install suit
```

---

## Quickstart

Save your current config (register the file):
```bash
suit add tmux/work -r .config/tmux/tmux.conf
```

Save another config:
```bash
suit add tmux/clean
```

Switch anytime:

```bash
suit use tmux/work
suit use tmux/clean
```

---

## What it does

- stores named config profiles
- switches between them quickly
- keeps everything as plain files

---

## What it doesn’t do

No templating.  
No repo management.  
No environments.  
No magic.

---

## Docs

Full spec and behavior:  
See [docs/spec.md](docs/spec.md)

---

## License

Licensed under the Apache License, Version 2.0.  
See the `LICENSE` file for details.
