use day_14::{Map, Vec2, ORIGIN};

fn main() {
    let input = include_str!("../../input.txt");

    let mut map = Map::parse(input);
    let floor_y = map.max.y + 2;

    let mut grains = 0;
    while !map.points.contains_key(&ORIGIN) {
        simulate_grain(&mut map, floor_y);
        grains += 1;
    }
    map.print();
    println!("grains: {}", grains);

    fn simulate_grain(map: &mut Map, floor_y: i32) {
        let mut pos = ORIGIN;
        loop {
            let candidate_positions = [
                pos.add(&Vec2::DOWN),
                pos.add(&Vec2::DOWN_LEFT),
                pos.add(&Vec2::DOWN_RIGHT),
            ];
            let mut has_moved = false;
            for candidate_position in &candidate_positions {
                let blocked = map.points.contains_key(candidate_position)
                    || candidate_position.y >= floor_y;
                if !blocked {
                    has_moved = true;
                    pos = candidate_position.clone();
                    break;
                }
            }
            if !has_moved {
                map.add_point(pos, 'o');
                return;
            }
        }
    }
}
