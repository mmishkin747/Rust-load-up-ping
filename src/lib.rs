use clap::{error, Parser};
use futures::executor::block_on;
use futures::future::join_all;
use std::collections::HashMap;
use std::error::Error;
use std::io::{BufRead, BufReader, LineWriter, Read, Write};
use std::net::{IpAddr, Shutdown, SocketAddr, TcpStream};
use std::ops::RangeInclusive;
use std::str::from_utf8;
use std::time::Duration;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Parser)]
#[command(name = "Rlup")]
#[command(author = "Mikhail V. <mmishkin747@gmail.com>")]
#[command(version = "1.0")]
#[command(about = "load channel by ping")]
struct Cli {
    /// User's name for connecting ups
    #[arg(short, long)]
    user: Option<String>,
    /// Password for connecting ups
    #[arg(short, long)]
    password: Option<String>,
    /// Network ipv4 address server
    address_server: IpAddr,
    /// Network ipv4 address host
    address_host: IpAddr,
    /// Network port to use
    #[arg( long, value_parser = port_in_range, default_value_t = 22)]
    port_server: u16,
    /// Count session
    #[arg(short, long, value_parser = count_sessions_in_range, default_value_t = 15)]
    count_session: u64,
    /// Timeout for write/right, sec
    #[arg(short, long, default_value_t = 10)]
    time_out: u64,
}
#[derive(Debug)]
pub struct Config {
    addr_server: SocketAddr,
    addr_host: IpAddr,
    user: String,
    passw: String,
    count_session: u64,
    timeout: Duration,
}

pub struct Connecter {
    writer: LineWriter<TcpStream>,
    reader: BufReader<TcpStream>,
}
impl Connecter {
    pub fn new(config: &Config) -> MyResult<Self> {
        let stream = TcpStream::connect_timeout(&config.addr_server, config.timeout)?;
        let writer = LineWriter::new(stream.try_clone()?);
        let reader = BufReader::new(stream);
        Ok(Self { reader, writer })
    }

    pub fn send_mes(&mut self, message: &str) -> MyResult<()> {
        self.writer.write_all(&message.as_bytes())?;
        self.writer.flush();
        Ok(())
    }
}

pub fn get_args() -> MyResult<Config> {
    let cli = Cli::parse();

    let addr_server = SocketAddr::new(cli.address_server, cli.port_server);

    let timeout: Duration = Duration::from_secs(cli.time_out); // this param may be chage if it need
    let mut user = String::new();
    let mut passw = String::new();
    if let Some(ref user_v) = cli.user {
        if let Some(ref passw_v) = cli.password {
            user = user_v.to_string();
            passw = passw_v.to_string();
        }
    }

    Ok(Config {
        addr_server,
        addr_host: cli.address_host,
        user,
        passw,
        timeout,
        count_session: cli.count_session,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    dbg!(&config);
    block_on(async_run(config));

    Ok(())
}

async fn async_run(config: Config) -> MyResult<()> {
    let mut vec = Vec::new();
    for i in 0..config.count_session {
        vec.push(con_auth(&config))
    }
    join_all(vec).await;

    Ok(())
}

async fn con_auth(config: &Config) -> MyResult<()> {
    println!("I am fn con and auth");
    let mut connecter = Connecter::new(config)?;
    let res = connecter.send_mes("Hello")?;
    Ok(())
}
/*
fn create_vec(f: fn(), config: &Config) -> Vec<fn()> {
    let mut vec: Vec<fn()> = Vec::new();
    for i in 0..config.count_session {
        vec.push(f());
    }
    vec
}
*/
/// This func check valid number port
fn count_sessions_in_range(s: &str) -> Result<u64, String> {
    let port_range: RangeInclusive<usize> = 1..=20;
    let port: usize = s
        .parse()
        .map_err(|_| format!("`{}` isn't a count sessions number", s))?;
    if port_range.contains(&port) {
        Ok(port as u64)
    } else {
        Err(format!(
            "count sessions not in range {}-{}",
            port_range.start(),
            port_range.end()
        ))
    }
}
fn port_in_range(s: &str) -> Result<u16, String> {
    let port_range: RangeInclusive<usize> = 1..=65535;
    let port: usize = s
        .parse()
        .map_err(|_| format!("`{}` isn't a port number", s))?;
    if port_range.contains(&port) {
        Ok(port as u16)
    } else {
        Err(format!(
            "Port not in range {}-{}",
            port_range.start(),
            port_range.end()
        ))
    }
}
