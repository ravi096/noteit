use serde::{Serialize, Deserialize};
use pretty_env_logger;
use log::{info, debug, error};
use anyhow::Error;
use clap::{App, SubCommand, Arg, AppSettings};
use std::env;
use std::path::Path;
use std::fs::File;
use std::io::Write;
use std::fs::OpenOptions;

#[derive(Serialize, Deserialize, Debug)]
struct Note {
    tstamp: String,
    md5: String,
    note: String,
    context: Option<String>,
}

impl Note {
    pub fn new(note: &str) -> Note {
        Note {
            tstamp: "my timestamp".to_string(),
            md5: "my md5".to_string(),
            note: note.to_string(),
            context: None,
        }
    }
}

struct NoteBook {
    name: String,
}

impl NoteBook {
    pub fn new(name: &str) -> NoteBook {
        NoteBook{name: name.to_string()}
    }

    pub fn addnote(&self, note: &str) -> Result<Note, Error> {
        let nobj = Note::new(note);
        self.savenote(&nobj)?;
        Ok(nobj)
    }

    pub fn savenote(&self, nobj: &Note) -> Result<(), Error> {
        // Append to file
        let path = Path::new(&env::var("HOME")
                             .unwrap_or("/tmp".to_string()))
                             .join(&self.name);

        debug!("Opening notebook {}", path.display());

        let mut fh = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(path)?;
        fh.write(nobj.note.as_bytes())?;
        Ok(())
    }
}

fn main() -> Result<(), Error> {
    pretty_env_logger::init();
    info!("Hello, world!");
    let matches = App::new("A simple noteit....")
        .version("0.1")
        .author("Ravikumar Alluboyina")
        .about("Trivial note taking assistant")
        .setting(AppSettings::SubcommandRequired)
        .subcommand(SubCommand::with_name("add")
                    .subcommand(SubCommand::with_name("book")
                                .arg(Arg::with_name("name")
                                     .help("New book name")
                                     .required(true)
                                     .index(1)))
                    .subcommand(SubCommand::with_name("note")
                                .arg(Arg::with_name("note")
                                    .help("Add a note")
                                    .required(true)
                                    .index(1))))
        .get_matches();

    debug!("{:#?}", matches);

    match matches.subcommand() {
        ("add", Some(m)) => {
            match m.subcommand() {
                
                ("book", Some(m)) => {
                    Ok(())
                },
                
                ("note", Some(m)) => {
                    match NoteBook::new("mybook").addnote(m.value_of("note").unwrap()) {
                        Ok(_) => {
                            info!("Successfully added note"); 
                            Ok(())
                        },
                        Err(e) => {
                            error!("{}", e);
                            Err(e)
                        }
                    }
                },
                _ => Ok(())
            }
        },
        ("del", Some(m)) => {
            match m.subcommand() {
                ("book", Some(m)) => Ok(()),
                ("note", Some(m)) => Ok(()),
                _ => Ok(())
            }
        },
        ("list", Some(m)) => {
            match m.subcommand() {
                ("book", Some(m)) => Ok(()),
                ("note", Some(m)) => Ok(()),
                _ => Ok(())
            }
        },
        _ => Ok(())
    }
}
