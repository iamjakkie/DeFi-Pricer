
pub async fn connect_to_db() {
    let (client, connection) = tokio_postgres::connect(
        "url", tokio_postgres::NoTls).await.unwrap();
}