# 🐸

A prototype re-implementation of [phog](https://gitlab.com/mobian1/phog) in Rust, [using libphosh](https://gitlab.com/mobian1/phog/-/issues/5).

To test with `fakegreet`:

```
cargo build

phoc -E "bash -lc './target/debug/fakegreet ./target/debug/phrog'"
```

The successful login combination is `user` / `0` / `9`.
