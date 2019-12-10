#[macro_use]
extern crate envconfig_derive;
extern crate envconfig;

use envconfig::{Envconfig, Error};
use std::env;

#[derive(Envconfig)]
pub struct Config {
    #[envconfig(from = "DB_HOST")]
    pub db_host: String,

    #[envconfig(from = "DB_PORT")]
    pub db_port: u16,
}

fn setup() {
    env::remove_var("DB_HOST");
    env::remove_var("DB_PORT");
}

#[test]
fn test_inits_config_from_env_variables() {
    setup();

    env::set_var("DB_HOST", "localhost");
    env::set_var("DB_PORT", "5432");

    let config = Config::init().unwrap();
    assert_eq!(config.db_host, "localhost");
    assert_eq!(config.db_port, 5432u16);
}

#[test]
fn test_checks_presence_of_env_vars() {
    setup();

    env::set_var("DB_HOST", "localhost");

    let err = Config::init().err().unwrap();
    let expected_err = Error::EnvVarMissing { name: "DB_PORT" };
    assert_eq!(err, expected_err);
}

#[test]
fn test_fails_if_can_not_parse_db_port() {
    setup();

    env::set_var("DB_HOST", "localhost");
    env::set_var("DB_PORT", "67000");

    let err = Config::init().err().unwrap();
    let expected_err = Error::ParseError { name: "DB_PORT" };
    assert_eq!(err, expected_err);
}

#[test]
fn test_custom_from_str() {
    use std::num::ParseIntError;
    use std::str::FromStr;

    setup();

    #[derive(Debug, PartialEq)]
    struct Point {
        x: i32,
        y: i32,
    }

    impl FromStr for Point {
        type Err = ParseIntError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let coords: Vec<&str> = s
                .trim_matches(|p| p == '(' || p == ')')
                .split(',')
                .collect();

            let x_fromstr = coords[0].parse::<i32>()?;
            let y_fromstr = coords[1].parse::<i32>()?;

            Ok(Point {
                x: x_fromstr,
                y: y_fromstr,
            })
        }
    }

    #[derive(Envconfig)]
    pub struct Config {
        #[envconfig(from = "DB_HOST")]
        point: Point,
    }

    env::set_var("DB_HOST", "(1,2)");

    let err = Config::init().unwrap();
    assert_eq!(err.point, Point { x: 1, y: 2 });
}
