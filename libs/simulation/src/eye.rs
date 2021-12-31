use crate::*;
use std::f32::consts::*;

const FOV_RANGE: f32 = 0.25;
const FOV_ANGLE: f32 = PI + FRAC_PI_4;
const CELLS: usize = 9;

#[derive(Debug, Clone)]
pub struct Eye {
    fov_range: f32,
    fov_angle: f32,
    cells: usize,
}

impl Eye {
    pub fn new(fov_range: f32, fov_angle: f32, cells: usize) -> Eye {
        assert!(fov_range > 0.0);
        assert!(fov_angle > 0.0);
        assert!(cells > 0);

        Eye {
            fov_range,
            fov_angle,
            cells,
        }
    }

    pub fn cells(&self) -> usize {
        self.cells
    }

    pub fn process_vision(
        &self,
        position: na::Point2<f32>,
        rotation: na::Rotation2<f32>,
        foods: &[Food],
    ) -> Vec<f32> {
        let mut cells = vec![0.0; self.cells];

        for food in foods {
            let vec = food.position - position;
            let dist = vec.norm();

            // Check if distance is greater than our vision range
            if dist >= self.fov_range {
                continue;
            }

            let angle = na::Rotation2::rotation_between(&na::Vector2::x(), &vec).angle();
            let angle = angle - rotation.angle();
            let angle = na::wrap(angle, -PI, PI);

            // Check if angle is within our vision angle
            if angle < -self.fov_angle / 2.0 || angle > self.fov_range / 2.0 {
                continue;
            }

            // Make angle relative to our FOV
            let angle = angle + self.fov_angle / 2.0;
            // Transform to [0, 1]
            let cell = angle / self.fov_angle;
            // Get cell index
            let cell = cell * (self.cells as f32);
            let cell = (cell as usize).min(cells.len() - 1);

            let energy = (self.fov_range - dist) / self.fov_range;
            cells[cell] += energy;
        }

        cells
    }
}

