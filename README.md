# rust-linux-memory-management

Seven small teaching examples that call the C-level allocation and
memory primitives (`malloc`, `free`, `realloc`, `memcpy`, `memset`,
`memcmp`, `memmove`) directly from Rust via the [`libc`] crate.

**None of this is idiomatic Rust.** For real code you'd use `Vec`,
`Box`, or the `std::alloc` API. This repo exists to make the
FFI-level machinery visible.

## Layout

- `src/lib.rs` — thin unsafe wrappers around the seven libc calls.
- `examples/`
  - `01_alloc.rs` — `malloc` + `free`
  - `02_realloc.rs` — `realloc` (grow)
  - `03_memcpy.rs` — `memcpy` between two allocations
  - `04_memset.rs` — `memcpy`, then `memset` back to zero
  - `05_memcmp_equal.rs` — `memcmp`, equal case
  - `06_memcmp_diff.rs` — `memcmp`, differing case
  - `07_memmove.rs` — `memmove` on non-overlapping regions
- `code for membory manament` — the original single-file dump, kept
  for history. All seven demos above were spun out of it.

## Running

```sh
cargo run --example 01_alloc
cargo run --example 02_realloc
# … etc
```

Or all of them:

```sh
for ex in $(ls examples | sed 's/\.rs$//'); do
    echo "== $ex =="
    cargo run --example "$ex"
done
```

## Requires

- Rust (any edition-2021-capable toolchain — install via [rustup]).
- A libc — you have one already on Linux/macOS. On Windows this uses
  the CRT and works, but the demos are labelled "linux" only because
  that's where they were written.

## What changed in this refresh

The original `code for membory manament` was a single file
concatenating seven `fn main` bodies with duplicate `allocate_memory`
definitions and no closing brace on the last snippet — it couldn't
compile at all. This refresh added a `Cargo.toml`, split the shared
helpers into `src/lib.rs`, and moved each demo into its own file under
`examples/`. The original file is preserved so the "before" is still
readable.

## License

MIT — see [LICENSE](LICENSE).

[`libc`]: https://crates.io/crates/libc
[rustup]: https://rustup.rs/
