use bevy::{math::vec3, prelude::Vec3};
use rand::{prelude::ThreadRng, Rng};

// Explanation
//
// Points on Screen:
//             12
//      ------------------
//     |                  |
//     |                  |
// -20 |        0         | 20
//     |                  |
//     |                  |
//      ------------------
//             -12
//
// First and last point are out of screen
// its for the bug spawn and dead out of screen
// but move on visible screen (up explanation)
//
//                    20
//       ------------------------------
//      |                              |
//      |             12               |
//      |                              |
//      |      ------------------      |
//      |     |                  |     |
//      |     |                  |     |
//  -25 | -20 |        0         | 20  | 25
//      |     |                  |     |
//      |     |                  |     |
//      |      ------------------      |
//      |                              |
//      |            -12               |
//      |                              |
//       ------------------------------
//                   -20
//
// So the range for spawn and despawn are:
//
//       left/bottom     or     right/top
//                       ||
//     -25 <= x >= -20   ||   20 <= x >= 25
//     -20 <= y >= -12   ||   12 <= y >= 20
//
//
pub fn generate_points(mut rnd: ThreadRng) -> Vec<Vec3> {
    let mut points = Vec::new();

    // start point out of screen
    points.push(vec3(
        // Generate based on left/right margin
        if rnd.gen_bool(0.5) {
            rnd.gen_range(-25.0..=-20.)
        } else {
            rnd.gen_range(20.0..=25.)
        },
        // Generate based on top/bottom margin
        if rnd.gen_bool(0.5) {
            rnd.gen_range(-20.0..=-12.)
        } else {
            rnd.gen_range(12.0..=20.)
        },
        0.,
    ));

    for _ in 0..7 {
        points.push(vec3(
            rnd.gen_range(-20.0..=20.),
            rnd.gen_range(-12.0..=12.),
            0.,
        ));
    }

    // end point out of screen
    points.push(vec3(
        if rnd.gen_bool(0.5) {
            rnd.gen_range(-25.0..=-20.)
        } else {
            rnd.gen_range(20.0..=25.)
        },
        if rnd.gen_bool(0.5) {
            rnd.gen_range(-20.0..=-12.)
        } else {
            rnd.gen_range(12.0..=20.)
        },
        0.,
    ));

    points
}
