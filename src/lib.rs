use pest_derive::*;
use reqwest::blocking::Client;
use serde::Deserialize;
use std::collections::HashMap;

use pest::Parser;

#[derive(Parser)]
#[grammar = "rc.pest"]
pub struct INIParser;

pub struct MessageBuilder<'a> {
    config: ZulipAPISettings,
    form: HashMap<&'a str, &'a str>,
}

impl<'a> MessageBuilder<'a> {
    pub fn to(&mut self, to: &'a str) {
        self.form.insert("to", to);
    }
    pub fn r#type(&mut self, t: &'a str) {
        self.form.insert("type", t);
    }
    pub fn topic(&mut self, subject: &'a str) {
        self.form.insert("subject", subject);
    }
    pub fn content(&mut self, content: &'a str) {
        self.form.insert("content", content);
    }
    pub fn send(&self) {
        let client = Client::new();
        let result = client
            .post(&format!("{}/api/v1/messages", &self.config.site))
            .basic_auth(&self.config.email, Some(&self.config.key))
            .header("application", "x-www-form-urlencoded")
            .form(&self.form)
            .send()
            .unwrap();
        println!("{:?}", result);
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct ZulipRuntimeConfig {
    api: ZulipAPISettings,
}

#[derive(Deserialize, Debug, Clone)]
struct ZulipAPISettings {
    email: String,
    key: String,
    site: String,
}

pub struct Zulip {
    rc: ZulipRuntimeConfig,
}

impl Zulip {
    pub fn new(rc_str: &str) -> Self {
        let rc = from_str(rc_str);
        Zulip { rc }
    }
    pub fn message(&self) -> MessageBuilder {
        MessageBuilder {
            config: self.rc.api.clone(),
            form: HashMap::new(),
        }
    }
}

pub fn from_str(rc: &str) -> ZulipRuntimeConfig {
    let pairs = INIParser::parse(Rule::file, rc).unwrap();
    let mut email = "";
    let mut key = "";
    let mut site = "";
    for pair in pairs {
        // A pair is a combination of the rule which matched and a span of input
        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::char => println!("char:  {}", inner_pair.as_str()),
                Rule::section => println!("section:   {}", inner_pair.as_str()),
                Rule::property => {
                    println!("property:   {}", inner_pair.as_str());
                    let mut rule = inner_pair.into_inner();
                    let name: &str = rule.next().unwrap().as_str();
                    if name == "email" {
                        email = rule.next().unwrap().as_str();
                    }
                    if name == "key" {
                        key = rule.next().unwrap().as_str();
                    }
                    if name == "site" {
                        site = rule.next().unwrap().as_str();
                    }
                }
                Rule::file => println!("file:   {}", inner_pair.as_str()),
                Rule::EOI => println!("EOI: {}", inner_pair.as_str()),
                _ => println!("{:?}", inner_pair),
            };
        }
    }
    ZulipRuntimeConfig {
        api: ZulipAPISettings {
            email: email.to_string(),
            key: key.to_string(),
            site: site.to_string(),
        },
    }
}
