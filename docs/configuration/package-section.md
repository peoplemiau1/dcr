# Package section

## Schema

```toml
[package]
name = "my-app"
version = "0.1.0"
type = "app"
```

## Fields

- `name` (string, required): used as output artifact base name.
- `version` (string, required): used in `dcr.lock` project entry.
- `type` (string, optional): project type (`app`, `lib`, `none`).

## Notes

- Empty `name` or `version` makes config invalid.
- `dcr new` sets `name` from the passed project name.
- `dcr init` sets `name` from current directory name.
- If `type = "lib"`, DCR generates `include/` and `lib/` directories in the target path, copying public headers and built libraries.
