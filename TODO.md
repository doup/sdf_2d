# TODO

- [ ] Add sidebars: left outline, right inspector
- [ ] Fix canvas resizing, it should keep it's original size.
  - Is Pixels really needed? Without it I could just compile for WASM, no?
    - https://emilk.github.io/egui/index.html => Painting demo
- Coloring
  - [X] Struct -> inside/outside/border
  - [X] Solid
  - [ ] Distance gradient
- Render text improvements
  - [ ] Parse *.fnt
  - [x] `Text.font` should be a reference, change the struct and learn about lifetimes
  - [ ] `BBox.char` should be a reference, change the struct and learn about lifetimes
  - [ ] Use a quad-tree for search optimization (or something similiar, BSP?)
  - [ ] Add font-size
- [ ] Move (g), Rotate (r) & Scale (s) like in Blender
- [ ] Change base to map from i/j to x/y space (using matrix multiplication instead of manual conversion)
  - See 3Blue1Brown
- [ ] Optimizations
  - SIMD, SSE2… => Bevy::math
- [ ] Gamepad integration :-D
- [/] Distorsions
  - [X] Integrate wave distortion
  - [ ] Add more distorsions
- [/] Transformation matrices
  - [ ] Add matrices behind feature gate `cfg`
  - [ ] Further research why it's slower
- [X] Fix Text lifetimes
- [X] Migrate to Pixels & Egui
- FIX color blending (again) => with gamma correction
  - [X] `mix` (between layers)
  - [X] `blend` (between inside/border/outside transition)
- [X] Change from Boxes to Enums? -> https://www.reddit.com/r/rust/comments/lpgw1n/hey_rustaceans_got_an_easy_question_ask_here_82021/gok6y5m/?utm_source=reddit&utm_medium=web2x&context=3
  - I'm not sure about this approach, I would need a single huge `get_distance` implementation for the struct.
- [X] Fix color blending
- [X] Render text
  - https://github.com/Chlumsky/msdfgen
  - Hiero FNT format => http://www.angelcode.com/products/bmfont/doc/file_format.html
  - [X] Load Text
  - [X] Load Image
  - [X] Render
    - [X] Create BBoxes
- [X] Split `main` into files
- [X] Skip layer rendering if top layer has already 1.0 alpha
- [X] Multi-core render
  - [X] Move to iterable loop
    - https://bitshifter.github.io/2018/05/07/path-tracing-in-parallel/
  - [X] Add rayon `par_chunks_mut` & fix lifetimes (or whatever it is the issue)
- [X] Show selected object outline
- [X] Select different objects
- [X] Tree arena: https://dev.to/deciduously/no-more-tears-no-more-knots-arena-allocated-trees-in-rust-44k6

---

shape vs color

shape
    - transform: translate, rotate & scale
    - primitives: circle, square, triangle, donut…
    - operators: union, substract (smoothed or not)
    - distorsion: noise, wave, onion, bend, mirror, repetition (finite/infinite)…

color
    - outside
        - none
        - solid
            - color
        - linear gradient
        - radial gradient
        - repeating gradient
            - is_repeating
            - size
            - profile
    - border
        - position: inside, outside, center
        - size
        - color
    - inside => same as outside

layer
    object/group

    color
    shape
        group
            primitive
            primitive

SDF = transform(distort(distort((primitive(p))))

transform(distort(union(p, sdf_1: SDF, sdf_2: SDF)))

return distance & Option(color)

Layer {
    color: ColorConfig
    shape: Primitive
}

Primitive {
    transform: Matrix?
    distort: SDF[]
    distance: Primitive | SDF
}
