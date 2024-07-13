use serde::{Serialize, Deserialize};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Hello {
    pub commandType: u8,
    pub udpPort: u16,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Welcome {
    pub replyType: u8,
    pub numStations: u16,
}

pub trait TypeInfo {
    fn type_of(&self) -> &'static str;
}

impl TypeInfo for Hello {
    fn type_of(&self) -> &'static str {
        "Hello"
    }
}

impl TypeInfo for Welcome {
    fn type_of(&self) -> &'static str {
        "Welcome"
    }
}
