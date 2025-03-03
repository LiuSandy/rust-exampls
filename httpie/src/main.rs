use std::{collections::HashMap, str::FromStr};

use anyhow::{anyhow, Ok, Result};
use clap::{Args, Parser, Subcommand};
use colored::*;
use mime::Mime;
use reqwest::{header, Client, Response, Url};

#[derive(Parser, Debug)]
#[command(version = "1.0", author = "Lies<nsiliushuang@gmail.com>")]
struct Opts {
    #[command(subcommand)]
    subcmd: SubCommands,
}

#[derive(Subcommand, Debug)]
enum SubCommands {
    Get(Get),
    Post(Post),
}

#[derive(Args, Debug)]
struct Get {
    #[arg(value_parser = parse_url)]
    url: String,
}

#[derive(Args, Debug)]
struct Post {
    #[arg(value_parser = parse_url)]
    url: String,

    #[arg(value_parser = parse_kv_pair)]
    body: Vec<KvPair>,
}

#[derive(Debug, Clone)]
struct KvPair {
    k: String,
    v: String,
}

impl FromStr for KvPair {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split("=");
        let err = || anyhow!(format!("Failed to parse {}", s));
        Ok(Self {
            k: (split.next().ok_or_else(err)?).to_string(),
            v: (split.next().ok_or_else(err)?).to_string(),
        })
    }
}

fn parse_kv_pair(s: &str) -> Result<KvPair> {
    Ok(s.parse()?)
}

fn parse_url(s: &str) -> Result<String> {
    let _url: Url = s.parse()?;
    Ok(s.into())
}

fn print_status(resp: &Response) {
    let status = format!("{:?} {}", resp.version(), resp.status()).blue();
    println!("{}\n", status);
}

fn print_headers(resp: &Response) {
    for (name, value) in resp.headers() {
        println!("{}: {:?}", name.to_string().green(), value);
    }
    println!("\n")
}

fn get_content_type(resp: &Response) -> Option<Mime> {
    resp.headers()
        .get(header::CONTENT_TYPE)
        .map(|v| v.to_str().unwrap().parse().unwrap())
}

fn print_body(m: Option<Mime>, body: &String) {
    match m {
        Some(v) if v == mime::APPLICATION_JSON => {
            println!("{}", jsonxf::pretty_print(body).unwrap().cyan())
        }
        _ => println!("{}", body),
    }
}

async fn print_resp(resp: Response) -> Result<()> {
    print_status(&resp);
    print_headers(&resp);
    let mime = get_content_type(&resp);
    let body = resp.text().await?;
    print_body(mime, &body);
    Ok(())
}

async fn get(client: Client, args: &Get) -> Result<()> {
    let resp = client.get(&args.url).send().await?;
    print_resp(resp).await?;
    Ok(())
}

/// 处理 post 子命令
async fn post(client: Client, args: &Post) -> Result<()> {
    let mut body = HashMap::new();
    for pair in args.body.iter() {
        body.insert(&pair.k, &pair.v);
    }
    let resp = client.post(&args.url).json(&body).send().await?;
    Ok(print_resp(resp).await?)
}

#[tokio::main]
async fn main() -> Result<()> {
    let opts: Opts = Opts::parse();

    let mut headers = header::HeaderMap::new();
    headers.insert("X-POWERED0-BY", "Rust".parse()?);
    headers.insert(header::USER_AGENT, "Rust Httpie".parse()?);

    let client = Client::builder().default_headers(headers).build()?;

    let result = match opts.subcmd {
        SubCommands::Get(ref args) => get(client, args).await?,
        SubCommands::Post(ref args) => post(client, args).await?,
    };

    Ok(result)

    // https://www.thumbor.org/300x200/smart/thumbor.readthedocs.io/en/latest/_images/logo-thumbor.png
}
