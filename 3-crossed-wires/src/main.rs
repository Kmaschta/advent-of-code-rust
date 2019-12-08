use std::env;
use std::fs;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, PartialEq)]
struct Path {
    points: Vec<Point>,
    corners: Vec<Point>,
}

fn add_point_to_path(path: Path, instruction_arg: &String) -> Path {
    let mut instruction = instruction_arg.clone();
    let mut new_path = Path {
        points: path.points.to_vec(),
        corners: path.corners.to_vec(),
    };

    let direction = instruction.remove(0);
    let steps = instruction.parse::<i32>().unwrap();
    let last_point = new_path.points[new_path.points.len() - 1];

    match direction {
        'U' => {
            for y in (last_point.y + 1)..(last_point.y + steps + 1) {
                new_path.points.push(Point { x: last_point.x, y })
            }
        }
        'D' => {
            for y in ((last_point.y - steps)..(last_point.y)).rev() {
                new_path.points.push(Point { x: last_point.x, y })
            }
        }
        'R' => {
            for x in (last_point.x + 1)..(last_point.x + steps + 1) {
                new_path.points.push(Point { x, y: last_point.y })
            }
        }
        'L' => {
            for x in ((last_point.x - steps)..(last_point.x)).rev() {
                new_path.points.push(Point { x, y: last_point.y })
            }
        }
        _ => panic!("Wrong direction"),
    };

    new_path
        .corners
        .push(new_path.points[new_path.points.len() - 1]);

    // println!(
    //     "{:?}: ({:?}) -> ({:?})",
    //     instruction_arg,
    //     last_point,
    //     new_path.points[new_path.points.len() - 1]
    // );

    new_path
}

fn parse_path(origin: Point) -> impl Fn(&Vec<String>) -> Path {
    move |path| {
        println!("Creating a new path");
        path.iter().fold(
            Path {
                points: vec![origin],
                corners: vec![origin],
            },
            |path, instruction| add_point_to_path(path, instruction),
        )
    }
}

fn get_shared_points(paths: &Vec<Path>) -> Vec<Point> {
    let mut shared_points = paths[0].points.to_vec();
    let total_points = shared_points.len();
    println!("Scanning {:?} points for shared points", total_points);

    for path in paths {
        println!(
            "Removing {:?} corners from shared points",
            path.corners.len()
        );

        shared_points.retain(|shared_point| {
            for corner in path.corners.iter() {
                if corner.x == shared_point.x && corner.y == shared_point.y {
                    return false;
                }
            }

            return true;
        });
    }

    // Shared points is a clone of the first path
    // No need to iterate over the first path, so we start at index 1
    for i in 1..(paths.len()) {
        let mut j = 0;
        shared_points.retain(|shared_point| {
            let mut point_found = false;

            j += 1;
            println!("Scanning point {:?}/{:?}", j, total_points);
            for point in paths[i].points.iter() {
                if point.x == shared_point.x && point.y == shared_point.y {
                    point_found = true;
                    break;
                }
            }

            point_found
        });
    }

    shared_points
}

fn compute_manhattan_distance(a: Point, b: Point) -> i32 {
    (b.x - a.x).abs() + (b.y - a.y).abs()
}

fn find_closted_point_distance(origin: Point, points: &Vec<Point>) -> i32 {
    println!("Finding closest point");

    return points.iter().rev().fold(0, |distance, point| {
        if *point == origin {
            return distance;
        }

        let new_distance = compute_manhattan_distance(origin, *point);

        // println!(
        //     "Origin: {:?} - Closest: {:?} - Point: {:?} - distance: {:?}",
        //     origin, distance, *point, new_distance
        // );

        if distance == 0 || new_distance < distance {
            return new_distance;
        }

        return distance;
    });
}

