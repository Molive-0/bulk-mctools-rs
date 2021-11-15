use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg};
use tokio::task::JoinHandle;

use steven_protocol::protocol::Conn;

#[tokio::main]
async fn main() -> () {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(
            Arg::new("threads")
                .short('t')
                .long("threads")
                .about("Number of threads")
                .default_value("10"),
        )
        .arg(
            Arg::new("timeout")
                .short('T')
                .long("timeout")
                .about("Server timeout")
                .default_value("5"),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .about("output file")
                .default_value("-"),
        )
        .arg(
            Arg::new("input")
                .short('i')
                .long("input")
                .about("input file")
                .default_value("-"),
        )
        .get_matches();

    let input = std::fs::read_to_string(matches.value_of_t_or_exit::<String>("input"))
        .expect("Could not load input file");

    let queue = input
        .split("\n")
        .map(str::trim)
        .map(str::to_string)
        .map(|i| {
            tokio::spawn(async move {
                let conn = Conn::new(&i, 756).unwrap();
                conn.do_status().unwrap().0;
            })
        })
        .collect::<Vec<JoinHandle<_>>>();

    'outer: for q in queue {
        let t = match q.await {
            Ok(t) => {
                println!("{:?}", t);
            }
            Err(e) => {
                println!("An error occured: {}", e);
                continue 'outer;
            }
        };
    }
}
