#![allow(clippy::use_self)] // is experimental in stable rust

use super::Data;
use super::UpdateConfig;
use super::FEATURE_NAME;
use crate::error::*;
use crate::feature;
use crate::wrapper::process;
use log::info;
use std::fmt;

enum IpAddress {
    V4,
    V6,
}

impl fmt::Display for IpAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "IPv{}",
            match self {
                IpAddress::V4 => 4,
                IpAddress::V6 => 6,
            }
        )
    }
}

pub(super) struct Updater {
    data: Data,
    config: UpdateConfig,
}

impl Updater {
    pub(super) const fn new(data: Data, config: UpdateConfig) -> Self {
        Self { data, config }
    }

    fn get_if_enabled<F: Fn() -> Result<Option<String>>>(
        &self,
        enabled: bool,
        builder: F,
    ) -> Result<Option<String>> {
        if enabled { builder() } else { Ok(None) }
    }
}

impl feature::Updatable for Updater {
    fn renderable(&self) -> &dyn feature::Renderable {
        &self.data
    }

    fn update(&mut self) -> Result<()> {
        let ipv4 = self.get_if_enabled(self.config.show_ipv4, || ip_address(&IpAddress::V4))?;
        let ipv6 = self.get_if_enabled(self.config.show_ipv6, || ip_address(&IpAddress::V6))?;
        let essid = self.get_if_enabled(self.config.show_essid, essid)?;

        self.data.update(ipv4, ipv6, essid);

        Ok(())
    }
}

fn essid() -> Result<Option<String>> {
    let command = process::Command::new("iwgetid", &["-r"]);
    let output = command
        .output()
        .wrap_error(FEATURE_NAME, "essid {} could not be fetched");

    Ok(normalize_output(output))
}

fn ip_address(address_type: &IpAddress) -> Result<Option<String>> {
    let command = process::Command::new(
        "dig",
        &[
            // decrease time and tries because commands are executed synchronously
            // TODO: make asychronous
            "+time=3",  // default: 5 seconds
            "+tries=1", // default: 3
            "@resolver1.opendns.com",
            match address_type {
                IpAddress::V4 => "A",
                IpAddress::V6 => "AAAA",
            },
            "myip.opendns.com",
            "+short",
        ],
    );

    let output = command.output().wrap_error(
        FEATURE_NAME,
        format!("ip address {} could not be fetched", address_type),
    );

    Ok(normalize_output(output))
}

fn normalize_output(output: Result<String>) -> Option<String> {
    match output {
        Ok(string) => {
            if string.is_empty() {
                None
            } else {
                Some(string)
            }
        },
        Err(error) => {
            info!("{}", error);
            None
        },
    }
}
