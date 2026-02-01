# suit — Specification

## NAME
suit — switch between named dotfile configurations

## SYNOPSIS
suit add <tool>/<profile> [-r <path>] [-f]
suit use <tool>/<profile>
suit list [tool]
suit current <tool>
suit rename <tool> <new_tool> [-f]
suit restore <tool>
suit rm <tool>/<profile> [-f]
suit rm <tool> [-f]

## DESCRIPTION
`suit` stores and activates named versions of configuration files.
Each tool has one config file. Each profile is a saved copy.
Operations: add, use, list, current, rename, restore, rm (profile or tool).

## TERMS
**tool** — program with one config file  
**profile** — saved version of a tool’s config  
**active** — profile currently in use  

## STORAGE
~/.config/suit/<tool>/<profile>  
Profiles are plain files. No metadata.

## COMMANDS
add <tool>/<profile> [-r <path>] [-f]  
    save current config as a profile; -r registers a file if default missing  

use <tool>/<profile>  
    activate profile  

list [tool]  
    list tools or profiles; active marked  

current <tool>  
    show active profile  

rename <tool> <new_tool> [-f]  
    rename a tool; updates stored profiles and active link  

restore <tool>  
    restore a tool to its last pre-suit-load state  

rm <tool>/<profile> [-f]  
    delete a stored profile; fails if active unless -f  

rm <tool> [-f]  
    delete all profiles for a tool; fails if any active unless -f  

## OPTIONS
-d, --dir <path>      storage directory (default: ~/.config/suit)
-f, --force           overwrite or remove without prompting  
-h, --help  
-v, --version  

## GUARANTEES
Single-file configs only. Deterministic. No background processes. Profiles remain plain files.

## NON-GOALS
Directories, templating, repo management, syncing, environments, automation.

