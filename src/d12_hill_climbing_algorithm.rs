use std::{
    borrow::Borrow,
    collections::{BinaryHeap, HashSet},
};

type Point = (usize, usize);
type NodeGraph2D = Vec<Vec<Node>>;

#[derive(Debug)]
struct Connection {
    node_index: Point,
    weight: i32,
}

#[derive(Debug)]
pub struct Node {
    connections: Vec<Connection>,
    height: i32,
    distance_to_start: usize,
}

pub fn part_1(input: &str) -> usize {
    let (mut nodes, start, end) = parse(input);

    nodes[start.1][start.0].distance_to_start = 0;

    // Dijkstra
    let mut visited = HashSet::new();
    let mut unvisited = BinaryHeap::new();
    unvisited.push((nodes[start.1][start.0].distance_to_start, start));
    visited.insert(start);

    // let mut closest = start.0.abs_diff(end.0) + start.1.abs_diff(end.1);

    while let Some((_, current)) = unvisited.pop() {
        // if current.0.abs_diff(end.0) + current.1.abs_diff(end.1) < closest {
        //     closest = current.0.abs_diff(end.0) + current.1.abs_diff(end.1);
        //     dbg!(closest, current);
        // }

        // if current == end {
        //     dbg!("!");
        //     break;
        // }

        for conn in 0..nodes[current.1][current.0].connections.len() {
            let conn_weight = nodes[current.1][current.0].connections[conn].weight;
            let conn_index = nodes[current.1][current.0].connections[conn].node_index;
            let old_distance = nodes[conn_index.1][conn_index.0].distance_to_start;

            // Find distance
            if conn_weight <= 1 {
                nodes[conn_index.1][conn_index.0].distance_to_start = nodes[conn_index.1]
                    [conn_index.0]
                    .distance_to_start
                    .min(nodes[current.1][current.0].distance_to_start + 1);

                if !visited.contains(&conn_index)
                    && nodes[conn_index.1][conn_index.0].distance_to_start < usize::MAX
                {
                    visited.insert(conn_index);
                    unvisited.push((
                        nodes[conn_index.1][conn_index.0].distance_to_start,
                        conn_index,
                    ));
                } else if nodes[conn_index.1][conn_index.0].distance_to_start < old_distance {
                    unvisited.push((
                        nodes[conn_index.1][conn_index.0].distance_to_start,
                        conn_index,
                    ));
                }
            }
        }
    }

    nodes[end.1][end.0].distance_to_start
}

pub fn part_2(input: &str) -> i32 {
    todo!()
}

pub fn parse(input: &str) -> (NodeGraph2D, Point, Point) {
    let area_width = input.lines().next().unwrap().len();
    let flat_input: String = input.lines().collect();

    let start_i = flat_input.find('S').unwrap();
    let start = (start_i % area_width, start_i / area_width);

    let end_i = flat_input.find('E').unwrap();
    let end = (end_i % area_width, end_i / area_width);

    let mut nodes: NodeGraph2D = input
        .lines()
        .map(|line| {
            line.chars()
                .into_iter()
                .map(|height| Node {
                    connections: Vec::new(),
                    height: match height {
                        'S' => 0,
                        'E' => 'z' as i32 - 'a' as i32,
                        _ => height as i32 - 'a' as i32,
                    },
                    distance_to_start: usize::MAX,
                })
                .collect()
        })
        .collect();

    for y in 0..nodes.len() {
        for x in 0..nodes.first().unwrap().len() {
            // Left
            if x >= 1 {
                let new_connection = Connection {
                    node_index: (x - 1, y),
                    weight: nodes[y][x - 1].height - nodes[y][x].height,
                };
                nodes[y][x].connections.push(new_connection);
            }

            // Right
            if x + 1 < nodes.first().unwrap().len() {
                let new_connection = Connection {
                    node_index: (x + 1, y),
                    weight: nodes[y][x + 1].height - nodes[y][x].height,
                };
                nodes[y][x].connections.push(new_connection);
            }

            // Up
            if y >= 1 {
                let new_connection = Connection {
                    node_index: (x, y - 1),
                    weight: nodes[y - 1][x].height - nodes[y][x].height,
                };
                nodes[y][x].connections.push(new_connection)
            }

            // Down
            if y + 1 < nodes.len() {
                let new_connection = Connection {
                    node_index: (x, y + 1),
                    weight: nodes[y + 1][x].height - nodes[y][x].height,
                };
                nodes[y][x].connections.push(new_connection)
            }
        }
    }

    (nodes, start, end)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    const TEST_INPUT: &str = "src\\d12_hill_climbing_algorithm.test";
    const INPUT: &str = "src\\d12_hill_climbing_algorithm.input";

    #[test]
    fn check_parse() {
        let input = fs::read_to_string(TEST_INPUT).unwrap();

        let (n, s, e) = parse(&input);
        dbg!(s, e);
        for i in 0..n.len() {
            for j in 0..n[i].len() {
                print!("{:?} ", n[i][j].connections.len())
            }
            println!()
        }
    }

    #[test]
    fn test_part_1() {
        let input = fs::read_to_string(TEST_INPUT).unwrap();

        assert_eq!(part_1(&input), 31)
    }

    #[test]
    fn run_part_1() {
        let input = fs::read_to_string(INPUT).unwrap();

        println!("{:?}", part_1(&input))
    }

    #[test]
    fn test_part_2() {
        let input = fs::read_to_string(TEST_INPUT).unwrap();

        assert_eq!(part_2(&input), 0)
    }

    #[test]
    fn run_part_2() {
        let input = fs::read_to_string(INPUT).unwrap();

        println!("{:?}", part_2(&input))
    }
}
