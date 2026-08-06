# Archgate proto plugin

A [proto](https://moonrepo.dev/proto) WASM plugin for installing and managing the [Archgate CLI](https://cli.archgate.dev).

## Requirements

proto 0.60.0 or newer.

proto 0.60 reworked the WASM plugin API, and a single plugin binary cannot
satisfy both the old and new shapes. Pick the locator that matches the proto you
run:

| proto version    | Plugin locator                          |
| ---------------- | --------------------------------------- |
| 0.60.0 and newer | `github://archgate/proto-plugin`        |
| 0.46.0 – 0.59.x  | `github://archgate/proto-plugin@v0.1.0` |

Mismatched pairs fail at install time. See [Troubleshooting](#troubleshooting)
for the two errors this produces and how to resolve each.

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

## Troubleshooting

### `missing field` errors from a plugin function call

```
Error: plugin::wasm::failed_function_call
  × missing field `tool_dir` at line 1 column 114
```

The plugin is older than the proto running it. proto 0.60 removed
`PluginUnresolvedContext.tool_dir`, so a plugin built against the pre-0.60 API
cannot deserialize the context proto hands it. The download succeeds and
`register_tool` succeeds — the failure lands on the next plugin call, which is
why the message says nothing about versions.

Resolve it in one of two directions.

**Upgrade the plugin** (preferred). Plugin v0.2.0 and newer target the 0.60 API.
An unpinned locator picks the new release up on the next install with no cache
clearing — proto queries the releases API each run, and the cached plugin file is
keyed by its download URL, so a new release resolves to a path that isn't cached
yet:

```bash
proto install archgate
```

If the locator is pinned to `@v0.1.0`, drop the pin or move it to `@v0.2.0`.

**Or hold proto below 0.60**, if something else in your setup needs the old API.
Pin *both* halves — pinning only the plugin leaves proto free to drift back to
0.60 and reintroduce the error:

```toml
# .prototools
proto = "0.59.0"

[plugins]
archgate = "github://archgate/proto-plugin@v0.1.0"
```

The install script defaults to the newest release, so pass an explicit version
wherever CI provisions proto:

```bash
curl -fsSL https://moonrepo.dev/install/proto.sh | bash -s -- 0.59.0 --yes
```

### `requires a minimum proto version`

```
Error: proto::tool::minimum_version_requirement

  × Unable to use the Archgate plugin with identifier archgate, as it requires
  │ a minimum proto version of 0.60.0, but found 0.56.4 instead.
```

The mirror image of the previous error: proto is older than the plugin. Either
upgrade proto, which is the direction that keeps you on current releases:

```bash
proto upgrade
```

Or pin the locator to `@v0.1.0` as shown in the table under
[Requirements](#requirements).

### `no applicable asset found for release ...`

```
Error: plugin::loader::github::asset_missing

  × Cannot download archgate plugin from GitHub (archgate/proto-plugin), no
  │ applicable asset found for release latest.
```

proto raises this whenever its call to the GitHub releases API returns anything
other than a usable release document. The wording points at the release assets,
but the release is rarely the problem — every published release here carries an
`archgate_tool.wasm` asset served with the `application/wasm` content type,
which is exactly what proto looks for.

The causes worth checking, in order:

1. **GitHub API rate limiting.** Unauthenticated callers get 60 requests per
   hour per IP address. Shared CI runners burn through that quota quickly, so
   this is the most common trigger.
2. **An invalid or expired token.** proto reads `WARPGATE_GITHUB_TOKEN`,
   `GH_TOKEN`, then `GITHUB_TOKEN`, and sends the first one it finds as a bearer
   token. A stale value makes the API return `401 Bad credentials`, which
   surfaces as this same message — so setting a token can keep the failure alive
   rather than clear it. Confirm the token works before blaming the release:

   ```bash
   curl -sS -H "Authorization: Bearer $GITHUB_TOKEN" \
     https://api.github.com/repos/archgate/proto-plugin/releases/latest
   ```

3. **A typo in the repository slug.** A locator pointing at a repository that
   does not exist produces this message too, since a `404` is indistinguishable
   from an empty release here.

Supplying a valid token is the fix for the rate-limited case. Pinning the
locator to an explicit tag is not — a pinned locator queries the same API and
fails the same way under the same conditions:

```toml
# Pins the plugin version; does nothing for API rate limits.
archgate = "github://archgate/proto-plugin@v0.1.0"
```
