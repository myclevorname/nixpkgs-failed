# nixpkgs-failed

This program gets a random failing package from [nixos:trunk-combined](https://hydra.nixos.org/jobset/nixos/trunk-combined/evals).
It is currently a work-in-progress.

## Build Instructions

The official method is to run:
```console
$ nix build
```
but you can also run:
```bash
$ cargo build
```

## Issues

Q: Why is this so slow?

A: I am calling the Hydra API directly.
I can't get the latest evaluation unless I get the list of every evaluation, and I have to manually check each build to see if it failed.
I could use the HTML page, but that isn't as simple as calling an API and decoding JSON.


## TODO

[X] Get the latest evaluation

[X] Get a failing build

[ ] Print metadata about it

[ ] Make it pretty

[ ] CLI options
