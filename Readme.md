# Graylog to Grafana

This tool can convert graylog dashboards into grafana dashboards.

## Build
If you want to build `graylog-to-grafana` from source, you need Rust 1.31 or higher. You can then use cargo to build everything:

```cmd
cargo build
```

## Run

1. Create a Graylog content pack.
2. Run `graylog-to-grafana`
    ```cmd
    cargo run -- dashboards.json tmp
    ```
3. Import dashboards into grafana
