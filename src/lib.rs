use async_std::task::sleep;
use clap::Parser;
use futures::executor::block_on;
use futures::future::join_all;
use std::error::Error;
use std::io::{BufReader, LineWriter, Read, Write};
use std::net::{IpAddr, SocketAddr, TcpStream};
use std::ops::RangeInclusive;
use std::time::Duration;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Parser)]
#[command(name = "Rlup")]
#[command(author = "Mikhail V. <mmishkin747@gmail.com>")]
#[command(version = "1.0")]
#[command(about = "Rust load Up by Ping")]
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
    #[arg( long, value_parser = port_in_range, default_value_t = 23)]
    port: u16,
    /// Count session
    #[arg(short, long, value_parser = count_sessions_in_range, default_value_t = 1)]
    count_session: u64,
    /// Timeout for write/right, sec
    #[arg(short, long, default_value_t = 10)]
    time_out: u64,
    /// Size MTU
    #[arg(short, long, default_value_t = 1500)]
    mtu: u16,
    /// Count repit ping
    #[arg(short, long, default_value_t = 1000)]
    repit: u16,
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

#[derive(Debug)]
pub struct Connecter {
    writer: LineWriter<TcpStream>,
    reader: BufReader<TcpStream>,
}
impl Connecter {
    pub fn new(config: &Config) -> MyResult<Self> {
        let stream = TcpStream::connect_timeout(&config.addr_server, config.timeout)
            .expect("Error conect to server");
        stream.set_read_timeout(Some(config.timeout)).unwrap();
        stream.set_write_timeout(Some(config.timeout)).unwrap();
        let writer = LineWriter::new(stream.try_clone()?);
        let reader = BufReader::new(stream);
        Ok(Self { reader, writer })
    }

    pub fn send_mes(&mut self, message: &str) -> MyResult<()> {
        self.writer
            .write_all(&message.as_bytes())
            .expect("didn't send messg");
        self.writer.write(&['\n' as u8])?;
        Ok(())
    }
    pub async fn read_mes(&mut self) -> MyResult<String> {
        let mut buf = [0 as u8; 1024];
        sleep(Duration::from_secs(1)).await;
        self.reader.read(&mut buf).unwrap();
        let res = String::from_utf8_lossy(&buf);
        Ok(res.to_string())
    }
}

pub fn get_args() -> MyResult<Config> {
    let cli = Cli::parse();

    let addr_server = SocketAddr::new(cli.address_server, cli.port);

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
    block_on(async_run(config))?;
    Ok(())
}

async fn async_run(config: Config) -> MyResult<()> {
    let mut vec = Vec::new();
    for _ in 0..config.count_session {
        vec.push(con_auth(&config))
    }

    let auth_res = join_all(vec).await;
    dbg!(auth_res);
    Ok(())
}

async fn con_auth(config: &Config) -> MyResult<Connecter> {
    let mut connecter = Connecter::new(config)?;
    let data = connecter.read_mes().await;
    dbg!(&data);
    connecter.send_mes(config.user.as_str()).unwrap();
    connecter.send_mes(config.passw.as_str())?;
    let data = connecter.read_mes().await;
    dbg!(&data);
    Ok(connecter)
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
