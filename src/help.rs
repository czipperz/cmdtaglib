//! Provides the `show_help(&String)` function, which shows help for a
//! user who incorrectly used the program.

use std::process::exit;

/// Shows help, using `progname` as the name of the program when display usage.  Exits with the
/// error code provided.
pub fn show_help(progname: &String, err_code: i32) -> ! {
    println!("");
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
    exit(err_code);
}
