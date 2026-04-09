use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[repr(u8)]
pub enum MoveStatus {
    Null = 0,
    Offline = 1,
    Online = 2,
}

impl MoveStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Null => "NULL",
            Self::Offline => "OFFLINE",
            Self::Online => "ONLINE",
        }
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[repr(u8)]
pub enum MoveAction {
    Anchored = 0,
    Online = 1,
    Offline = 2,
    Unanchored = 3,
}

impl MoveAction {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Anchored => "ANCHORED",
            Self::Online => "ONLINE",
            Self::Offline => "OFFLINE",
            Self::Unanchored => "UNANCHORED",
        }
    }
}

#[derive(Deserialize)]
pub struct MoveAssemblyStatus {
    pub status: MoveStatus,
}
