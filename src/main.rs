//! A command line application that allows you to interact with `taglib`.
//! It allows for easy reading and writing of tags for a script or user.

extern crate taglib;
use taglib::Tag;

use std::env::args;
use std::process::exit;
use std::str::FromStr;

pub mod cmd;
use cmd::CMD::*;

pub mod help;
use help::show_help;

pub mod parse;
use parse::parse_cmd;

fn main() {
    let mut args: Vec<_> = args().collect();
    let progname = args.remove(0);
    if args.len() <= 1 {
        println!("{}",
                 if args.len() == 0 {
                     "No arguments were provided"
                 } else {
                     "Need to give commands as well as file name"
                 });
        show_help(&progname, 1);
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
                                                            _ => {
                                                                if str == "help" ||
                                                                   str == "--help" {
                                                                    println!("Showing help:");
                                                                    show_help(&progname, 0);
                                                                } else {
                                                                    println!("Unrecognized \
                                                                              command: {}",
                                                                             str);
                                                                    show_help(&progname, 1);
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
