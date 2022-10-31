

```bash
cargo new --lib my_tools
```


## How to publish a crate

- Go on https://crates.io/
- Then go on https://crates.io/settings/profile
- Generate a token, store it to `CARGO_REGISTRY_TOKEN`

To use an API token, run `cargo login` on the command line and paste the key when prompted. This will save it to a local file (`~/.cargo/credentials.toml` by default).

- https://doc.rust-lang.org/cargo/reference/publishing.html#before-publishing-a-new-crate

### Packaging a crate

```bash
cargo publish --dry-run
```

## How to use it locally and remotely


