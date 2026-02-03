# robe — Specification

## NAME

**robe** — switch between named dotfile configurations

---

## SYNOPSIS

```bash
robe add <target>/<profile> [-r <path>] [-f]
robe use <target>/<profile>
robe list [target]
robe rm <target>/<profile>
robe rm <target>
robe view <target>
robe view <target>/<profile>
```

---

## DESCRIPTION

**robe** stores and activates named versions of configuration files.

- Each **target** maps to one config file / directory  
- Each **profile** is a saved copy  
- robe only saves, switches, lists, removes, or views profiles  

No magic. No hidden behavior.

---

## TERMS

| Term      | Meaning                                       |
|-----------|-----------------------------------------------|
| target    | program mapped to one config file / directory |
| profile   | named saved copy of that file / directory     |

---

## STORAGE

- **Linux:** `~/.config/robe/wardrobe/<target>/<profile>`  
- **macOS:** `~/Library/Application Support/robe/wardrobe/<target>/<profile>`

Profiles are plain files.  
No metadata is stored.

---

## COMMANDS

### add `<target>/<profile>`
Save current config as a profile.

Options:
- `-r, --register <path>` — register target file
- `-f, --force` — overwrite existing profile or registration

---

### use `<target>/<profile>`
Activate profile.

---

### list `[target]`
List targets or profiles.

---

### rm `<target>/<profile>`
Delete a stored profile.

### rm `<target>`
Delete all profiles for a target.

---

### view `<target>`
Display the currently active config for a target.

### view `<target>/<profile>`
Display a stored profile without activating it.

---

## OPTIONS

- `-h, --help` — show help  
- `-v, --version` — show version  

---

## GUARANTEES

- deterministic behavior
- no hidden state
- no background processes
- profiles remain normal files

---

## NON-GOALS

- templating
- repositories
- syncing
- environments
- automation
