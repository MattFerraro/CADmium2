# CADmium2

**Status**: Early experiments with the technology. Completely nonfunctional.

This is my attempt at creating a completely open source (MIT License) CAD package that runs entirely in browser. There is no server component except for hosting the static site contents.

The heavy lifting is done in Rust, relying mostly on the [truck](https://github.com/ricosjp/truck) package for boundary representation.

The CADmium package is a set of Rust wrappers around the b-rep engine, and it provides the concept of a Project which contains Sketches, Planes, Extrusions, etc.

All the Rust code gets compiled to wasm and then actuated from Javascript. The 3D engine is provided by three.js and relies on WebGL.

A demo is running live [here](https://mattferraro.github.io/CADmium2/), via Github Pages.

# Running Locally

## Rust

```
cd cadmium
cargo test
```

## Javascript

Locally:
```
cd cadmium-spa
npm start
```

Deploying the demo to Github pages:
```
npm run build-web && npm run deploy
```

As an Electron app:
```
npm run build-electron
npm run elect
```


# TODO (code only)

- Remove the "id" field from cadmium::sketch::Point
- Simplify the cadmium::common:Plane field to remove extra fields
- cadmium::sketch::Ring should be a struct not a type alias
- combine all the wasm_bindgen annotations into the one primary cadmium crate?
- replace the JS Map stuff with something simpler (map of sketches, etc)
- check: do we get extra segments from .find_faces()?

# TODO (features)

- add sketch.get_bounds(), either to sketch or sketchView or both
- for a sketch, draw a rectangle and put the sketch name on it
- add ability to edit the name of a step
- on mouseover for a part name, highlight it in 3D
- boolean operations
- generalize extrusion step to allow for merging
- ability to put sketches on arbitrary faces of existing geometry
- ability to delete a part
- incorporate revolve

# TODO (Extraordinarily difficult)

- 2D sketch constraint solver
- Fillters
- Chamfers
