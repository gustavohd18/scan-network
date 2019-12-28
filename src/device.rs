#[derive(Debug, PartialEq, Clone)]
pub struct Device {
    pub ip: String,
    pub mac_address: String,
}

impl Device {
    pub fn new(ip: String, mac_address: String) -> Self {
        Self {
            ip: ip,
            mac_address: mac_address,
        }
    }
}
