use std::env;
use std::io;
use std::process::ExitCode;

use tokio::net::TcpListener;

use async_graphql_axum::GraphQLRequest;
use async_graphql_axum::GraphQLResponse;

use rs_containers2ql::bollard;

use bollard::Docker;

use rs_containers2ql::query::ContainerSchema;
use rs_containers2ql::query::docker2schema;

fn env2sock_path() -> Result<String, io::Error> {
    env::var("DOCKER_SOCK_PATH")
        .map_err(|_| io::Error::new(io::ErrorKind::NotFound, "DOCKER_SOCK_PATH not set"))
}

fn env2timeout_seconds() -> Result<u64, io::Error> {
    let s: String = env::var("DURATION_SECONDS").unwrap_or_else(|_| "10".to_string());
    s.parse().map_err(|_| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            "DURATION_SECONDS must be a valid number",
        )
    })
}

fn env2addr_port() -> Result<String, io::Error> {
    env::var("LISTEN_ADDR_PORT")
        .map_err(|_| io::Error::new(io::ErrorKind::NotFound, "LISTEN_ADDR_PORT not set"))
}

async fn req2res(s: &ContainerSchema, req: GraphQLRequest) -> GraphQLResponse {
    s.execute(req.into_inner()).await.into()
}

async fn sub() -> Result<(), io::Error> {
    let addr_port: String = env2addr_port()?;
    let sock_path: String = env2sock_path()?;
    let to_seconds: u64 = env2timeout_seconds()?;

    let dver = bollard::API_DEFAULT_VERSION;

    let d: Docker =
        Docker::connect_with_socket(&sock_path, to_seconds, dver).map_err(io::Error::other)?;

    let s = docker2schema(d.into());
    let sdl: String = s.sdl();
    std::fs::write("./docker-containers.gql", sdl.as_bytes())?;

    let listener = TcpListener::bind(addr_port).await?;

    let app = axum::Router::new().route(
        "/",
        axum::routing::post(|req: GraphQLRequest| async move { req2res(&s, req).await }),
    );

    axum::serve(listener, app).await
}

#[tokio::main]
async fn main() -> ExitCode {
    if let Err(e) = sub().await {
        eprintln!("{e}");
        std::process::exit(1);
    }
    std::process::exit(0)
}
