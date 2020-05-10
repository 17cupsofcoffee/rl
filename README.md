# rl

[![Build Status](https://img.shields.io/github/workflow/status/17cupsofcoffee/rl/CI%20Build/master)](https://github.com/17cupsofcoffee/rl/actions?query=branch%3Amaster)

This is a simple (and extremely work-in-progress) roguelike, written in Rust. It's designed to demonstrate one possible way that the ECS architecture could be applied to a roguelike.

It uses:

* [Tetra](https://github.com/17cupsofcoffee/Tetra) - rendering/input/windowing/etc
* [hecs](https://github.com/Ralith/hecs) - ECS

There is also an [older branch](https://github.com/17cupsofcoffee/rl/tree/specs) that utilizes the [Specs ECS library](https://github.com/amethyst/specs).

## Building

To build this project, you'll need the SDL2 development libraries set up on your machine. Instructions can be found [on Tetra's website](https://tetra.seventeencups.net/installation).