fn get_paths_distance(paths_str: Vec<Vec<String>>) -> i32 {
    let origin = Point { x: 0, y: 0 };
    let paths: Vec<Path> = paths_str.iter().map(parse_path(origin)).collect();
    let shared_points = get_shared_points(&paths);
    return find_closted_point_distance(origin, &shared_points);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_point_to_path_up() {
        let original_path = Path {
            points: vec![Point { x: 0, y: 0 }],
            corners: vec![Point { x: 0, y: 0 }],
        };

        let path = add_point_to_path(original_path, &String::from("U2"));
        assert_eq!(
            path,
            Path {
                points: vec![
                    Point { x: 0, y: 0 },
                    Point { x: 0, y: 1 },
                    Point { x: 0, y: 2 }
                ],
                corners: vec![Point { x: 0, y: 0 }, Point { x: 0, y: 2 }]
            }
        );
    }

    #[test]
    fn test_add_point_to_path_down() {
        let original_path = Path {
            points: vec![Point { x: 0, y: 0 }],
            corners: vec![Point { x: 0, y: 0 }],
        };

        let path = add_point_to_path(original_path, &String::from("D3"));
        assert_eq!(
            path,
            Path {
                points: vec![
                    Point { x: 0, y: 0 },
                    Point { x: 0, y: -1 },
                    Point { x: 0, y: -2 },
                    Point { x: 0, y: -3 }
                ],
                corners: vec![Point { x: 0, y: 0 }, Point { x: 0, y: -3 }]
            }
        );
    }

    #[test]
    fn test_add_point_to_path_right() {
        let original_path = Path {
            points: vec![Point { x: -12, y: 40 }],
            corners: vec![
                Point { x: 0, y: 0 },
                Point { x: 0, y: 40 },
                Point { x: -12, y: 40 },
            ],
        };

        let path = add_point_to_path(original_path, &String::from("R5"));
        assert_eq!(
            path,
            Path {
                points: vec![
                    Point { x: -12, y: 40 },
                    Point { x: -11, y: 40 },
                    Point { x: -10, y: 40 },
                    Point { x: -9, y: 40 },
                    Point { x: -8, y: 40 },
                    Point { x: -7, y: 40 },
                ],
                corners: vec![
                    Point { x: 0, y: 0 },
                    Point { x: 0, y: 40 },
                    Point { x: -12, y: 40 },
                    Point { x: -7, y: 40 }
                ],
            }
        );
    }

    #[test]
    fn test_add_point_to_path_left() {
        let original_path = Path {
            points: vec![Point { x: 35, y: 3 }],
            corners: vec![Point { x: 35, y: 3 }],
        };

        let path = add_point_to_path(original_path, &String::from("L12"));
        assert_eq!(
            path,
            Path {
                points: vec![
                    Point { x: 35, y: 3 },
                    Point { x: 34, y: 3 },
                    Point { x: 33, y: 3 },
                    Point { x: 32, y: 3 },
                    Point { x: 31, y: 3 },
                    Point { x: 30, y: 3 },
                    Point { x: 29, y: 3 },
                    Point { x: 28, y: 3 },
                    Point { x: 27, y: 3 },
                    Point { x: 26, y: 3 },
                    Point { x: 25, y: 3 },
                    Point { x: 24, y: 3 },
                    Point { x: 23, y: 3 },
                ],
                corners: vec![Point { x: 35, y: 3 }, Point { x: 23, y: 3 }],
            }
        );
    }

    #[test]
    fn test_get_paths_distance() {
        let paths = vec![
            vec![
                String::from("R98"),
                String::from("U47"),
                String::from("R26"),
                String::from("D63"),
                String::from("R33"),
                String::from("U87"),
                String::from("L62"),
                String::from("D20"),
                String::from("R33"),
                String::from("U53"),
                String::from("R51"),
            ],
            vec![
                String::from("U98"),
                String::from("R91"),
                String::from("D20"),
                String::from("R16"),
                String::from("D67"),
                String::from("R40"),
                String::from("U7"),
                String::from("R15"),
                String::from("U6"),
                String::from("R7"),
            ],
        ];

        assert_eq!(get_paths_distance(paths), 135);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    println!("Reading {}...", filename);

    let content = fs::read_to_string(filename).expect("Error while reading file");
    let lines = content
        .split("\n")
        .map(|s| String::from(s))
        .collect::<Vec<String>>();

    let paths = lines
        .iter()
        .map(|s| {
            s.split(",")
                .map(|sub| String::from(sub))
                .collect::<Vec<String>>()
        })
        .collect::<Vec<Vec<String>>>();

    println!("Computing Manathan distance for {:?} paths", paths.len());

    let result = get_paths_distance(paths);

    println!("Result is {:?}", result);
}
