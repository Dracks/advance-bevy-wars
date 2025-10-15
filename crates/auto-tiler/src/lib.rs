use bevy::{math::{I16Vec2, IVec2,  UVec2, Vec2}};


pub struct SimpleAutoTiler<T,I> {
    pub terrain: T,
    pub none: I,
    pub n: I,
    pub e: I,
    pub s: I,
    pub w: I,
    pub ne: I,
    pub ns: I,
    pub nw: I,
    pub es: I,
    pub ew: I,
    pub sw: I,
    pub nes: I,
    pub new: I,
    pub nse: I,
    pub nesw: I,
}

pub trait BoardTrait<T> {
    fn get(&self, pos: UVec2) -> &T;
    fn get_list(&self, pos: Vec<IVec2>) -> Vec<(UVec2, T)>;
}

pub trait AutoTiler<T, I> {
    fn get(&self, board:&impl BoardTrait<T>, pos: UVec2) -> Vec<I>;
}

fn adjacent(pos: UVec2) -> Vec<IVec2> {
    let i_pos = pos.as_ivec2();
    vec![i_pos+IVec2::X, i_pos+IVec2::Y, i_pos-IVec2::X, i_pos-IVec2::Y]
}

impl<T, I: Clone+Copy> SimpleAutoTiler<T, I> {
    fn get_one_related(&self, relative:I16Vec2) -> Vec<I> {
        let elem = match (relative.x, relative.y) {
            (x, 0) => if x<0 { self.w } else {self.e },
            (0, y) => if y<0 { self.n } else {self.s },
            coords => panic!("No support for diagonals {:?}", coords)
        };
        vec![elem.clone()]
    }
}

impl<T: Eq, I: Clone+Copy> AutoTiler<T, I> for SimpleAutoTiler<T, I> {
    fn get(&self, board:&impl BoardTrait<T>, pos: UVec2) -> Vec<I> {
        let adjacent = board.get_list(adjacent(pos));
        let coords : Vec<&UVec2> = adjacent.iter()
                .filter(|(_, test)| *test == self.terrain)
                .map(|(coord, _)| coord).collect();
        match coords[..] {
            [] => vec![self.none.clone()],
            [coord]  => self.get_one_related(coord.as_i16vec2()-pos.as_i16vec2()),
            _ => todo!("")
        }
    }
}

#[cfg(test)]
mod tests {
    use bevy::math::{uvec2};

    use super::*;

    struct TestBoard {
        list: Vec<(UVec2, i32)>
    }

    impl BoardTrait<i32> for TestBoard {
        fn get(&self, _pos: UVec2) -> &i32 {
            &self.list[0].1
        }

        fn get_list(&self, _pos: Vec<IVec2>) -> Vec<(UVec2, i32)> {
            self.list.clone()
        }
    }

    fn get_tiler(terrain: i32) -> SimpleAutoTiler<i32, char> {
        SimpleAutoTiler {
            terrain,
            none: 'a',
            n: 'b',
            e: 'c',
            s: 'd',
            w: 'e',
            ne: 'f',
            ns: 'g',
            nw: 'h',
            es: 'i',
            ew: 'j',
            sw: 'k',
            nes: 'l',
            new: 'm',
            nse: 'n',
            nesw: 'o',
        }
    }

    #[test]
    fn test_1_way_handling() {
        let board = TestBoard {
            list: vec![
                (uvec2(1,0), 1),
                (uvec2(0,1), 2),
                (uvec2(2,1), 3),
                (uvec2(1,2), 4),
            ]
        };

        let tiler_0 = get_tiler(0);
        let none = tiler_0.get(&board, uvec2(1, 1));
        assert_eq!(none.len(), 1);
        assert_eq!(none[0], 'a');

        let tiler_1 = get_tiler(1);
        let n = tiler_1.get(&board, uvec2(1, 1));
        assert_eq!(n.len(), 1);
        assert_eq!(n[0], 'b');

        let tiler_2 = get_tiler(3);
        let e = tiler_2.get(&board, uvec2(1, 1));
        assert_eq!(e.len(), 1);
        assert_eq!(e[0], 'c');
    }
}
