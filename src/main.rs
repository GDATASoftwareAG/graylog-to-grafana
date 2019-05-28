use reqwest::Client;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::path::PathBuf;
use structopt::StructOpt;

mod grafana;
mod graylog;

/// Allows to save Grafana dashboards into a directory
#[derive(StructOpt, Debug)]
pub struct GenerateArguments {
    /// Directory for output grafana dashboards
    #[structopt(name = "output", parse(from_os_str))]
    output: PathBuf,
}
/// Allows to add automatically dashboards to Grafana
#[derive(StructOpt, Debug)]
pub struct AddArguments {
    #[structopt(long = "url")]
    url: String,

    #[structopt(long = "token", default_value = "graylog")]
    token: String,

    #[structopt(long = "folder", default_value = "0")]
    folder: i64,
}
#[derive(Debug, StructOpt)]
pub enum Command {
    /// Allows to save Grafana dashboards into a directory
    #[structopt(name = "generate")]
    Generate(GenerateArguments),

    /// Allows to add automatically dashboards to Grafana
    #[structopt(name = "add")]
    Add(AddArguments),
}

#[derive(Debug, StructOpt)]
#[structopt(name = "graylog-to-grafana")]
pub struct ApplicationArguments {
    /// Graylog content pack to process
    #[structopt(name = "input", parse(from_os_str))]
    input: PathBuf,

    #[structopt(long = "datasource", default_value = "graylog")]
    datasource: String,

    #[structopt(subcommand)]
    command: Command,
}

fn main() {
    env_logger::init();

    let opt: &ApplicationArguments = &ApplicationArguments::from_args();
    let u = read_content_pack_from_file(&opt.input).unwrap();
    let dashboards: Vec<_> = u
        .dashboards
        .into_iter()
        .map(|t| grafana::Dashboard::create_dashboard_from_graylog(t, opt))
        .collect();

    match &opt.command {
        Command::Generate(generate) => {
            dashboards
                .iter()
                .for_each(|s| write_grafana_dashboard(s, &generate).unwrap());
        }
        Command::Add(add) => {
            dashboards
                .into_iter()
                .map(|s| grafana::ApiDashboard {
                    dashboard: s,
                    folder_id: add.folder,
                    overwrite: true,
                })
                .for_each(|dashboard| {
                    let url = format!("{}{}", add.url, "/api/dashboards/db/");
                    let client = Client::new();
                    client
                        .post(&url)
                        .header("Authorization", format!("{} {}", "Bearer", add.token))
                        .json(&dashboard)
                        .send()
                        .unwrap();
                }); //83
        }
    }
}

fn write_grafana_dashboard(
    s: &grafana::Dashboard,
    opt: &GenerateArguments,
) -> Result<(), Box<Error>> {
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
