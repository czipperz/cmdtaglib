//! Provides the `CMD` enum, representing the different commands that can be invoked.

/// An enum of all the different commands that can be invoked.
#[derive(PartialEq, Eq, Debug)]
pub enum CMD {
    /// Display the album name
    Album,
    /// Set the song's album name
    SetAlbum(String),
    /// Display the artist(s)'s name(s)
    Artist,
    /// Set the song's artist
    SetArtist(String),
    /// Display the comment placed on the song
    Comment,
    /// Set the song's comment
    SetComment(String),
    /// Display the genre of the song
    Genre,
    /// Set the song's genre
    SetGenre(String),
    /// Display the title of the song
    Title,
    /// Set the song's title
    SetTitle(String),
    /// Display the track number of the song
    Track,
    /// Set the song's track number
    SetTrack(String),
    /// Display the year of the song
    Year,
    /// Set the song's year
    SetYear(String),
}
