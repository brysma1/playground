use std::{
    error::Error,
    fs::File,
    io::{BufReader, Read},
};

use dotenv::dotenv;
use libsql::Builder;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv()?;

    let url =
        std::env::var("TURSO_DATABASE_URL").expect("TURSO_DATABASE_URL must be set in .env file");
    let token =
        std::env::var("TURSO_AUTH_TOKEN").expect("TURSO_AUTH_TOKEN must be set in .env file");

    let db = Builder::new_remote(url, token).build().await?;
    let conn = db.connect()?;

    println!("Trying to create table if it doesn't exist");
    conn.execute(
        "CREATE TABLE IF NOT EXISTS test_from_rust (\
            id INTEGER PRIMARY KEY,\
            text TEXT);",
        (),
    )
    .await?;

    let mut reader = BufReader::new(File::open("/dev/urandom")?);

    println!("Getting current number of rows");

    let mut rows = conn
        .query("SELECT COUNT(*) FROM test_from_rust;", ())
        .await?;

    let mut row_count: u64 = 0;

    while let Some(row) = rows.next().await? {
        row_count = row.get(0)?;
        println!("row count: {}", row_count);
    }

    println!("Creating {} random text rows", 100 - row_count);

    for _ in 0..(100 - row_count) {
        let mut buf = vec![0; 16];
        reader.read_exact(&mut buf)?;

        buf.iter_mut().for_each(|v| {
            *v = (*v as u64 * 93 / 255) as u8 + 33;
        });

        let text = String::from_utf8(buf)?;
        conn.execute("INSERT INTO test_from_rust (text) VALUES ( ?1 );", [text])
            .await?;
    }

    println!("\nGetting first 10 rows");

    let mut rows = conn
        .query("SELECT * FROM test_from_rust LIMIT 10;", ())
        .await?;

    while let Some(row) = rows.next().await? {
        let id: u64 = row.get(0)?;
        let text: String = row.get(1)?;

        println!("{:02}: {}", id, text);
    }

    println!("Getting total row count");

    let mut rows = conn
        .query("SELECT COUNT(*) FROM test_from_rust;", ())
        .await?;

    while let Some(row) = rows.next().await? {
        row_count = row.get(0)?;
        println!("total row count in table: {}", row_count);
    }

    Ok(())
}
