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

pub enum Orbital { Planet(u64), Starbase(u64) }

pub struct TypeAndIDDescriptor {
    r#type: u64,
    id: u64
}

pub struct EmpireFlag {
    icon: Image,
    background: Image,
    colors: [Option<String>;4] // Always has 4 strings, but they can be "null"
}
struct Image {
    catagory: String,
    file: String,
}