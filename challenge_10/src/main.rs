use std::fmt;
#[derive(Debug)]
enum Location {
    Unknown,
    Anonymous,
    Known(f64, f64), // latitude, longitude
}
impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Location::Unknown => write!(f, "Unknown location"),
            Location::Anonymous => write!(f, "Anonymous location"),
            Location::Known(lat, lon) => write!(f, "Known location at latitude: {}, longitude: {}", lat, lon),
        }
    }
}
impl Location {
    fn display(&self) {
        println!("{}", self);
    }
}

fn main() {
    let address = Location::Unknown;
    address.display();
    let address = Location::Anonymous;
    address.display();
    let address = Location::Known(28.608295, -80.604177);
    address.display();
}