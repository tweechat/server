pub async fn migrate(scylla: &scylla::Session) -> Result<(), Box<dyn std::error::Error>> {
    scylla.query(
        "CREATE KEYSPACE IF NOT EXISTS ks WITH REPLICATION = {'class' : 'SimpleStrategy', 'replication_factor' : 1}",
        &[],
    )
    .await?;
    scylla
        .query(
            "CREATE TABLE IF NOT EXISTS ks.users (id int primary key, name string)",
            &[],
        )
        .await?;
    Ok(())
}