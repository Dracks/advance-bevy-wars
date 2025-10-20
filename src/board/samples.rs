use crate::board::Board;

#[allow(dead_code)]
pub fn base_board() -> Board {
    let data = vec![
        vec!["p", "p", "p", "p", "p", "p", "p", "p", "p", "p", "p", "p", "p", "p", "p", "p", "p", "p", "p", "p"],
        vec!["p", "s", "s", "B", "p", "r", "r", "r", "r", "r", "p", "p", "s", "s", "s", "p", "b", "b", "b", "p"],
        vec!["p", "B", "s", "B", "p", "r", "p", "r", "p", "r", "p", "p", "s", "p", "s", "p", "b", "p", "b", "p"],
        vec!["p", "s", "s", "B", "p", "r", "r", "r", "r", "r", "p", "p", "s", "s", "s", "p", "b", "b", "b", "p"],
        vec!["p", "p", "p", "r", "p", "r", "p", "r", "p", "r", "p", "p", "p", "p", "p", "p", "p", "p", "p", "p"],
        vec!["p", "p", "p", "r", "r", "r", "r", "r", "r", "r", "p", "p", "p", "p", "p", "p", "p", "p", "p", "p"],
        vec!["p", "p", "p", "p", "p", "p", "p", "p", "p", "p", "p", "p", "p", "p", "p", "p", "p", "p", "p", "p"],
    ];
    Board::from(data)
}
