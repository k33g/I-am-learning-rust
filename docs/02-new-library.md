

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
cargo package --list --allow-dirty
# you need to commit everything
cargo publish
```

## How to use it

> https://crates.io/crates/my_tools

```toml
[dependencies]
my_tools = "0.1.0"
```


## How to use it locally and remotely

https://stackoverflow.com/questions/33025887/how-to-use-a-local-unpublished-crate
https://doc.rust-lang.org/cargo/reference/overriding-dependencies.html