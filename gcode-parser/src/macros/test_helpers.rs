/// Create a coordinate with only some fields populated
#[macro_export]
macro_rules! coord {
    ($x:expr, $y:expr, $z:expr) => {
        Coord {
            x: Some(($x).into()),
            y: Some(($y).into()),
            z: Some(($z).into()),
            ..Coord::default()
        }
    };
    ($x:expr, $y:expr) => {
        Coord {
            x: Some(($x).into()),
            y: Some(($y).into()),
            ..Coord::default()
        }
    }; // TODO: Other permutations of args
}
