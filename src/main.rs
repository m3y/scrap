use anyhow::Result;
use clap::{crate_authors, crate_description, crate_name, crate_version, Clap};
use select::document::Document;
use select::predicate::Name;
use std::io;

#[derive(Clap, Debug)]
#[clap(
    name = crate_name!(),
    version = crate_version!(),
    author = crate_authors!(),
    about = crate_description!()
)]
struct Opts {
    url: Option<String>,
}

fn main() -> Result<()> {
    env_logger::init();

    let opts = Opts::parse();
    let url = match opts.url {
        Some(url) => url,
        None => {
            let mut u = String::new();
            io::stdin().read_line(&mut u).expect("error");
            u.retain(|c| c != '\n');
            println!("{}", u);
            u
        }
    };

    log::info!("Get \"{}\"", url);

    let body = reqwest::blocking::get(&url)?.text()?;

    match Document::from(body.as_str()).find(Name("title")).nth(0) {
        Some(t) => {
            println!("- {}\n  - [ ] {}\n", t.text(), url);
        }
        None => log::error!("not exists"),
    }

    Ok(())
}
