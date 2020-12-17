# peach-lib

![Generic badge](https://img.shields.io/badge/version-1.2.1-<COLOR>.svg)

JSON-RPC client library for the PeachCloud ecosystem.

`peach-lib` offers the ability to programmatically interact with the `peach-network`, `peach-oled` and `peach-stats` microservices.

### Usage

Define the dependency in your `Cargo.toml` file:

`peach-lib = { git = "https://github.com/peachcloud/peach-lib", branch = "main"  }`

Import the required client from the library:

```rust
use peach_lib::network_client;
```

Call one of the exposed methods:

```rust
network_client::ip("wlan0")?;
```

### Licensing

AGPL-3.0
