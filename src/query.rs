use std::io;
use std::sync::Arc;

use bollard::Docker;
use bollard::models::ContainerSummary;
use bollard::query_parameters::ListContainersOptions;

use async_graphql::Object;

use async_graphql::EmptyMutation;
use async_graphql::EmptySubscription;
use async_graphql::Schema;

use crate::filter::ContainerFilterInput;

use crate::container::BasicContainerSummary;
use crate::container::list_containers;

pub struct Query {
    pub docker: Arc<Docker>,
}

#[Object]
impl Query {
    async fn list_containers(
        &self,
        all: Option<bool>,
        limit: Option<i32>,
        size: Option<bool>,
        filter: Option<ContainerFilterInput>,
    ) -> Result<Vec<BasicContainerSummary>, io::Error> {
        let fin: ContainerFilterInput = filter.unwrap_or_default();
        let opts: Option<ListContainersOptions> = Some(fin.into_options(all, limit, size));
        let d: &Docker = &self.docker;

        let v: Vec<ContainerSummary> = list_containers(d, opts).await?;
        Ok(v.into_iter().map(|b| b.into()).collect())
    }
}

pub type ContainerSchema = Schema<Query, EmptyMutation, EmptySubscription>;

pub fn schema_new(q: Query) -> ContainerSchema {
    Schema::build(q, EmptyMutation, EmptySubscription).finish()
}

pub fn docker2schema(d: Arc<Docker>) -> ContainerSchema {
    let query = Query { docker: d };
    schema_new(query)
}
