use async_graphql::Enum;

use bollard::secret::ContainerSummaryStateEnum as BolState;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Enum, Hash)]
pub enum ContainerSummaryStateEnum {
    Empty,
    Created,
    Running,
    Paused,
    Restarting,
    Exited,
    Removing,
    Dead,
}

impl ContainerSummaryStateEnum {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Empty => "empty",
            Self::Created => "created",
            Self::Running => "running",
            Self::Paused => "paused",
            Self::Restarting => "restarting",
            Self::Exited => "exited",
            Self::Removing => "removing",
            Self::Dead => "dead",
        }
    }
}

impl From<BolState> for ContainerSummaryStateEnum {
    fn from(b: BolState) -> Self {
        match b {
            BolState::EMPTY => ContainerSummaryStateEnum::Empty,
            BolState::CREATED => ContainerSummaryStateEnum::Created,
            BolState::RUNNING => ContainerSummaryStateEnum::Running,
            BolState::PAUSED => ContainerSummaryStateEnum::Paused,
            BolState::RESTARTING => ContainerSummaryStateEnum::Restarting,
            BolState::EXITED => ContainerSummaryStateEnum::Exited,
            BolState::REMOVING => ContainerSummaryStateEnum::Removing,
            BolState::DEAD => ContainerSummaryStateEnum::Dead,
        }
    }
}
