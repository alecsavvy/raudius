# 🎧 raudius 

(pronounced raw-dee-us) A rust API client for the Audius project.

Full support for the [Audius API documented here](https://audiusproject.github.io/api-docs/#audius-api-docs).

- Users
- Playlists
- Tracks
- Tips
- Resolution

# How to use 🛠

## rust support

The bulk of this project is written in rust. You can use this crate like any other project by importing it via cargo.

```toml 
raudius = "x.x.x"
```

## node.js support

Node.js support is provided via Neon. This is so development between wasm and node can proceed independently if necessary.

## wasm support

TODO :(

# Examples 🎉

In the [examples](./examples/) dir you can find some ways of how you can use this project.

# Future features

- caching
- mocking / faking requests