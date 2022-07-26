pub async fn migrate(scylla: &scylla::Session) -> Result<(), Box<dyn std::error::Error>> {
    scylla.query(
        "CREATE KEYSPACE IF NOT EXISTS twee WITH REPLICATION = {'class' : 'SimpleStrategy', 'replication_factor' : 1}",
        &[],
    )
    .await?;
    scylla
        .query(
            "CREATE TABLE IF NOT EXISTS twee.users (username text primary key, email text, password text, salt text, totp text, privkey text, pubkey text, security text)",
            &[],
        )
        .await?;
    scylla
        .query(
            "CREATE TABLE IF NOT EXISTS twee.messages (recipient text, content text, sender text, sent timestamp, PRIMARY KEY (recipient, sent))",
            &[],
        )
        .await?;
    Ok(())
}
