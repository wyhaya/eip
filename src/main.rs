use nix;
use nix::ifaddrs::InterfaceAddress;
use std::io::prelude::*;

const REQUEST_ADDTESS: &[(&str, &str)] = &[
    ("icanhazip.com:80", "/"),
    ("checkip.amazonaws.com:80", "/"),
    ("bot.whatismyipaddress.com:80", "/"),
    ("www.trackip.net:80", "/ip"),
    ("ifconfig.me:80", "/"),
    ("ipinfo.io:80", "/ip"),
];

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    if let Some(arg) = args.get(1) {
        if arg == "-e" {
            return external();
        }
    }

    return internal();
}

fn external() {
    let mut threads = vec![];

    for addr in REQUEST_ADDTESS {
        let thread = std::thread::spawn(move || {
            if let Ok(ip) = http(&addr) {
                print!("{}", ip);
                std::process::exit(0);
            }
        });
        threads.push(thread);
    }

    for spawn in threads {
        let _ = spawn.join();
    }

    eprintln!("error: Cannot get external ip address");
    std::process::exit(1);
}

fn internal() {
    let address = match nix::ifaddrs::getifaddrs() {
        Ok(address) => address,
        Err(err) => {
            eprintln!("error: {:?}", err);
            std::process::exit(1);
        }
    };

    let mut networks: Vec<Vec<InterfaceAddress>> = vec![];
    for ifaddr in address {
        let index = networks
            .iter()
            .position(|item| item[0].interface_name == ifaddr.interface_name);

        if let Some(i) = index {
            networks[i].push(ifaddr);
        } else {
            networks.push(vec![ifaddr]);
        }
    }

    for network in networks {
        let any = network
            .iter()
            .any(|item| if let None = item.address { false } else { true });
        if any {
            println!("{}:", network[0].interface_name);
            for item in network {
                for ip in item.address {
                    println!("    {}", ip);
                }
            }
        }
    }
}

fn http(addr: &(&str, &str)) -> std::io::Result<String> {
    let mut connect = std::net::TcpStream::connect(addr.0)?;

    let req = format!(
        "GET {} HTTP/1.1\r\nHost: {}\r\n\r\n",
        addr.1,
        addr.0.replace(":80", "")
    );
    connect.write_all(req.as_bytes())?;

    let mut res = std::io::BufReader::new(connect);

    Ok(body(res.fill_buf()?))
}

fn body(data: &[u8]) -> String {
    const SPLIT: &[u8] = &[13, 10, 13, 10];

    for (i, item) in data.windows(SPLIT.len()).enumerate() {
        if item == SPLIT {
            return String::from_utf8_lossy(&data[i + SPLIT.len()..]).to_string();
        }
    }
    String::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_http() {}

    #[test]
    fn test_body() {
        assert_eq!(body(b"\r\n\r\n0"), "0");
        assert_eq!(body(b"GET / HTTP/1.1\r\n\r\n0"), "0");
    }

}
