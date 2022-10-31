use std::collections::HashMap;
use crossterm::event::{KeyCode};


#[derive(Debug)]
pub struct HexKeyMap(pub HashMap<KeyCode, usize>);

impl Default for HexKeyMap {
    fn default() -> Self {
        let mut hmap = HashMap::new();

        hmap.insert(KeyCode::Char('1'), 0);
        hmap.insert(KeyCode::Char('2'), 1);
        hmap.insert(KeyCode::Char('3'), 2);
        hmap.insert(KeyCode::Char('4'), 3);
        hmap.insert(KeyCode::Char('q'), 4);
        hmap.insert(KeyCode::Char('w'), 5);
        hmap.insert(KeyCode::Char('e'), 6);
        hmap.insert(KeyCode::Char('r'), 7);
        hmap.insert(KeyCode::Char('a'), 8);
        hmap.insert(KeyCode::Char('s'), 9);
        hmap.insert(KeyCode::Char('d'), 10);
        hmap.insert(KeyCode::Char('f'), 11);
        hmap.insert(KeyCode::Char('z'), 12);
        hmap.insert(KeyCode::Char('x'), 13);
        hmap.insert(KeyCode::Char('c'), 14);
        hmap.insert(KeyCode::Char('v'), 15);

        Self(hmap)
    }
}