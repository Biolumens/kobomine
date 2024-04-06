extern crate rusqlite;
use clap::Parser;
use rusqlite::{Connection, OpenFlags, Result};
use std::{fs::OpenOptions, path::PathBuf};
use std::io::prelude::*;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Provide Kobo SQLite DB file
    #[arg(short, long, default_value = "./KoboReader.sqlite")]
    file: PathBuf,
}
struct Bookmark {
    text: String,
}
fn main() -> Result<()> {
    let args = Args::parse();
    let conn = Connection::open_with_flags(
        args.file,
        OpenFlags::SQLITE_OPEN_READ_WRITE
            | OpenFlags::SQLITE_OPEN_URI
            | OpenFlags::SQLITE_OPEN_NO_MUTEX,
    )?;

    let mut stmt = conn.prepare("SELECT Text, DateCreated FROM Bookmark")?;
    let rows = stmt.query_map([], |row| {
        Ok(Bookmark {
            text: row.get(0)?,
   //         date: row.get(1)?,
        })
    })?;

    //  for bookmark in rows {
    //      match bookmark {
    //          Ok(p) => println!("Date: {}, Text: {}", p.date, p.text),
    //          Err(e) => eprintln!("Error: {e:?}"),
    //    }
    //  }

    //let mut bkvec: Vec<String> = Vec::new();
    //let mut temp: String;
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
    .open("Yomichan_me.html").unwrap();

    file.write_all("<html>\n\t<head>\n\t\t<title>Use Yomichan on me!</title>\n\t</head>\n\t<body>\n".as_bytes()).unwrap();

    for bookmark in rows {
        match bookmark {
            Ok(b) => file.write_all(("\t\t<p>".to_owned() + &b.text + "</p>\n").as_bytes()).unwrap(),
          //  Ok(b) => file.write_all("<p>".to_owned() + &b.text + "</p>\n"),
            Err(e) => eprintln!("Error: {e:?}"),
        }
    }
    
    file.write_all("\t</body>\n</html>".as_bytes()).unwrap();

    println!("Sentences are located at \"Yomichan-me.html\"");

    Ok(())
}
