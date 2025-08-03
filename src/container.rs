use std::io;

use bollard::Docker;
use bollard::query_parameters::ListContainersOptions;

use async_graphql::SimpleObject;

use bollard::secret::ContainerSummary;

use crate::port::BasicPort;
use crate::state::ContainerSummaryStateEnum;

pub type Unixtime = i64;

pub fn bport2port(b: bollard::secret::Port) -> BasicPort {
    b.into()
}

pub fn bstate2state(b: bollard::secret::ContainerSummaryStateEnum) -> ContainerSummaryStateEnum {
    b.into()
}

#[derive(Debug, PartialEq, Eq, Clone, SimpleObject)]
pub struct BasicContainerSummary {
    pub id: Option<String>,
    pub names: Option<Vec<String>>,
    pub image: Option<String>,
    pub command: Option<String>,
    pub created: Option<Unixtime>,
    pub status: Option<String>,
    pub ports: Option<Vec<BasicPort>>,
    pub state: Option<ContainerSummaryStateEnum>,
}

impl From<ContainerSummary> for BasicContainerSummary {
    fn from(b: ContainerSummary) -> Self {
        BasicContainerSummary {
            id: b.id,
            names: b.names,
            image: b.image,
            command: b.command,
            created: b.created,
            status: b.status,
            ports: b.ports.map(|v| v.into_iter().map(|b| b.into()).collect()),
            state: b.state.map(|b| b.into()),
        }
    }
}

pub async fn list_containers(
    d: &Docker,
    opts: Option<ListContainersOptions>,
) -> Result<Vec<ContainerSummary>, io::Error> {
    d.list_containers(opts).await.map_err(io::Error::other)
}
