use crate::prelude::*;

const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
}

pub fn map_idx(x: i32, y: i32) -> usize {
    ((y * SCREEN_WIDTH) + x) as usize
}

pub struct Map {
    pub tiles: Vec<TileType>,
}

impl Map {
    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Floor; NUM_TILES],
        }
    }

    pub fn in_bounds(&self, point : Point) -> bool {
        point.x >= 0 && point.x < SCREEN_WIDTH && point.y >= 0 && point.y < SCREEN_HEIGHT
    }

    pub fn try_idx(&self, point : Point) -> Option<usize> {
        if !self.in_bounds(point) {
            None
        } else {
            Some(map_idx(point.x, point.y))
        }
    }

    pub fn can_enter_tile(&self, point : Point) -> bool {
        self.in_bounds(point) && self.tiles[map_idx(point.x, point.y)]==TileType::Floor
    }

    fn is_exit_valid(&self, x: i32, y: i32) -> bool {
        if x < 1 || x > SCREEN_WIDTH-1 || y < 1 || y > SCREEN_HEIGHT-1 { 
            return false; 
        }
        let idx = map_idx(x, y);
        self.tiles[idx] == TileType::Floor
    }
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(SCREEN_WIDTH, SCREEN_HEIGHT)
    }
}

impl BaseMap for Map {
    fn is_opaque(&self, idx: usize) -> bool {
        self.tiles[idx as usize] == TileType::Wall
    }
    
    fn get_available_exits(&self, idx: usize) -> SmallVec<[(usize, f32); 10]> {
        let mut exits = SmallVec::new();
        let x = idx as i32 % SCREEN_WIDTH;
        let y = idx as i32 / SCREEN_WIDTH;
        let w = SCREEN_WIDTH;

        if self.is_exit_valid(x-1, y) { exits.push((idx-1, 1.0)) };
        if self.is_exit_valid(x+1, y) { exits.push((idx+1, 1.0)) };
        if self.is_exit_valid(x, y-1) { exits.push((idx-w as usize, 1.0)) };
        if self.is_exit_valid(x, y+1) { exits.push((idx+w as usize, 1.0)) };

        // Diagonals
        if self.is_exit_valid(x-1, y-1) { exits.push(((idx-w as usize)-1, 1.45)); }
        if self.is_exit_valid(x+1, y-1) { exits.push(((idx-w as usize)+1, 1.45)); }
        if self.is_exit_valid(x-1, y+1) { exits.push(((idx+w as usize)-1, 1.45)); }
        if self.is_exit_valid(x+1, y+1) { exits.push(((idx+w as usize)+1, 1.45)); }

        exits
    }

    fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32 {
        let w = SCREEN_WIDTH;
        let p1 = Point::new(idx1 as i32 % w, idx1 as i32 / w);
        let p2 = Point::new(idx2 as i32 % w, idx2 as i32 / w);
        DistanceAlg::Pythagoras.distance2d(p1, p2)
    }
}