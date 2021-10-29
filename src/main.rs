use postgres::{Client, Error, NoTls};

fn main() -> Result<(), Error> {
    let mut client = Client::connect(
        "postgresql://dboperator:operatorpass123@localhost:5243/postgres",
        NoTls,
    )?;

    client.batch_execute(
        "
        CREATE TABLE IF NOT EXISTS app_user (
            id              SERIAL PRIMARY KEY,
            username        VARCHAR UNIQUE NOT NULL,
            password        VARCHAR NOT NULL,
            email           VARCHAR UNIQUE NOT NULL
            )
    ",
    )?;

    client.execute(
        "INSERT INTO app_user (username, password, email) VALUES ($1, $2, $3)",
        &[&"user1", &"mypass", &"user@test.com"],
    )?;

    client.execute(
        "INSERT INTO app_user (username, password, email) VALUES ($1, $2, $3)",
        &[&"user2", &"mypass2", &"use2@gmail.com"],
    )?;

    client.execute(
        "INSERT INTO app_user (username, password, email) VALUES ($1, $2, $3)",
        &[&"user3", &"anotherpass", &"mister3@test.com"],
    )?;

    for row in client.query("SELECT id, username, password, email FROM app_user", &[])? {
        let id: i32 = row.get(0);
        let username: &str = row.get(1);
        let password: &str = row.get(2);
        let email: &str = row.get(3);

        println!(
            "found app user: {}) {} | {} | {}",
            id, username, password, email
        );
    }

    client.execute(
        "UPDATE app_user SET username=$1 WHERE id=$2",
        &[&"jack1", &4],
    )?;

    client.execute("DELETE FROM app_user WHERE id=$1", &[&1])?;
    client.execute("DELETE FROM app_user WHERE id=$1", &[&3])?;

    Ok(())
}
