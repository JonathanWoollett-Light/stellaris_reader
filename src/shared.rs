use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Coordinate {
    x: f32,
    y: f32,
    origin: u64,
    randomized: Option<bool>
}
#[derive(Serialize, Deserialize)]
pub struct Flag {
    name: String,
    value: u64
}
#[derive(Serialize, Deserialize)]
pub enum Ethos {
    Authoritarian,Egalitarian,
    Materialist,Spiritualist,
    Militarist,Pacifist,
    Xenophile,Xenophobe
}
#[derive(Serialize, Deserialize)]
pub struct Resource {
    name: String,
    amount: f32
}
#[derive(Serialize, Deserialize)]
pub enum Orbital { Planet(u64), Starbase(u64) }
#[derive(Serialize, Deserialize)]
pub struct TypeAndIDDescriptor {
    r#type: u64,
    id: u64
}
#[derive(Serialize, Deserialize)]
pub struct EmpireFlag {
    icon: Image,
    background: Image,
    colors: [Option<String>;4] // Always has 4 strings, but they can be "null"
}
#[derive(Serialize, Deserialize)]
struct Image {
    catagory: String,
    file: String,
}

#[derive(Serialize, Deserialize)]
pub enum Gender { Male, Female }