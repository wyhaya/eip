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
    let mut spawns = vec![];

    for addr in REQUEST_ADDTESS {
        let spawn = std::thread::spawn(move || {
            if let Ok(ip) = request(&addr) {
                print!("{}", ip);
                std::process::exit(0);
            }
        });
        spawns.push(spawn)
    }

    for spawn in spawns {
        let _ = spawn.join();
    }
    eprintln!("\x1b[91merror: \x1b[0m{}", "Cannot get external ip address");
}

fn request(addr: &(&str, &str)) -> std::io::Result<String> {
    let mut connect = std::net::TcpStream::connect(addr.0)?;

    let req = format!(
        "GET {} HTTP/1.1\r\nHost: {}\r\n\r\n",
        addr.1,
        addr.0.replace(":80", "")
    );
    connect.write_all(req.as_bytes())?;

    let mut res = std::io::BufReader::new(connect);

    Ok(parse_body(res.fill_buf()?))
}

fn parse_body(data: &[u8]) -> String {
    for i in 0..data.len() {
        if data.len() - 4 >= i {
            if &data[i..i + 4] == &[13, 10, 13, 10] {
                return String::from_utf8_lossy(&data[i + 4..]).to_string();
            }
        }
    }
    String::new()
}


#[cfg(test)]
mod tests {
    use crate::parse_body;

    #[test]
    fn test_request() {

    }

    #[test]
    fn test_parse_body() {
        assert_eq!(parse_body(b"\r\n\r\n0"), "0");
        assert_eq!(parse_body(b"GET / HTTP/1.1\r\n\r\n0"), "0");
    }

}
