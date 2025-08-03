use async_graphql::SimpleObject;

use bollard::secret::Port;

use crate::ptyp::PortTypeEnum;

#[derive(Debug, PartialEq, Eq, Clone, SimpleObject)]
pub struct BasicPort {
    pub host_ip: Option<String>,
    pub container_port: u16,
    pub host_port: Option<u16>,
    pub port_type: Option<PortTypeEnum>,
}

pub fn btyp2typ(btyp: bollard::secret::PortTypeEnum) -> PortTypeEnum {
    btyp.into()
}

impl From<Port> for BasicPort {
    fn from(port: bollard::secret::Port) -> Self {
        BasicPort {
            host_ip: port.ip,
            container_port: port.private_port,
            host_port: port.public_port,
            port_type: port.typ.map(|b| b.into()),
        }
    }
}
