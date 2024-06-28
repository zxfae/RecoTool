use std::env;

pub struct Args {
    pub full_name: Option<String>,
    pub ip: Option<String>,
    pub username: Option<String>,
}

pub fn parse_args() -> Args {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return Args {
            full_name: None,
            ip: None,
            username: None,
        };
    }

    match args[1].as_str() {
        "-f" | "--fn" => Args {
            full_name: args.get(2).cloned(),
            ip: None,
            username: None,
        },
        "-i" | "--ip" => Args {
            full_name: None,
            ip: args.get(2).cloned(),
            username: None,
        },
        "-u" | "--username" => Args {
            full_name: None,
            ip: None,
            username: args.get(2).cloned(),
        },
        _ => Args {
            full_name: None,
            ip: None,
            username: None,
        },
    }
}
