# robe

**Wear your configs.**

`robe` is a tiny CLI for saving and switching between dotfile configurations.

Keep multiple versions.  
Switch instantly.  
Nothing else.

---

## Install

```bash
cargo install robe
```

---

## Quickstart

Save your current config (register the file):

```bash
robe add tmux/work -r .config/tmux/tmux.conf
```

Save another config:

```bash
robe add tmux/clean
```

Switch anytime:

```bash
robe use tmux/work
robe use tmux/clean
```

View current or stored configs:
```bash
# Current config
robe view tmux
# Stored 'clean' config
robe view tmux/clean
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
See the [LICENSE](LICENSE) file for details.
