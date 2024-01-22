pub use crate::prelude::*;


#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Render {
    pub color: ColorPair,       // stores both a foreground and background color in a single struct
    pub glyph: FontCharType,    // store a single character of glyph
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Player;              // empty struct with no data. Serves as a tag indicating that an entity is a player

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Enemy;              // empty struct with no data. Serves as a tag indicating that an entity is a player
