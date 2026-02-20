# robe — Specification

## NAME
robe — switch between named dotfile configurations

## SYNOPSIS
`robe add <target>/<profile> [-r <path>] [-f]`  
`robe use <target>/<profile>`  
`robe view <target>[/profile] [--raw]`  
`robe edit <target>[/profile]`  
`robe list [target]`  
`robe ls [target]`  
`robe rm <target>[/<profile>]`  

## DESCRIPTION
robe stores and activates named versions of configuration files or directories.

Each target maps to one config path.  
Each profile is a saved copy of that path.  

robe only saves, switches, prints, edits, lists, and removes profiles.

## TERMS
`target` — file or directory being managed  
`profile` — named saved copy of that config  

## STORAGE
`~/.config/robe/wardrobe/<target>/<profile>`  

Profiles are plain files or directories.  
No metadata is stored.  

## COMMANDS

`add <target>/<profile> [-r <path>] [-f]`  
    save current config as a profile  
    `-r` registers the path if not already registered  
    `-f` overwrites existing data (update)  

`use <target>/<profile>`  
    activate profile  

`view <target>[/profile] [--raw]`  
    print contents to stdout  
    file → prints contents  
    dir  → lists entries  
    `--raw` skips headers and formatting for piping

`edit <target>[/profile]`  
    open config or profile in `$EDITOR`  

`list [target]`  
    list targets or profiles  

`ls [target]`  
    alias list  

`rm <target>[/<profile>]`  
    delete a stored profile  

`rm <target>`  
    delete all profiles for a target  

## OPTIONS
`-h, --help`       show help  
`-v, --version`    show version  

## GUARANTEES
- file or directory units only  
- deterministic behavior  
- no hidden state  
- no background processes  
- profiles remain normal filesystem objects  

## NON-GOALS
templating, repos, syncing, environments, automation
