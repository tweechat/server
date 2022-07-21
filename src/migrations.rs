pub async fn migrate(scylla: &scylla::Session) -> Result<(), Box<dyn std::error::Error>> {
    scylla.query(
        "CREATE KEYSPACE IF NOT EXISTS twee WITH REPLICATION = {'class' : 'SimpleStrategy', 'replication_factor' : 1}",
        &[],
    )
    .await?;
    scylla
        .query(
            "CREATE TABLE IF NOT EXISTS twee.users (id bigint primary key, username string, email string, password string, salt string, totp string)",
            &[],
        )
        .await?;
    Ok(())
}
