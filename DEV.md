# TODO

- [ ] Do we use traits or not??
    - Yeah: two main types `KeyboardLike { type Key, type Mods }` and implementing type `Keyboard<Key, Mods>`
    - Only breaking change _should_ be type names, and a need to import the traits (which can be in a prelude)
