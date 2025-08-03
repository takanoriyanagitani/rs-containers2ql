use async_graphql::Enum;

use bollard::secret::PortTypeEnum as BolTyp;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Enum)]
pub enum PortTypeEnum {
    Empty,
    Tcp,
    Udp,
    Sctp,
}

impl From<BolTyp> for PortTypeEnum {
    fn from(b: BolTyp) -> Self {
        match b {
            BolTyp::EMPTY => PortTypeEnum::Empty,
            BolTyp::TCP => PortTypeEnum::Tcp,
            BolTyp::UDP => PortTypeEnum::Udp,
            BolTyp::SCTP => PortTypeEnum::Sctp,
        }
    }
}
