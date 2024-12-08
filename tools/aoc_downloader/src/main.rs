#![deny(clippy::all, clippy::pedantic, clippy::panic, clippy::unwrap_used)]
#![allow(clippy::missing_errors_doc, clippy::module_name_repetitions)]

use std::{
    env,
    fs::File,
    io::{BufWriter, Write},
    path::{Path, PathBuf},
};

use anyhow::{anyhow, Context, Result};
use clap::Parser;
use cli::util::{example_dir_for_year_and_day, file_path};
use reqwest::{
    blocking::Client,
    header::{HeaderMap, HeaderValue, COOKIE},
};

use cli::{
    part::Part,
    util::{current_year, default_data_dir},
};

const BASE_AOC_URL: &str = "https://adventofcode.com";

/// AOC challenge data downloader
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Data directory
    #[arg(short, long, default_value=default_data_dir().into_os_string())]
    data_dir: PathBuf,
    /// AOC challenge year
    #[arg(short, long, default_value = current_year(), value_parser=clap::value_parser!(i32).range(2015..))]
    year: i32,
    /// AOC session cookie
    #[arg(short, long)]
    aoc_session: Option<String>,
    /// Problem day
    #[arg(value_parser=clap::value_parser!(u16).range(1..=25))]
    day: u16,
    /// Problem part
    #[arg()]
    problem_part: Part,
}

fn session_cookie(args: &Args) -> Option<String> {
    match (&args.aoc_session, env::var("AOC_SESSION_COOKIE").ok()) {
        (Some(args_session), _) => Some(String::from(args_session)),
        (None, Some(env_session)) => Some(env_session),
        (None, None) => None,
    }
}

fn download(cookie: &str, output_dir: &Path, year: i32, day: u16, part: Part) -> Result<()> {
    fn perform_request(cookie: &str, year: i32, day: u16) -> Result<String> {
        let aoc_url = format!("{BASE_AOC_URL}/{year}/day/{day}/input");
        let mut headers = HeaderMap::new();
        headers.insert(
            COOKIE,
            HeaderValue::from_str(&format!("session={cookie}")).expect("valid header"),
        );
        let client = Client::builder()
            .default_headers(headers)
            .build()
            .expect("bad reqwest builder");
        match client.get(aoc_url).send() {
            Ok(res) => Ok(res.text().context("bad content in aoc example")?),
            Err(e) => Err(e).context("failed to download the requested example"),
        }
    }

    fn write_response(example_dir: &Path, part: Part, data: &str) -> Result<()> {
        let f = File::create(file_path(example_dir, part, false))
            .context("failed to create output file")?;
        let mut f = BufWriter::new(f);
        f.write_all(data.as_bytes())
            .context("failed to write example file")
    }

    let example_dir = example_dir_for_year_and_day(output_dir, year, day);

    std::fs::create_dir_all(&example_dir).context("failed to create output directory")?;
    let data = perform_request(cookie, year, day)?;
    write_response(&example_dir, part, &data)
}

fn main() -> Result<()> {
    dotenv::dotenv().context("failed to load dotenv environment")?;
    let args = Args::parse();
    let cookie = session_cookie(&args).ok_or(anyhow!(
        "you must specify aoc_session as a cli
         arg or via the AOC_SESSION_COOKIE environment variable"
    ))?;
    download(
        &cookie,
        &args.data_dir,
        args.year,
        args.day,
        args.problem_part,
    )
}
