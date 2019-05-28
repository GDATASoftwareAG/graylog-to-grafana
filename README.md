# Graylog to Grafana &emsp; [![Build Status]][travis] [![Latest Version]][crates.io]

[Build Status]: https://travis-ci.org/GDATASoftwareAG/graylog-to-grafana.svg?branch=master
[travis]: https://travis-ci.org/GDATASoftwareAG/graylog-to-grafana
[Latest Version]: https://img.shields.io/crates/v/graylog-to-grafana.svg
[crates.io]: https://crates.io/crates/graylog-to-grafana

This tool can convert Graylog dashboards into Grafana dashboards.

```
graylog-to-grafana 0.1.1
jan.jansen <jan.jansen@gdata.de>
This tool can convert Graylog dashboards into Grafana dashboards.

USAGE:
    graylog-to-grafana [OPTIONS] <input> <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --datasource <datasource>     [default: graylog]

ARGS:
    <input>    Graylog content pack to process

SUBCOMMANDS:
    add         Allows to add automatically dashboards to Grafana
    generate    Allows to save Grafana dashboards into a directory
    help        Prints this message or the help of the given subcommand(s
```

## How to use

### Create a content pack
Create a Graylog [content pack](https://docs.graylog.org/en/3.0/pages/content_packs.html).


### Automatically import dashboards into Grafana

```cmd
graylog-to-grafana dashboards.json add --token [bearer-token] --url [grafana-url] --folder [folder-id]
```

### Just convert dashboard into Grafana Json

```cmd
graylog-to-grafana dashboards.json generate dashboard
```

You can import these dashboard into grafana using the default user interface, see here [Import dashboards](https://grafana.com/docs/reference/export_import/).

## Installation

### From source

If you want to build `graylog-to-grafana` from source, you need Rust 1.31 or higher. You can then use [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) to build everything:

```
cargo install graylog-to-grafana
```

