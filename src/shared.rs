pub struct Coordinate {
    x: f32,
    y: f32,
    origin: u64,
    randomized: Option<bool>
}
pub struct Flag {
    name: String,
    value: u64
}
pub enum Ethos {
    Authoritarian,Egalitarian,
    Materialist,Spiritualist,
    Militarist,Pacifist,
    Xenophile,Xenophobe
}
pub struct Resource {
    name: String,
    amount: f32
}