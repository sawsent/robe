# suit — Specification

## NAME
suit — switch between named dotfile configurations

## SYNOPSIS
suit add <tool>/<profile> [-r <path>] [-f]
suit use <tool>/<profile>
suit list [tool]
suit rm <tool>/<profile>
suit rm <tool>

## DESCRIPTION
suit stores and activates named versions of configuration files.

Each tool maps to one config file.
Each profile is a saved copy.

suit only saves, switches, lists, and removes profiles.

## TERMS
tool      program mapped to one config file
profile   named saved copy of that file

## STORAGE
~/.config/suit/<tool>/<profile>

Profiles are plain files.
No metadata is stored.

## COMMANDS
add <tool>/<profile> [-r <path>] [-f]
    save current config as a profile
    -r, --register <path>  register target file
    -f, --force            overwrite existing profile or registration

use <tool>/<profile>
    activate profile

list [tool]
    list tools or profiles

rm <tool>/<profile>
    delete a stored profile

rm <tool>
    delete all profiles for a tool

## OPTIONS
-h, --help       show help
-v, --version    show version

## GUARANTEES
- single-file configs only
- deterministic behavior
- no hidden state
- no background processes
- profiles remain normal files

## NON-GOALS
directories, templating, repos, syncing, environments, automation

