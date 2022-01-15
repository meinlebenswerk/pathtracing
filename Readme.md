# pathtracing

**This Project is a work in progress and expected to change a lot in the future**

Following along with [pbrt](https://www.pbrt.org/) in rust as a programming exercise to learn the language.

It's planned to support:
  - [x] Forwards Path-tracing
  - [ ] Shadow-rays
  - [x] Acceleration Structures
  - [ ] STL Loading
    - [x] binary
    - [ ] ASCII
  - [ ] CLI-Arguments
    - [ ] Scene parsing from file
    - [ ] Resume rendering
  - [ ] Multithreading
  - [ ] Some fancy optimization methods, to make it less brute-force and better handle caustics
  - [ ] Ray-Volume interactions

## Materials
Currently diffuse, metallic and dielectric materials are supported, maybe I can think of a way to merge them together at some point -> So something like a generic material can be used.

## Performance
The Ray-tracer is very slow -> It requires a lot of samples to properly converge, since we don't do shadow-rays as of now.
This needs to be optimized, and multi-threading should be enabled, but I didn't want to bother with it in rust, for now.


## Running the Raytracer
Since we can't parse scenes from file, yet, the scenes need to be defined in the code.
Check out the main.rs for an example.
As soon as scene-files are supported, this paragraph will be updated.

The raytracer can be run via cargo
``
cargo run
``
This builds the executable and runs it.
There are no CLI-Arguments as of now, and all the configuration is done inside the code.



### Inspiration and references
The project started out as a follow-along with [Ray Tracing in One Weekend](https://raytracing.github.io/).
Also the [TU Wien Rendering / Ray Tracing Course](https://www.youtube.com/playlist?list=PLujxSBD-JXgnGmsn7gEyN28P1DnRZG7qi) provided a great deal of insight into the underlying concepts.
But currently most of the project-structure is under work to more closely match that of [pbrt](https://www.pbrt.org/), so I have a solid foundation to implement some of the fancier techniques.

