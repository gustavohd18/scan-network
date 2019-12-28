use super::device::Device;
use super::error::Error;
use eui::{EUI, EUI48};
use regex::Regex;
use std::convert::TryFrom;
use std::net::IpAddr;
use std::process::Command;

pub struct Network {
    devices: Vec<Device>,
    network_extern: String,
}

impl Network {
    pub fn new() -> Self {
        Self {
            devices: Vec::new(),
            network_extern: String::new(),
        }
    }

    pub fn get_all_devices_in_network(&mut self) -> Result<Vec<Device>, Error> {
        if self.devices.is_empty() {
            self.map_all_device_in_network()?;
        }

        let all_devices = self.devices.to_owned();

        return Ok(all_devices);
    }

    pub fn find_device_with_mac_address(&mut self, mac_address: String) -> Result<Device, Error> {
        self.valid_mac_address(mac_address.as_str())?;

        if self.devices.is_empty() {
            self.map_all_device_in_network()?;
        }

        let devices = self.devices.to_owned();

        for device in devices {
            if device.mac_address == mac_address {
                return Ok(device);
            }
        }

        return Err(Error::NotFindDevice);
    }

    pub fn find_device_with_ip(&mut self, ip: String) -> Result<Device, Error> {
        let ip_address: IpAddr = ip.parse()?;

        let ip = ip_address.to_string();

        if self.devices.is_empty() {
            self.map_all_device_in_network()?;
        }

        let devices = self.devices.to_owned();

        for device in devices {
            if device.ip == ip {
                return Ok(device);
            }
        }

        return Err(Error::NotFindDevice);
    }

    fn map_all_device_in_network(&mut self) -> Result<(), Error> {
        if self.network_extern.is_empty() {
            self.network_extern = self.start()?;
        }
        let regex_ip = Regex::new(r"\b\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}\b")
            .map_err(|_| Error::ExpressionRegularToIPIncorrect)?;

        let regex_mac = Regex::new(r"\b\w{1,3}:\w{1,3}:\w{1,3}:\w{1,3}:\w{1,3}:\w{1,3}\b")
            .map_err(|_| Error::ExpressionRegularToMACIncorrect)?;

        for network_information in self.network_extern.trim().lines() {
            let ip = regex_ip
                .find(network_information)
                .ok_or(Error::NotFoundIPAddress)?;

            let mac = regex_mac
                .find(network_information)
                .ok_or(Error::NotFoundMACAddress)?;

            let device = Device::new(ip.as_str().to_owned(), mac.as_str().to_owned());

            self.devices.push(device);
        }

        Ok(())
    }

    fn valid_mac_address(&self, mac_address: &str) -> Result<String, Error> {
        let mac_address = EUI48::try_from(mac_address)
            .map_err(|_| Error::InvalidMACAddress)?
            .to_colon_fmt();
        Ok(mac_address)
    }

    // this method is execute extern, so dont't run any unit test.
    fn start(&self) -> Result<String, Error> {
        // No matter OS System running create Command in this case.
        // one condicional is have ARP install in case UNIX stay install
        let all_mac_adress_and_ip_adress = Command::new("arp")
            .arg("-a")
            .output()
            .map_err(|_| Error::FailedRunARPCommand)?;

        let parser_string_command =
            String::from_utf8_lossy(&all_mac_adress_and_ip_adress.stdout).to_string();

        Ok(parser_string_command)
    }
}
