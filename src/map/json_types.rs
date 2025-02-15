use serde::Deserialize;

#[derive(Deserialize, Clone, Copy, Debug)]
pub enum Dimension {
    Light,
    Dark,
}

#[derive(Deserialize)]
pub struct MapData {
    pub name: String,
    pub size: i32,
    pub start_x: i32,
    pub start_y: i32,
    pub goal_x: i32,
    pub goal_y: i32,
    pub walls: Vec<Wall>,
    pub doors: Vec<Door>,
    pub keys: Vec<Key>,
}

#[derive(Deserialize)]
pub struct Wall {
    pub x: i32,
    pub y: i32,
    pub dimension: Dimension,
}

#[derive(Deserialize)]
pub struct Door {
    pub x: i32,
    pub y: i32,
    pub id: u32,
    pub dimension: Dimension,
}

#[derive(Deserialize)]
pub struct Key {
    pub x: i32,
    pub y: i32,
    pub door_id: u32,
    pub dimension: Dimension,
}