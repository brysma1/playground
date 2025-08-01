use std::{
    error::Error,
    fs::File,
    io::{BufReader, BufWriter, Read, Write, stdout},
};

use libsql::Builder;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let db = Builder::new_local("test.db").build().await?;
    let con = db.connect()?;

    con.execute(
        "CREATE TABLE IF NOT EXISTS person (\
        id INTEGER PRIMARY KEY,\
        randtext TEXT);",
        (),
    )
    .await?;

    let mut reader = BufReader::new(File::open("/dev/urandom")?);

    for _ in 0..10000 {
        let mut buf = vec![0; 8];
        reader.read_exact(&mut buf)?;

        buf.iter_mut().for_each(|v| {
            *v = (*v as u64 * 93 / 255) as u8 + 33;
        });

        let text = String::from_utf8(buf)?;
        con.execute("INSERT INTO person (randtext) VALUES ( ?1 );", [text])
            .await?;
    }

    let mut rows = con.query("SELECT * FROM person;", ()).await?;

    let mut writer = BufWriter::new(stdout());

    while let Some(row) = rows.next().await? {
        writer.write_fmt(format_args!(
            "{:05}: {:}\n",
            row.get_value(0)?.as_integer().unwrap(),
            row.get_value(1)?.as_text().unwrap()
        ))?;
    }

    writer.flush()?;

    Ok(())
}
