use std::error::Error;
use std::collections::HashMap;
use std::io;

use ini::Ini;
use rpassword;
// use scraper::{Html, Selector};
use reqwest::blocking;

#[derive(Debug)]
pub struct Config {
    pub username: String,
    pub saml_url: String,
}

pub fn parse_config(config_path: &String) -> Result<Config, Box<dyn Error>> {

    let conf = match Ini::load_from_file(config_path) {
        Ok(ini) => {
            println!("Found existing config file at {}!", config_path);
            ini
        },
        Err(_) => {
            println!("Failed to find existing config file, building..");
            Ini::new()
        },
    };

    let username = if let Some(username) = conf.get_from(Some("rsaml"), "username") {
        username.to_string()
    } else {
        ask_for_var("username")
    };

    let saml_url = match conf.get_from(Some("rsaml"), "saml_url") {
        Some(saml_url) => saml_url.to_string(),
        None => ask_for_var("saml_url"),
    };

    let cfg = Config {
        username: username,
        saml_url: saml_url,
    };

    write_cfg(&cfg, config_path);

    Ok(cfg)
}

pub fn ask_for_var(var: &str) -> String {
    println!("No config value found for {}, please enter:", var);
    let mut response = String::new();
    io::stdin().read_line(&mut response).expect("Needs to be a string");
    response.pop();
    response
}

pub fn write_cfg(cfg: &Config, config_path: &String) {

    let mut conf = Ini::new();

    conf.with_section(Some("rsaml")).
        set("username", &cfg.username).
        set("saml_url", &cfg.saml_url);

    conf.write_to_file(config_path).unwrap();
}

#[derive(Debug)]
pub struct SamlReqwest {
    client: blocking::Client,
    resp_body: Option<String>,
    cfg: Config,
}

impl SamlReqwest {

    pub fn new(cfg: Config) -> SamlReqwest {
        let client = blocking::Client::new();
        SamlReqwest {
            client: client,
            resp_body: None,
            cfg: cfg,
        }
    }

    pub fn get_saml_assertion(&mut self)  {
        
        let password = rpassword::read_password_from_tty(Some("Enter password")).unwrap();

        let mut resp_url = String::new();
        match self.client.get(&self.cfg.saml_url).basic_auth(&self.cfg.username, Some(&password)).send() {
            Ok(resp) => {
                resp_url = resp.url().as_str().to_string();
                self.resp_body = Some(resp.text().unwrap());
                ()
            },
            Err(_) => panic!("aaaaa"),
        };

        println!("resp url is {}", &resp_url);

        // dbg!(&self.resp_body);

        // let doc = Html::parse_document(&self.resp_body.as_ref().unwrap());
        // let input_selector = Selector::parse("#loginForm input").unwrap();

        let mut post_params = HashMap::new();

        post_params.insert("userNameInput", r#"legal\scottj7"#);
        post_params.insert("passwordInput", &password);
        // post_params.insert("kmsiInput", "true");
        // post_params.insert("optionForms", "FormsAuthentication");

        // dbg!(&post_params);
        // form(&post_params)
        let resp = match self.client.post(resp_url).form(&post_params).send() {
            Ok(resp) => resp,
            Err(_) => panic!("a"),
        };
        // println!("{:#?}", resp);

        dbg!(&resp.status());
        dbg!(&resp.text());
    }
}

// fn parse_response() {

// }