#[macro_use]
extern crate itertools;

extern crate crisp;
use crisp::var::{BTreeSetVar, Variable, VarSet, VarId};
use crisp::Model;

type Value = u8;
type Var = BTreeSetVar<u8>;
type Board = [[Value; 9]; 9];

fn build_model(board: &Board) -> (Model<Var>, Vec<Vec<VarId>>) {
    let mut model = Model::<Var>::new();
    let var_id_matrix = model.create_var_matrix(&[1, 2, 3, 4, 5, 6, 7, 8, 9], 9, 9);

    for r in 0..9 {
        let row = &var_id_matrix[r];
        model.all_different(row);
    }

    for c in 0..9 {
        let col = var_id_matrix.iter().map(|row| row[c]);
        model.all_different(col);
    }

    for (&r, &c) in iproduct!(&[0, 3, 6], &[0, 3, 6]) {
        let block = (0..9).map(|n| var_id_matrix[r + n / 3][c + n % 3]);
        model.all_different(block);
    }

    for r in 0..9 {
        for c in 0..9 {
            if board[r][c] != 0 {
                model.set(var_id_matrix[r][c], &board[r][c]);
            }
        }
    }

    (model, var_id_matrix)
}

fn verify_solution(var_ids: &Vec<Vec<VarId>>, solution: &VarSet<Var>, expected: &Board) {
    for r in 0..9 {
        for c in 0..9 {
            assert_eq!(solution.var(var_ids[r][c]).value().unwrap(), &expected[r][c]);
        }
    }
}

fn test_sudoku(board: &Board, expected: &Board) {
    let (model, var_id_matrix) = build_model(&board);
    let mut solver = model.solve();
    let solution = solver.next();
    assert!(solution.is_some());
    assert!(solver.next().is_none());
    verify_solution(&var_id_matrix, &solution.unwrap(), &expected);
}

#[test]
fn sudoku1() {
    let board = [
        [ 0, 0, 0, 2, 6, 0, 7, 0, 1 ],
        [ 6, 8, 0, 0, 7, 0, 0, 9, 0 ],
        [ 1, 9, 0, 0, 0, 4, 5, 0, 0 ],
        [ 8, 2, 0, 1, 0, 0, 0, 4, 0 ],
        [ 0, 0, 4, 6, 0, 2, 9, 0, 0 ],
        [ 0, 5, 0, 0, 0, 3, 0, 2, 8 ],
        [ 0, 0, 9, 3, 0, 0, 0, 7, 4 ],
        [ 0, 4, 0, 0, 5, 0, 0, 3, 6 ],
        [ 7, 0, 3, 0, 1, 8, 0, 0, 0 ],
    ];
    let expected = [
        [ 4, 3, 5, 2, 6, 9, 7, 8, 1 ],
        [ 6, 8, 2, 5, 7, 1, 4, 9, 3 ],
        [ 1, 9, 7, 8, 3, 4, 5, 6, 2 ],
        [ 8, 2, 6, 1, 9, 5, 3, 4, 7 ],
        [ 3, 7, 4, 6, 8, 2, 9, 1, 5 ],
        [ 9, 5, 1, 7, 4, 3, 6, 2, 8 ],
        [ 5, 1, 9, 3, 2, 6, 8, 7, 4 ],
        [ 2, 4, 8, 9, 5, 7, 1, 3, 6 ],
        [ 7, 6, 3, 4, 1, 8, 2, 5, 9 ],
    ];
    test_sudoku(&board, &expected);
}
