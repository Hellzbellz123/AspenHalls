use bevy::prelude::*;

use std::collections::HashSet;

use super::grid2d::Grid2D;

struct Generator2DPlugin;

struct CellType(u8);
const NONE: CellType = CellType(0);
const ROOM: CellType = CellType(1);
const HALLWAY: CellType = CellType(2);

struct Room {
    bounds: RectInt,
}

impl Room {
    fn new(location: Vec2, size: Vec2) -> Room {
        Room {
            bounds: RectInt::new(location, size),
        }
    }

    fn intersect(a: &Room, b: &Room) -> bool {
        !(a.bounds.x >= b.bounds.x + b.bounds.width
            || a.bounds.x + a.bounds.width <= b.bounds.x
            || a.bounds.y >= b.bounds.y + b.bounds.height
            || a.bounds.y + a.bounds.height <= b.bounds.y)
    }
}

struct Generator2D {
    size: Vec2,
    room_count: u32,
    room_max_size: Vec2,
    cube_materials: (Handle<StandardMaterial>, Handle<StandardMaterial>),
}

impl Generator2D {
    fn new(
        size: Vec2,
        room_count: u32,
        room_max_size: Vec2,
        cube_materials: (Handle<StandardMaterial>, Handle<StandardMaterial>),
    ) -> Generator2D {
        Generator2D {
            size,
            room_count,
            room_max_size,
            cube_materials,
        }
    }

    fn generate(
        &mut self,
        commands: &mut Commands,
        materials: &mut ResMut<Assets<StandardMaterial>>,
    ) {
        let mut random = Random::new(0);
        let mut grid = Grid2D::new(self.size, Vec2::ZERO);
        let mut rooms = Vec::new();

        self.place_rooms(&mut random, &mut grid, &mut rooms, commands, materials);
        self.triangulate(&rooms);
        self.create_hallways(&mut random);
        self.pathfind_hallways(&mut grid, &rooms, commands, materials);
    }

    fn place_rooms(
        &mut self,
        random: &mut Random,
        grid: &mut Grid2D<CellType>,
        rooms: &mut Vec<Room>,
        commands: &mut Commands,
        materials: &mut ResMut<Assets<StandardMaterial>>,
    ) {
        for _ in 0..self.room_count {
            let location = Vec2::new(
                random.range(0, self.size.x as i32) as f32,
                random.range(0, self.size.y as i32) as f32,
            );

            let room_size = Vec2::new(
                random.range(1, self.room_max_size.x as i32 + 1) as f32,
                random.range(1, self.room_max_size.y as i32 + 1) as f32,
            );

            let add = {
                let new_room = Room::new(location, room_size);
                let buffer = Room::new(
                    location + Vec2::new(-1.0, -1.0),
                    room_size + Vec2::new(2.0, 2.0),
                );

                !rooms.iter().any(|room| Room::intersect(room, &buffer))
                    && new_room.bounds.x > 0.0
                    && new_room.bounds.x + new_room.bounds.width < self.size.x
                    && new_room.bounds.y > 0.0
                    && new_room.bounds.y + new_room.bounds.height < self.size.y
            };

            if add {
                let new_room = Room::new(location, room_size);
                rooms.push(new_room);
                self.place_room(
                    new_room.bounds.position,
                    new_room.bounds.size,
                    commands,
                    materials,
                );

                for pos in new_room.bounds.all_positions_within() {
                    grid[pos] = ROOM;
                }
            }
        }
    }

    fn triangulate(&self, rooms: &[Room]) {
        let mut vertices = Vec::new();

        for room in rooms {
            vertices.push(Vertex::new(
                room.bounds.position + room.bounds.size / 2.0,
                room,
            ));
        }

        let delaunay = Delaunay2D::triangulate(&vertices);
    }

    fn create_hallways(&self, random: &mut Random) {
        let mut edges = Vec::new();

        for edge in delaunay.edges.iter() {
            edges.push(PrimEdge(edge.u, edge.v));
        }

        let mst = Prim::minimum_spanning_tree(&edges, edges[0].u);

        let mut selected_edges = HashSet::new();
        let mut remaining_edges = HashSet::from_iter(edges.iter().cloned());
        remaining_edges.difference(&selected_edges);

        for edge in remaining_edges {
            if random.next_double() < 0.125 {
                selected_edges.insert(edge);
            }
        }
    }

    fn pathfind_hallways(
        &self,
        grid: &Grid2D<CellType>,
        rooms: &[Room],
        commands: &mut Commands,
        materials: &mut ResMut<Assets<StandardMaterial>>,
    ) {
        let mut a_star = DungeonPathfinder2D::new(self.size);

        for edge in selected_edges {
            let start_room = (edge.u as Vertex<Room>).item;
            let end_room = (edge.v as Vertex<Room>).item;

            let start_pos_f = start_room.bounds.center();
            let end_pos_f = end_room.bounds.center();
            let start_pos = Vec2::new(start_pos_f.x as i32, start_pos_f.y as i32);
            let end_pos = Vec2::new(end_pos_f.x as i32, end_pos_f.y as i32);

            let path = a_star.find_path(start_pos, end_pos, |a, b| {
                let mut path_cost = DungeonPathfinder2DPathCost::default();

                path_cost.cost = b.position.distance(end_pos); // heuristic

                if grid[b.position] == ROOM {
                    path_cost.cost += 10;
                } else if grid[b.position] == NONE {
                    path_cost.cost += 5;
                } else if grid[b.position] == HALLWAY {
                    path_cost.cost += 1;
                }

                path_cost.traversable = true;

                path_cost
            });

            if let Some(path) = path {
                for i in 0..path.len() {
                    let current = path[i];

                    if grid[current] == NONE {
                        grid[current] = HALLWAY;
                    }

                    if i > 0 {
                        let prev = path[i - 1];
                        let delta = current - prev;
                    }
                }

                for pos in path {
                    if grid[pos] == HALLWAY {
                        self.place_hallway(pos, commands, materials);
                    }
                }
            }
        }
    }

    fn place_cube(
        &self,
        location: Vec2,
        size: Vec2,
        material: Handle<StandardMaterial>,
        commands: &mut Commands,
        materials: &mut ResMut<Assets<StandardMaterial>>,
    ) {
        commands.spawn_bundle(PbrBundle {
            mesh: bevy::render::mesh::Cube {
                size: size.extend(1.0),
            }
            .into(),
            material,
            transform: Transform::from_translation(location.extend(0.0)),
            ..Default::default()
        });
    }

    fn place_room(
        &self,
        location: Vec2,
        size: Vec2,
        commands: &mut Commands,
        materials: &mut ResMut<Assets<StandardMaterial>>,
    ) {
        self.place_cube(
            location,
            size,
            self.cube_materials.0.clone(),
            commands,
            materials,
        );
    }

    fn place_hallway(
        &self,
        location: Vec2,
        commands: &mut Commands,
        materials: &mut ResMut<Assets<StandardMaterial>>,
    ) {
        self.place_cube(
            location,
            Vec2::new(1.0, 1.0),
            self.cube_materials.1.clone(),
            commands,
            materials,
        );
    }
}

impl Plugin for Generator2DPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system());
    }
}

fn setup(mut commands: Commands, mut materials: ResMut<Assets<StandardMaterial>>) {
    let generator = Generator2D::new(
        Vec2::new(10.0, 10.0),
        5,
        Vec2::new(3.0, 3.0),
        (
            materials.add(StandardMaterial::red()),
            materials.add(StandardMaterial::blue()),
        ),
    );

    generator.generate(&mut commands, &mut materials);
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(Generator2DPlugin)
        .run();
}
