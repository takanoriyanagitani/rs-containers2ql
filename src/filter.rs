use std::collections::HashMap;

use async_graphql::Enum;
use async_graphql::InputObject;

use bollard::query_parameters::ListContainersOptions;
use bollard::query_parameters::ListContainersOptionsBuilder;

use crate::state::ContainerSummaryStateEnum;

pub type ContainerStatus = ContainerSummaryStateEnum;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Enum)]
pub enum ContainerHealth {
    Starting,
    Healthy,
    Unhealthy,
    None,
}

impl ContainerHealth {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Starting => "starting",
            Self::Healthy => "healthy",
            Self::Unhealthy => "unhealthy",
            Self::None => "none",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default, InputObject)]
pub struct ContainerFilterInput {
    pub id: Option<Vec<String>>,
    pub name: Option<Vec<String>>,
    pub status: Option<Vec<ContainerStatus>>,
    pub exited: Option<Vec<i32>>,
    pub health: Option<Vec<ContainerHealth>>,
    pub network: Option<Vec<String>>,
    pub volume: Option<Vec<String>>,
}

impl ContainerFilterInput {
    pub fn into_options(
        self,
        all: Option<bool>,
        limit: Option<i32>,
        size: Option<bool>,
    ) -> ListContainersOptions {
        let mut builder: ListContainersOptionsBuilder = ListContainersOptionsBuilder::new();

        if let Some(a) = all {
            builder = builder.all(a);
        }

        if let Some(l) = limit {
            builder = builder.limit(l);
        }

        if let Some(s) = size {
            builder = builder.size(s);
        }

        let mut filters: HashMap<&str, Vec<String>> = HashMap::new();

        if let Some(id) = self.id {
            filters.insert("id", id);
        }

        if let Some(names) = self.name {
            filters.insert("name", names);
        }

        if let Some(statuses) = self.status {
            let status_strings: Vec<String> = statuses
                .into_iter()
                .filter(|f| !f.eq(&ContainerSummaryStateEnum::Empty))
                .map(|s| s.as_str().into())
                .collect();
            filters.insert("status", status_strings);
        }

        if let Some(exiteds) = self.exited {
            let exited_strings: Vec<String> = exiteds.into_iter().map(|e| e.to_string()).collect();
            filters.insert("exited", exited_strings);
        }

        if let Some(healths) = self.health {
            let health_strings: Vec<String> =
                healths.into_iter().map(|h| h.as_str().into()).collect();
            filters.insert("health", health_strings);
        }

        if let Some(networks) = self.network {
            filters.insert("network", networks);
        }

        if let Some(volumes) = self.volume {
            filters.insert("volume", volumes);
        }

        builder = builder.filters(&filters);

        builder.build()
    }
}
