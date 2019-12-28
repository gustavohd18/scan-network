#[derive(Debug, PartialEq)]
pub enum Error {
    FailedRunARPCommand,
    ExpressionRegularToIPIncorrect,
    ExpressionRegularToMACIncorrect,
    NotFoundIPAddress,
    NotPossibleParserDevice,
    InvalidMACAddress,
    NotFoundConnectNetwork,
    NotFoundMACAddress,
    NotParserIP(std::net::AddrParseError),
    NotFindDevice,
}

impl From<std::net::AddrParseError> for Error {
    fn from(error: std::net::AddrParseError) -> Self {
        Error::NotParserIP(error)
    }
}
