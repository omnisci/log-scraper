mod log_parser;

use std::env;

extern crate regex;

#[macro_use]
extern crate clap;

extern crate pager;
use pager::Pager;

use colored;

fn main() -> std::io::Result<()> {
    let mut pager = Pager::new();
    pager.setup();

    let params = clap_app!(myapp =>
        (name: "omnisci-log-scraper")
        (version: "0.1.0")
        (author: "Alex Baden <alex.baden@mapd.com>, Mike Hinchey <mike.hinchey@omnisci.com>")
        (about: "Scrapes OmniSci DB logs for useful data")

        // TODO implement more filter tags: vega, exec, ops, connect, version, failure, error, warning
        (@arg FILTER: -f --filter +takes_value "Filter logs: all, sql")

        // TODO select
        // (@arg SELECT: -s --select +takes_value "Select column sets: all, min, exec, ...")

        // TODO arg input dir

        // TODO arg DB URL

        // TODO arg file index selector: "-1", -5..-1", "..-1"

        // TODO arg output format type: json, load_table, kafka
        (@arg TYPE: -t --type +takes_value "Output format: csv, tsv, terminal, sql")

        (@arg OUTPUT: -o --output +takes_value "Ouput file")

        (@arg INPUT: +multiple "Input log files")

        (@arg debug: -d ... "Debugging information")
    ).get_matches();

    let inputs = match params.indices_of("INPUT") {
        None => vec!("data/mapd_log/omnisci_server.INFO".to_string()),
        Some(indices) => {
            let args: Vec<String> = env::args().collect();
            let mut vec = Vec::new();
            for i in indices {
                vec.push(args[i].to_string());
            };
            vec
        },
    };

    let output = params.value_of("OUTPUT");
    let filter = match params.value_of("FILTER") {
        None => "all",
        Some(x) => x,
    };
    let filter: Vec<&str> = filter.split(",").map(|x| x.trim()).collect();

    let output_type = match params.value_of("TYPE") {
        // TODO if OUTPUT file is set, disable terminal, default to csv
        None => if pager.is_on() {
            // since we know we're printing to terminal, force the pager on, so colors work
            colored::control::set_override(true);
            log_parser::OutputType::Terminal
        } else {
            log_parser::OutputType::CSV
        },
        Some(x) => log_parser::OutputType::new(x),
    };

    for input in inputs {
        match log_parser::transform_logs(&input, output, &filter, &output_type) {
            Ok(_) => continue,
            Err(x) => return Err(x),
        };
    }
    Ok(())
}
