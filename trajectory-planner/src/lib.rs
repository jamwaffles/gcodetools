use gcode_parser::{Program, Token};
use nalgebra::U9;
use std::fs;
use std::path::Path as FilePath;
use trajectories::Path;

// Test method that collects ONLY coordinates from a program and creates a path. It ignores things
// like velocity, rapid/feed move, feed rate, etc. It _only_ creates a path from straight waypoints
// fn convert_tokens_to_path(program: &Program) -> Path<U9> {
//     let waypoints: Vec<VectorN<U9>> = program
//         .tokens()
//         .iter()
//         .filter(|token| match token {
//             Token::Coord(_) => true,
//             _ => false,
//         })
//         .scan(Coord9::zeros(), |prev, token| match token {
//             Token::Coord(coord) => {
//                 let mut new_coord = Coord9::zeros();
//                 new_coord[0] = if let Some(curr) = &coord.x {
//                     curr.as_f64().unwrap()
//                 } else {
//                     prev[0]
//                 };
//                 new_coord[1] = if let Some(curr) = &coord.y {
//                     curr.as_f64().unwrap()
//                 } else {
//                     prev[1]
//                 };
//                 new_coord[2] = if let Some(curr) = &coord.z {
//                     curr.as_f64().unwrap()
//                 } else {
//                     prev[2]
//                 };
//                 new_coord[3] = if let Some(curr) = &coord.a {
//                     curr.as_f64().unwrap()
//                 } else {
//                     prev[3]
//                 };
//                 new_coord[4] = if let Some(curr) = &coord.b {
//                     curr.as_f64().unwrap()
//                 } else {
//                     prev[4]
//                 };
//                 new_coord[5] = if let Some(curr) = &coord.c {
//                     curr.as_f64().unwrap()
//                 } else {
//                     prev[5]
//                 };
//                 new_coord[6] = if let Some(curr) = &coord.u {
//                     curr.as_f64().unwrap()
//                 } else {
//                     prev[6]
//                 };
//                 new_coord[7] = if let Some(curr) = &coord.v {
//                     curr.as_f64().unwrap()
//                 } else {
//                     prev[7]
//                 };
//                 new_coord[8] = if let Some(curr) = &coord.w {
//                     curr.as_f64().unwrap()
//                 } else {
//                     prev[8]
//                 };
//                 *prev = new_coord.clone();
//                 Some(new_coord)
//             }
//             _ => panic!("Not a coord"),
//         })
//         .collect();

//     println!("WAYPOINTS {:#?}", waypoints);

//     Path::from_waypoints(&waypoints, 0.001)
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_program_to_path() {
        let program = fs::read_to_string(&FilePath::new("./assets/simple_traj.ngc")).unwrap();

        let parsed = Program::from_str(&program).unwrap();

        // let path = convert_tokens_to_path(&parsed);

        // let _trajectory = Trajectory::new(
        //     path,
        //     Coord9::repeat(1.0),
        //     Coord9::repeat(1.0),
        //     0.000001,
        //     0.001,
        // );
    }
}
