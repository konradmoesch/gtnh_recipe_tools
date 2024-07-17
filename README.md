
# GTNH recipe tools

This repository contains various tools to view recipes from the minecraft modpack [Gregtech New Horizons](https://github.com/GTNewHorizons/GT-New-Horizons-Modpack).
Currently, a library for loading recipes and a recipe viewer are implemented.

## Exporting recipes
To export your recipes as .json file, install the [RecEx](https://github.com/GTNewHorizons/RecEx) mod and open the `Export` menu (default keybind: `k`)

## Building
To build the tools, simply run `cargo build`. Since the  [egui framework](https://github.com/emilk/egui/) is used, building as WASM app is also supported. Simply run `trunk serve` in the `gtnh-recipe-viewer` folder
