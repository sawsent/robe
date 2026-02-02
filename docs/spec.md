# robe — Specification

## NAME

**robe** — switch between named dotfile configurations

---

## SYNOPSIS

```bash
robe add <tool>/<profile> [-r <path>] [-f]
robe use <tool>/<profile>
robe list [tool]
robe rm <tool>/<profile>
robe rm <tool>
robe view <tool>
robe view <tool>/<profile>
```

---

## DESCRIPTION

**robe** stores and activates named versions of configuration files.

- Each **tool** maps to one config file  
- Each **profile** is a saved copy  
- robe only saves, switches, lists, removes, or views profiles  

No magic. No hidden behavior.

---

## TERMS

| Term      | Meaning                                   |
|-----------|-------------------------------------------|
| tool      | program mapped to one config file         |
| profile   | named saved copy of that file             |

---

## STORAGE

- **Linux:** `~/.config/robe/<tool>/<profile>`  
- **macOS:** `~/Library/Application Support/robe/<tool>/<profile>`

Profiles are plain files.  
No metadata is stored.

---

## COMMANDS

### add `<tool>/<profile>`
Save current config as a profile.

Options:
- `-r, --register <path>` — register target file
- `-f, --force` — overwrite existing profile or registration

---

### use `<tool>/<profile>`
Activate profile.

---

### list `[tool]`
List tools or profiles.

---

### rm `<tool>/<profile>`
Delete a stored profile.

### rm `<tool>`
Delete all profiles for a tool.

---

### view `<tool>`
Display the currently active config for a tool.

### view `<tool>/<profile>`
Display a stored profile without activating it.

---

## OPTIONS

- `-h, --help` — show help  
- `-v, --version` — show version  

---

## GUARANTEES

- single-file configs only
- deterministic behavior
- no hidden state
- no background processes
- profiles remain normal files

---

## NON-GOALS

- directories
- templating
- repositories
- syncing
- environments
- automation