impl Default for Eye {
    fn default() -> Eye {
        Eye::new(FOV_RANGE, FOV_ANGLE, CELLS)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_EYE_CELLS: usize = 13;

    struct TestCase {
        foods: Vec<Food>,
        fov_range: f32,
        fov_angle: f32,
        x: f32,
        y: f32,
        rotation: f32,
        expected_vision: &'static str,
    }

    impl TestCase {
        fn run(self) {
            let eye = Eye::new(self.fov_range, self.fov_angle, TEST_EYE_CELLS);

            let actual_vision = eye.process_vision(
                na::Point2::new(self.x, self.y),
                na::Rotation2::new(self.rotation),
                &self.foods,
            );

            let actual_vision: Vec<_> = actual_vision
                .into_iter()
                .map(|cell| {
                    if cell >= 0.7 {
                        "#"
                    } else if cell >= 0.4 {
                        "+"
                    } else if cell >= 0.2 {
                        "-"
                    } else {
                        " "
                    }
                })
                .collect();

            let actual_vision = actual_vision.join("");

            assert_eq!(actual_vision, self.expected_vision);
        }
    }

    fn food(x: f32, y: f32) -> Food {
        Food {
            position: na::Point2::new(x, y),
        }
    }

    mod different_fov_ranges {
        use super::*;
        use test_case::test_case;

        #[test_case(1.0, "      +      ")]
        #[test_case(0.9, "      +      ")]
        #[test_case(0.8, "      -      ")]
        #[test_case(0.7, "      -      ")]
        #[test_case(0.6, "             ")]
        #[test_case(0.5, "             ")]
        #[test_case(0.4, "             ")]
        #[test_case(0.3, "             ")]
        #[test_case(0.2, "             ")]
        #[test_case(0.1, "             ")]
        fn test(fov_range: f32, expected_vision: &'static str) {
            TestCase {
                foods: vec![food(1.0, 0.5)],
                fov_angle: FRAC_PI_2,
                x: 0.5, // Center
                y: 0.5,
                rotation: 0.0,
                fov_range,
                expected_vision,
            }
            .run()
        }
    }

    mod different_rotations {
        use super::*;
        use test_case::test_case;

        #[test_case(0.00 * PI, "         #   ")]
        #[test_case(0.25 * PI, "        #    ")]
        #[test_case(0.50 * PI, "      #      ")]
        #[test_case(0.75 * PI, "    #        ")]
        #[test_case(1.00 * PI, "   #         ")]
        #[test_case(1.25 * PI, " #           ")]
        #[test_case(1.50 * PI, "            #")]
        #[test_case(1.75 * PI, "           # ")]
        #[test_case(2.00 * PI, "         #   ")]
        #[test_case(2.25 * PI, "        #    ")]
        #[test_case(2.50 * PI, "      #      ")]
        fn test(rotation: f32, expected_vision: &'static str) {
            TestCase {
                foods: vec![food(0.5, 1.0)],
                fov_range: 7.0,
                fov_angle: 2.0 * PI,
                x: 0.5, // Center
                y: 0.5,
                rotation,
                expected_vision,
            }
            .run()
        }
    }

    mod different_positions {
        use super::*;
        use test_case::test_case;

        // Check X axis
        #[test_case(0.9, 0.5, "             ")]
        #[test_case(0.8, 0.5, "            #")]
        #[test_case(0.7, 0.5, "#           #")]
        #[test_case(0.6, 0.5, " #         # ")]
        #[test_case(0.5, 0.5, "  #       #  ")]
        #[test_case(0.4, 0.5, "   +     +   ")]
        #[test_case(0.3, 0.5, "   +     +   ")]
        #[test_case(0.2, 0.5, "   +     +   ")]
        #[test_case(0.1, 0.5, "    +   +    ")]
        #[test_case(0.0, 0.5, "    +   +    ")]
        // Check Y axis
        #[test_case(0.5, 1.0, "  #          ")]
        #[test_case(0.5, 0.9, "    #        ")]
        #[test_case(0.5, 0.8, "     #       ")]
        #[test_case(0.5, 0.7, "+      #     ")]
        #[test_case(0.5, 0.6, " +      #    ")]
        #[test_case(0.5, 0.4, "    #      + ")]
        #[test_case(0.5, 0.3, "     #      +")]
        #[test_case(0.5, 0.2, "       #    +")]
        #[test_case(0.5, 0.1, "        #   +")]
        #[test_case(0.5, 0.0, "          # +")]
        fn test(x: f32, y: f32, expected_vision: &'static str) {
            TestCase {
                foods: vec![food(1.0, 0.25), food(1.0, 0.75)],
                fov_range: 2.0,
                fov_angle: FRAC_PI_2,
                x,
                y,
                rotation: 0.0,
                expected_vision,
            }
            .run()
        }
    }

    mod different_fov_angles {
        use super::*;
        use test_case::test_case;

        #[test_case(0.25 * PI, " #         #+")]
        #[test_case(0.50 * PI, "+  #     #  +")]
        #[test_case(0.75 * PI, "  + #   # +  ")]
        #[test_case(1.00 * PI, "   + # # +   ")]
        #[test_case(1.25 * PI, "   + # # +   ")]
        #[test_case(1.50 * PI, "+   +# #+    ")]
        #[test_case(1.75 * PI, "+   +# #+    ")]
        #[test_case(2.00 * PI, "#+  +# #+    ")]
        fn test(fov_angle: f32, expected_vision: &'static str) {
            TestCase {
                foods: vec![
                    food(0.0, 0.0),
                    food(0.0, 0.33),
                    food(0.0, 0.66),
                    food(0.0, 1.0),
                    food(1.0, 0.0),
                    food(1.0, 0.33),
                    food(1.0, 0.66),
                    food(1.0, 1.0),
                ],
                fov_range: 2.0,
                x: 0.5,
                y: 0.5,
                rotation: 0.0,
                fov_angle,
                expected_vision,
            }
            .run()
        }
    }
}
