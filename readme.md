# Setup

```
cargo install
cargo run < hashes.txt
```

The release build is much faster:

```
cargo run --release < hashes.txt
```

The release build loads and searches through 1.6 million hashes in about 1.5s  The debug build takes about 30s.