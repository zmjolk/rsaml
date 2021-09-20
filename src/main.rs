// use rpassword::read_password;
use std::process;
use std::env;

use rsaml;
// use argparse::{ArgumentParser, StoreTrue, Store};

fn main() {

    let config_path = format!("{}/.aws/rsaml.ini", env::var("HOME").unwrap());

    let cfg = match rsaml::parse_config(&config_path) {
        Ok(res) => res,
        Err(e) => {
            println!("No user string entered or prewritten cfg dir found! {}", e);
            process::exit(1);
        },
    };

    rsaml::write_cfg(&cfg, &config_path);

    let mut saml_reqwest = rsaml::SamlReqwest::new(cfg);
    saml_reqwest.get_saml_assertion();

}
