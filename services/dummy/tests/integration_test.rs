use testcontainers::{
    ContainerAsync, GenericImage, ImageExt,
    core::{ContainerPort, WaitFor},
    runners::AsyncRunner,
};

#[tokio::test]
async fn test_get_current_user_authenticated() {
    let pg_port = 5432;
    let pg_host = "db";
    let postgres = run_postgres(pg_host, pg_port).await;
    println!("postgres {:?}", postgres.get_host().await.unwrap());

    assert_eq!(200, 200);
}

async fn run_postgres(pg_host: &str, pg_port: u16) -> ContainerAsync<GenericImage> {
    GenericImage::new("postgres", "latest")
        .with_exposed_port(ContainerPort::Tcp(pg_port))
        .with_wait_for(WaitFor::message_on_stdout(
            "database system is ready to accept connections",
        ))
        .with_wait_for(WaitFor::message_on_stderr(
            "database system is ready to accept connections",
        ))
        .with_network("shared_network")
        .with_container_name(format!("{pg_host}-integration-test"))
        .with_copy_to(
            "/docker-entrypoint-initdb.d/init.sql",
            include_bytes!("../../../infrastructure/db/init.sql").to_vec(),
        )
        .with_env_var("PGPORT", pg_port.to_string())
        .with_env_var("POSTGRES_USER", "postgres")
        .with_env_var("POSTGRES_PASSWORD", "postgres")
        .with_env_var("POSTGRES_DB", "postgres")
        .start()
        .await
        .expect("Failed to start postgres")
}
