# Archgate proto plugin

A [proto](https://moonrepo.dev/proto) WASM plugin for installing and managing the [Archgate CLI](https://cli.archgate.dev).

## Installation

Add the plugin to your `.prototools` file:

```toml
[plugins]
archgate = "github://archgate/proto-plugin"
```

Then install:

```bash
proto install archgate
```

## Usage

```bash
# Install a specific version
proto install archgate 0.15.0

# Use a specific version
proto use archgate 0.15.0

# List available versions
proto list-remote archgate

# Pin a version
proto pin archgate 0.15.0
```
