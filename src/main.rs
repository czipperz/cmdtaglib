extern crate taglib;
use taglib::Tag;

use std::env::args;
use std::process::exit;
use std::str::FromStr;

#[derive(Debug)]
enum CMD {
    Album,
    SetAlbum(String),
    Artist,
    SetArtist(String),
    Comment,
    SetComment(String),
    Genre,
    SetGenre(String),
    Title,
    SetTitle(String),
    Track,
    SetTrack(String),
    Year,
    SetYear(String),
}
use CMD::*;

fn main() {
    let mut args: Vec<_> = args().collect();
    let progname = args.remove(0);
    if args.len() <= 1 {
        show_help(&progname);
    }
    let fname: String = args.remove(0);
    let args = args;
    let it = args.iter();
    let mut cmds = Vec::new();
    for str in it {
        cmds.push(match parse_cmd(str, "album") {
            Some(Some(s)) => SetAlbum(s),
            Some(None) => Album,
            _ => {
                match parse_cmd(str, "artist") {
                    Some(Some(s)) => SetArtist(s),
                    Some(None) => Artist,
                    _ => {
                        match parse_cmd(str, "comment") {
                            Some(Some(s)) => SetComment(s),
                            Some(None) => Comment,
                            _ => {
                                match parse_cmd(str, "genre") {
                                    Some(Some(s)) => SetGenre(s),
                                    Some(None) => Genre,
                                    _ => {
                                        match parse_cmd(str, "title") {
                                            Some(Some(s)) => SetTitle(s),
                                            Some(None) => Title,
                                            _ => {
                                                match parse_cmd(str, "track") {
                                                    Some(Some(s)) => SetTrack(s),
                                                    Some(None) => Track,
                                                    _ => {
                                                        match parse_cmd(str, "year") {
                                                            Some(Some(s)) => SetYear(s),
                                                            Some(None) => Year,
                                                            _ => show_help(&progname),
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        });
    }
    let file = match taglib::File::new(&fname) {
        Ok(f) => f,
        Err(e) => {
            println!("Invalid file {}.  Error: {:?}", fname, e);
            exit(2);
        }
    };
    let mut tags = match file.tag() {
        Ok(t) => t,
        Err(e) => {
            println!("No available tags for {}.  Error: {:?}", fname, e);
            exit(3);
        }
    };
    for cmd in cmds {
        match cmd {
            Album => println!("{}", tags.album()),
            SetAlbum(s) => tags.set_album(&s),
            Artist => println!("{}", tags.artist()),
            SetArtist(s) => tags.set_artist(&s),
            Comment => println!("{}", tags.comment()),
            SetComment(s) => tags.set_comment(&s),
            Genre => println!("{}", tags.genre()),
            SetGenre(s) => tags.set_genre(&s),
            Title => println!("{}", tags.title()),
            SetTitle(s) => tags.set_title(&s),
            Track => println!("{}", tags.track()),
            SetTrack(s) => tags.set_track(u32::from_str(&s).unwrap()),
            Year => println!("{}", tags.year()),
            SetYear(s) => tags.set_year(u32::from_str(&s).unwrap()),
        }
    }
}

// return type means outer Option is for parsed correctly,
// inner Option is for whether to set or display
fn parse_cmd(cmd: &String, expected: &str) -> Option<Option<String>> {
    let mut ci = cmd.chars();
    let mut ei = expected.chars();
    loop {
        match (ci.next(), ei.next()) {
            (None, None) => return Some(None),
            (Some('='), None) => return Some(Some(ci.collect())),
            (Some(c), Some(e)) => {
                if c == e {
                    continue;
                } else {
                    return None;
                }
            }
            _ => return None,
        }
    }
}

fn show_help(progname: &String) -> ! {
    println!("Usage: {} <file> [commands]", progname);
    println!("");
    println!("Commands:");
    println!("  album                 Print out the album of the track");
    println!("  album=ALBUM           Sets the album of the track to ALBUM");
    println!("  artist                Print out the artist of the track");
    println!("  artist=ARTIST         Sets the artist of the track to ARTIST");
    println!("  comment               Print out the comment of the track");
    println!("  comment=COMMENT       Sets the comment of the track to COMMENT");
    println!("  genre                 Print out the genre of the track");
    println!("  genre=GENRE           Sets the genre of the track to GENRE");
    println!("  title                 Print out the title of the track");
    println!("  title=TITLE           Sets the title of the track to TITLE");
    println!("  track                 Print out the track number of the track");
    println!("  track=TRACK_NUMBER    Sets the track number of the track to TRACK_NUMBER");
    println!("  year                  Print out the year of the track or 0 if it isn't present");
    println!("  year=YEAR             Sets the year of the track to YEAR");
    exit(1);
}
