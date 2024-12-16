use itertools::Itertools;
use pathfinding::{directed::astar, prelude::{astar_bag, dijkstra}};

use crate::{runner::AocDay, utils::{direction::Direction, Vec2}};

#[derive(Clone, PartialEq)]
enum Tile {
    Wall, Space
}

#[derive(Clone)]
struct Maze {
    maze: Vec<Vec<Tile>>,
    start: Vec2,
    end: Vec2,
}

type Node = (Vec2, Direction);

#[allow(dead_code)]
impl Maze {
    fn get(&self, pos: &Vec2) -> Tile {
        self.maze[pos.y as usize][pos.x as usize].clone()
    }

    fn dfs(&self, pos: &Vec2, dir: &Direction, visited: &[Vec2], steps: isize, rotations: isize, min_result: &mut isize) {
        if (steps + (1000 * rotations)) >= *min_result {
            return;
        }

        let new_tile = self.get(&pos);
        if new_tile == Tile::Wall {
            return;
        }

        if visited.contains(&pos) {
            return;
        }

        let mut new_visited = visited.to_vec();
        new_visited.push(*pos);

        if *pos == self.end {
            println!("Found end in {} steps and {} rotations (Score={})", steps, rotations, steps + 1000 * rotations);
            for (x, row) in self.maze.iter().enumerate() {
                for (y, tile) in row.iter().enumerate() {
                    if *pos == Vec2::new(y as isize, x as isize) {
                        print!("X");
                    } else if new_visited.contains(&Vec2::new(y as isize, x as isize)) {
                        print!("O"); 
                    } else {
                        match tile {
                            Tile::Wall => print!("#"),
                            Tile::Space => print!("."),
                        }
                    }
                }
                println!();
            }

            if (steps + 1000 * rotations) < *min_result {
                *min_result = steps + 1000 * rotations;
            }
            return;
        }

        for new_dir in (*dir).iter_forwards_from() {
            let new_pos = pos + new_dir.to_point();
            if new_pos.x < 0 || new_pos.y < 0 || new_pos.x >= self.maze[0].len() as isize || new_pos.y >= self.maze.len() as isize {
                continue;
            }

            if new_dir == *dir {
                self.dfs(&new_pos, &new_dir, &new_visited, steps + 1, rotations, min_result);
            } else {
                self.dfs(&new_pos, &new_dir, &new_visited, steps + 1, rotations + 1, min_result);
            }
        }
    }

    fn search(&self, pos: &Vec2, dir: &Direction) -> isize {
        let start = (pos.clone(), dir.clone());

        let result = dijkstra(
            &start,
            |node: &Node| {
                let orthogonal_moves = node.1.orthogonal();
            
                let mut next_nodes = Vec::with_capacity(3);
            
                for &dir in &orthogonal_moves {
                    let next_orthogonal = node.0.move_towards(dir);
                    if self.get(&next_orthogonal) != Tile::Wall {
                        next_nodes.push(((node.0, dir), 1000));
                    }
                }
            
                let next_straight = node.0.move_towards(node.1);
                if self.get(&next_straight) != Tile::Wall {
                    next_nodes.push(((next_straight, node.1), 1));
                }

                next_nodes
            },
            |node: &Node| node.0 == self.end
        );

        if let Some(result) = result {
            for (x, row) in self.maze.iter().enumerate() {
                for (y, tile) in row.iter().enumerate() {
                    if *pos == Vec2::new(y as isize, x as isize) {
                        print!("X");
                    } else if result.0.iter().any(|v| v.0.x == (x as isize) && v.0.y == (y as isize)) {
                        print!("O"); 
                    } else {
                        match tile {
                            Tile::Wall => print!("#"),
                            Tile::Space => print!("."),
                        }
                    }
                }
                println!();
            }
            result.1
        } else {
            0
        }
    }

    fn search_tiles(&self, pos: &Vec2, dir: &Direction) -> isize {
        let start = (pos.clone(), dir.clone());

        let result = astar_bag(
            &start,
            |node: &Node| {
                let orthogonal_moves = node.1.orthogonal();
            
                let mut next_nodes = Vec::with_capacity(3);
            
                for &dir in &orthogonal_moves {
                    let next_orthogonal = node.0.move_towards(dir);
                    if self.get(&next_orthogonal) != Tile::Wall {
                        next_nodes.push(((node.0, dir), 1000));
                    }
                }
            
                let next_straight = node.0.move_towards(node.1);
                if self.get(&next_straight) != Tile::Wall {
                    next_nodes.push(((next_straight, node.1), 1));
                }

                next_nodes
            },
            |node: &Node| node.0.manhattan_distance(&self.end),
            |node: &Node| node.0 == self.end
        );

        if let Some(result) = result {
            result.0
                .into_iter()
                .flat_map(|sol| sol.into_iter().map(|(c, _)| c))
                .unique()
                .count() as isize
        } else {
            0
        }
    }
}

pub struct ReindeerMaze{
    maze: Maze,
}

impl AocDay for ReindeerMaze {
    fn new(content: String) -> Self {
        // Parse the maze (E = End, S = Start, # = Wall, . = Path)
        let mut maze = Vec::new();
        let mut start = Vec2::new(0, 0);
        let mut end = Vec2::new(0, 0);
        for (y, line) in content.lines().enumerate() {
            let mut row = Vec::new();
            for (x, c) in line.chars().enumerate() {
                let tile = match c {
                    '#' => Tile::Wall,
                    '.' => Tile::Space,
                    'S' => {
                        start = Vec2::new(x as isize, y as isize);
                        Tile::Space
                    },
                    'E' => {
                        end = Vec2::new(x as isize, y as isize);
                        Tile::Space
                    },
                    _ => panic!("Invalid character in maze: {}", c),
                };
                row.push(tile);
            }
            maze.push(row);
        }

        Self {
            maze: Maze {
                maze,
                start,
                end,
            }
        }
    }

    fn part1(&self) -> String {
        self.maze.search(&self.maze.start, &Direction::Left).to_string()
    }

    fn part2(&self) -> String {
        self.maze.search_tiles(&self.maze.start, &Direction::Left).to_string()
    }
}