use structopt::StructOpt;

use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::path::PathBuf;

mod grafana;
mod graylog;

/// A basic example
#[derive(StructOpt, Debug)]
#[structopt(name = "graylog-to-grafana")]
pub struct Opt {
    // A flag, true if used in the command line. Note doc comment will
    // be used for the help message of the flag.
    /// Activate debug mode
    #[structopt(short = "d", long = "debug")]
    debug: bool,

    /// Graylog content pack to process
    #[structopt(name = "input", parse(from_os_str))]
    input: PathBuf,

    /// Directory for output grafana dashboards
    #[structopt(name = "output", parse(from_os_str))]
    output: PathBuf,

    #[structopt(long = "datasource", default_value = "graylog")]
    datasource: String,
}

fn main() {
    let opt = &Opt::from_args();
    let u = read_content_pack_from_file(&opt.input).unwrap();
    u.dashboards
        .into_iter()
        .map(|t| grafana::Dashboard::create_dashboard_from_graylog(t, opt))
        .for_each(|s| write_grafana_dashboard(s, opt).unwrap());
}

fn write_grafana_dashboard(s: grafana::Dashboard, opt: &Opt) -> Result<(), Box<Error>> {
    let filename = format!(
        "{}.json",
        s.title.replace(" ", "_").replace("/", "_").to_lowercase()
    );
    let mut path = opt.output.clone();
    path.push(filename);
    let file = File::create(path)?;
    serde_json::to_writer_pretty(file, &s)?;
    Ok(())
}

fn read_content_pack_from_file<P: AsRef<Path>>(
    path: &P,
) -> Result<graylog::ContentPack, Box<Error>> {
    // Open the file in read-only mode with buffer.
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `User`.
    let u = serde_json::from_reader(reader)?;

    // Return the `User`.
    Ok(u)
}
