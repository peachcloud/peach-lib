# peach-lib

![Generic badge](https://img.shields.io/badge/version-1.2.6-<COLOR>.svg)

JSON-RPC client library for the PeachCloud ecosystem.

`peach-lib` offers the ability to programmatically interact with the `peach-network`, `peach-oled` and `peach-stats` microservices.

## Overview

The `peach-lib` crate bundles JSON-RPC client code for making requests to the three PeachCloud microservices which expose JSON-RPC servers (`peach-network`, `peach-oled` and `peach-menu`). The full list of available RPC APIs can be found in the READMEs of the respective microservices ([peach-network](https://github.com/peachcloud/peach-network), [peach-oled](https://github.com/peachcloud/peach-oled), [peach-menu](https://github.com/peachcloud/peach-menu)), or in the [developer documentation for PeachCloud](http://docs.peachcloud.org/software/microservices/index.html). 

The library also includes a custom error type, `PeachError`, which bundles the underlying error types into three variants: `JsonRpcHttp`, `JsonRpcCore` and `Serde`. When used as the returned error type in a `Result` function response, this allows convenient use of the `?` operator (as illustrated in the example usage code below).

## Usage

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

Further example usage can be found in the [`peach-menu`](https://github.com/peachcloud/peach-menu) code (see `src/states.rs`).

## Licensing

AGPL-3.0
