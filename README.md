# Graylog to Grafana &emsp; [![Build Status]][travis] [![Latest Version]][crates.io]

[Build Status]: https://travis-ci.org/GDATASoftwareAG/graylog-to-grafana.svg?branch=master
[travis]: https://travis-ci.org/GDATASoftwareAG/graylog-to-grafana
[Latest Version]: https://img.shields.io/crates/v/graylog-to-grafana.svg
[crates.io]: https://crates.io/crates/graylog-to-grafana

This tool can convert Graylog dashboards into Grafana dashboards.

## Build
If you want to build `graylog-to-grafana` from source, you need Rust 1.31 or higher. You can then use cargo to build everything:

```cmd
cargo build
```

## How to use

1. Create a Graylog [content pack](https://docs.graylog.org/en/3.0/pages/content_packs.html).
2. Run `graylog-to-grafana`
    ```cmd
    cargo run -- dashboards.json tmp
    ```
3. [Import dashboards](https://grafana.com/docs/reference/export_import/) into Grafana