use thiserror::Error;

pub mod consts {
    pub const N: &str = "-n";
    pub const NETWORK: &str = "--network";
    pub const H: &str = "-h";
    pub const HOST: &str = "--host";
    pub const P: &str = "-p";
    pub const PORT: &str = "--port";

    pub const PEAQ: &str = "peaq";
    pub const AGUNG: &str = "agung";
    pub const KREST: &str = "krest";
    pub const DEV: &str = "dev";
}

#[derive(Error, Debug, PartialEq)]
pub enum ArgError {
    #[error("No arguments provided")]
    NoArguments,
    #[error("Network is not specified")]
    NoNetworkConfiguration,
    #[error("Network is not recognised")]
    UnrecognisedNetwork,
    #[error("Port is not valid")]
    FailedToParsePort,
}

pub struct Network;
pub struct Host;
pub struct Port;

impl Network {
    pub fn config(option: String) -> Result<Config, ArgError> {
        match option.as_str() {
            consts::PEAQ => Ok(Peaq::config()),
            consts::AGUNG => Ok(Agung::config()),
            consts::KREST => Ok(Krest::config()),
            consts::DEV => Ok(Dev::config()),
            _ => return Err(ArgError::UnrecognisedNetwork),
        }
    }
}

trait IsArgument<'a>: Sized {
    fn predicates(&self) -> Vec<&'a str>;
}

impl<'a> IsArgument<'a> for Network {
    fn predicates(&self) -> Vec<&'a str> {
        vec![consts::N, consts::NETWORK]
    }
}

impl<'a> IsArgument<'a> for Host {
    fn predicates(&self) -> Vec<&'a str> {
        vec![consts::H, consts::HOST]
    }
}
impl<'a> IsArgument<'a> for Port {
    fn predicates(&self) -> Vec<&'a str> {
        vec![consts::P, consts::PORT]
    }
}

#[derive(Debug)]
pub struct Args(Vec<String>);

impl Args {
    pub fn new(raw_args: Vec<String>) -> Self {
        Args(raw_args)
    }

    pub fn get(&self) -> &Vec<String> {
        &self.0
    }

    pub fn get_value<'a>(self, predicates: Vec<&'a str>) -> (Option<String>, Self) {
        let mut iter = self.get().iter().peekable();
        iter.find(|p| predicates.iter().any(|pred| p == pred));
        (iter.next().cloned(), self)
    }
}

trait Parsable<'a> {
    type Output;
    fn parse(&self, args: Args) -> Self::Output;
}

impl<'a> Parsable<'a> for Network {
    type Output = Result<(Option<String>, Args), ArgError>;

    fn parse(&self, args: Args) -> Self::Output {
        let (value, args) = args.get_value(self.predicates());
        match value {
            Some(value) => Ok((Some(value), args)),
            None => return Err(ArgError::NoNetworkConfiguration),
        }
    }
}

impl<'a> Parsable<'a> for Host {
    type Output = (Option<String>, Args);
    fn parse(&self, args: Args) -> Self::Output {
        args.get_value(self.predicates())
    }
}

impl<'a> Parsable<'a> for Port {
    type Output = Result<(Option<u16>, Args), ArgError>;

    fn parse(&self, args: Args) -> Self::Output {
        let (value, args) = args.get_value(self.predicates());
        match value {
            Some(value) => match value.parse() {
                Ok(port) => Ok((Some(port), args)),
                Err(_) => return Err(ArgError::FailedToParsePort),
            },
            None => Ok((None, args)),
        }
    }
}

struct Peaq;
struct Agung;
struct Krest;
struct Dev;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Config {
    pub network: String,
    pub host: String,
    pub port: u16,
    pub chain_id: u16,
}

impl<'a> Config {
    pub fn new(args: Args) -> Result<Config, ArgError> {
        if args.get().len() == 0 {
            return Err(ArgError::NoArguments);
        }

        let (network, args) = Network.parse(args)?;
        let network = match network {
            Some(value) => value,
            None => return Err(ArgError::UnrecognisedNetwork),
        };

        let mut config = Network::config(network)?;

        let (host, args) = Host.parse(args);
        if let Some(host) = host {
            config.host = host;
        }

        let (port, _args) = Port.parse(args)?;
        if let Some(port) = port {
            config.port = port;
        }

        Ok(config)
    }
}

trait NetworkConfig: Sized {
    fn config() -> Config;
}

impl NetworkConfig for Peaq {
    fn config() -> Config {
        Config {
            network: "Peaq".to_string(),
            host: "https://erpc-mpfn1.peaq.network".to_string(),
            port: 9933,
            chain_id: 3338,
        }
    }
}

impl NetworkConfig for Krest {
    fn config() -> Config {
        Config {
            network: "Krest".to_string(),
            host: "https://erpc-krest.peaq.network".to_string(),
            port: 9944,
            chain_id: 2241,
        }
    }
}

impl NetworkConfig for Agung {
    fn config() -> Config {
        Config {
            network: "Agung".to_string(),
            host: "https://rpcpc1-qa.agung.peaq.network".to_string(),
            port: 9944,
            chain_id: 9990,
        }
    }
}

impl NetworkConfig for Dev {
    fn config() -> Config {
        Config {
            network: "dev".to_string(),
            host: "http://127.0.0.1".to_string(),
            port: 9944,
            chain_id: 9990,
        }
    }
}
