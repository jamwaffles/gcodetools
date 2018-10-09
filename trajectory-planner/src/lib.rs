use gcode_parser::{Program, Token};
use nalgebra::U9;
use trajectories::Path;

// Test method that collects ONLY coordinates from a program and creates a path. It ignores things
// like velocity, rapid/feed move, feed rate, etc. It _only_ creates a path from straight waypoints
fn convert_tokens_to_path(program: &Program) -> Path<U9> {
    let waypoints = program.tokens().iter().filter(|token| match token {
        Token::Coord(_) => true,
        _ => false,
    });

    // TODO: Remove `None`s by tracking which coords have changed. Do it in this crate; the parser
    // shouldn't care about this.

    unimplemented!()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
