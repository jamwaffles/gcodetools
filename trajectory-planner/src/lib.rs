use gcode_parser::{
    token::{Coord, TokenType},
    Program,
};
use nalgebra::{VectorN, U9};
use std::fs;
use std::path::Path as FilePath;
use trajectories::{Path, Trajectory};

type Vector9 = VectorN<f64, U9>;

fn merge_vector9_and_coord(current: &Vector9, coord: &Coord) -> Vector9 {
    let mut new = current.clone();
    let coord_c = coord.clone();

    new[0] = coord_c
        .x
        .map(|c| c.as_f64_unchecked())
        .unwrap_or_else(|| current[0]);
    new[1] = coord_c
        .y
        .map(|c| c.as_f64_unchecked())
        .unwrap_or_else(|| current[1]);
    new[2] = coord_c
        .z
        .map(|c| c.as_f64_unchecked())
        .unwrap_or_else(|| current[2]);
    new[3] = coord_c
        .u
        .map(|c| c.as_f64_unchecked())
        .unwrap_or_else(|| current[3]);
    new[4] = coord_c
        .v
        .map(|c| c.as_f64_unchecked())
        .unwrap_or_else(|| current[4]);
    new[5] = coord_c
        .w
        .map(|c| c.as_f64_unchecked())
        .unwrap_or_else(|| current[5]);
    new[6] = coord_c
        .a
        .map(|c| c.as_f64_unchecked())
        .unwrap_or_else(|| current[6]);
    new[7] = coord_c
        .b
        .map(|c| c.as_f64_unchecked())
        .unwrap_or_else(|| current[7]);
    new[8] = coord_c
        .c
        .map(|c| c.as_f64_unchecked())
        .unwrap_or_else(|| current[8]);

    new
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_program_to_path() {
        let program = fs::read_to_string(&FilePath::new("./assets/simple_traj.ngc")).unwrap();

        let parsed = Program::from_str(&program).unwrap();

        let coords: Vec<Coord> = parsed
            .iter_flat()
            .cloned()
            .filter_map(|token| match token.token {
                TokenType::Coord(c) => Some(c),
                _ => None,
            })
            .collect();

        // Simulate the current state/position of the machine
        let current_position = Vector9::repeat(9.99);

        let waypoints: Vec<Vector9> = coords
            .iter()
            .scan(current_position, |current, coord| {
                let new = merge_vector9_and_coord(current, &coord);

                *current = new;

                Some(new)
            })
            .collect();

        println!("{:#?}", waypoints);

        let path = Path::from_waypoints(&waypoints, 0.001);

        let trajectory = Trajectory::new(
            path,
            Vector9::repeat(1.0),
            Vector9::repeat(1.0),
            0.000001,
            0.001,
        );

        assert!(trajectory.is_ok());
    }

    #[test]
    fn long_program() {
        let program = fs::read_to_string(&FilePath::new(
            "../test_files/universal_gcode_sender/stress_test.gcode",
        ))
        .unwrap();

        let parsed = Program::from_str(&program).unwrap();

        let coords: Vec<Coord> = parsed
            .iter_flat()
            .cloned()
            .filter_map(|token| match token.token {
                TokenType::Coord(c) => Some(c),
                _ => None,
            })
            .collect();

        // Simulate the current state/position of the machine
        let current_position = Vector9::repeat(9.99);

        let waypoints: Vec<Vector9> = coords
            .iter()
            .scan(current_position, |current, coord| {
                let new = merge_vector9_and_coord(current, &coord);

                *current = new;

                Some(new)
            })
            .collect();

        // println!("{:#?}", waypoints);

        let path = Path::from_waypoints(&waypoints, 0.001);

        let trajectory = Trajectory::new(
            path,
            Vector9::repeat(1.0),
            Vector9::repeat(1.0),
            0.000001,
            0.001,
        );

        assert!(trajectory.is_ok());
    }
}
