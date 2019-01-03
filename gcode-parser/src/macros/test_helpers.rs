#[macro_export]
macro_rules! coord {
    ($x:expr, $y:expr, $z:expr) => {
        Coord {
            x: Some($x),
            y: Some($y),
            z: Some($z),
            ..Coord::default()
        }
    };
    ($x:expr, $y:expr) => {
        Coord {
            x: Some($x),
            y: Some($y),
            ..Coord::default()
        }
    }; // TODO: Other permutations of args
}
