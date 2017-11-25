#![feature(test)]
extern crate crisp;
use crisp::var::{BTreeSetVar, Variable, VarSet, VarId};
use crisp::propagate::{PropSet, AllDifferent};
use crisp::solve::Solver;

extern crate test;

type Var = BTreeSetVar<u8>;

#[test]
fn sudoku() {
    let mut var_set = VarSet::new();
    let mut var_ids = vec![];
    for row in 0..9 {
        var_ids.push(vec![]);
        for _ in 0..9 {
            var_ids[row].push(var_set.create_var(&[1, 2, 3, 4, 5, 6, 7, 8, 9]));
        }
    }

    let mut prop_set = PropSet::new();

    for row in 0..9 {
        let mut ids = vec![];
        for col in 0..9 {
            ids.push(var_ids[row][col]);
        }
        prop_set.add_propagator(AllDifferent::new(ids));
    }

    for col in 0..9 {
        let mut ids = vec![];
        for row in 0..9 {
            ids.push(var_ids[row][col]);
        }
        prop_set.add_propagator(AllDifferent::new(ids));
    }

    for &row in &[0, 3, 6] {
        for &col in &[0, 3, 6] {
            let mut ids = vec![];
            for row_offset in 0..3 {
                for col_offset in 0..3 {
                    ids.push(var_ids[row + row_offset][col + col_offset]);
                }
            }
            prop_set.add_propagator(AllDifferent::new(ids));
        }
    }

    var_set.set(var_ids[0][0], &5);
    var_set.set(var_ids[0][1], &3);
    var_set.set(var_ids[0][4], &7);
    var_set.set(var_ids[1][0], &6);
    var_set.set(var_ids[1][3], &1);
    var_set.set(var_ids[1][4], &9);
    var_set.set(var_ids[1][5], &5);
    var_set.set(var_ids[2][1], &9);
    var_set.set(var_ids[2][2], &8);
    var_set.set(var_ids[2][7], &6);
    var_set.set(var_ids[3][0], &8);
    var_set.set(var_ids[3][4], &6);
    var_set.set(var_ids[3][8], &3);
    var_set.set(var_ids[4][0], &4);
    var_set.set(var_ids[4][3], &8);
    var_set.set(var_ids[4][5], &3);
    var_set.set(var_ids[4][8], &1);
    var_set.set(var_ids[5][0], &7);
    var_set.set(var_ids[5][4], &2);
    var_set.set(var_ids[5][8], &6);
    var_set.set(var_ids[6][1], &6);
    var_set.set(var_ids[6][6], &2);
    var_set.set(var_ids[6][7], &8);
    var_set.set(var_ids[7][3], &4);
    var_set.set(var_ids[7][4], &1);
    var_set.set(var_ids[7][5], &9);
    var_set.set(var_ids[7][8], &5);
    var_set.set(var_ids[8][4], &8);
    var_set.set(var_ids[8][7], &7);
    var_set.set(var_ids[8][8], &9);

    let mut solver = Solver::new(var_set, prop_set);
    if let Some(var_set) = solver.next() {
        print_sudoku(&var_ids, &var_set);
    }
}

#[test]
fn sudoku2() {
    let mut var_set = VarSet::<Var>::new();
    let mut var_ids = vec![];
    for row in 0..9 {
        var_ids.push(vec![]);
        for _ in 0..9 {
            var_ids[row].push(var_set.create_var(&[1, 2, 3, 4, 5, 6, 7, 8, 9]));
        }
    }

    let mut prop_set = PropSet::new();

    for row in 0..9 {
        let mut ids = vec![];
        for col in 0..9 {
            ids.push(var_ids[row][col]);
        }
        prop_set.add_propagator(AllDifferent::new(ids));
    }

    for col in 0..9 {
        let mut ids = vec![];
        for row in 0..9 {
            ids.push(var_ids[row][col]);
        }
        prop_set.add_propagator(AllDifferent::new(ids));
    }

    for &row in &[0, 3, 6] {
        for &col in &[0, 3, 6] {
            let mut ids = vec![];
            for row_offset in 0..3 {
                for col_offset in 0..3 {
                    ids.push(var_ids[row + row_offset][col + col_offset]);
                }
            }
            prop_set.add_propagator(AllDifferent::new(ids));
        }
    }

    var_set.set(var_ids[0][0], &8);
    var_set.set(var_ids[0][1], &1);
    var_set.set(var_ids[0][8], &3);
    var_set.set(var_ids[1][3], &1);
    var_set.set(var_ids[1][5], &4);
    var_set.set(var_ids[2][1], &2);
    var_set.set(var_ids[2][2], &5);
    var_set.set(var_ids[2][3], &3);
    var_set.set(var_ids[2][8], &7);
    var_set.set(var_ids[3][2], &3);
    var_set.set(var_ids[3][3], &4);
    var_set.set(var_ids[3][4], &8);
    var_set.set(var_ids[3][7], &7);
    var_set.set(var_ids[5][2], &1);
    var_set.set(var_ids[5][5], &7);
    var_set.set(var_ids[5][6], &2);
    var_set.set(var_ids[5][8], &8);
    var_set.set(var_ids[6][0], &2);
    var_set.set(var_ids[6][1], &5);
    var_set.set(var_ids[6][6], &9);
    var_set.set(var_ids[7][3], &9);
    var_set.set(var_ids[7][8], &4);
    var_set.set(var_ids[8][2], &8);
    var_set.set(var_ids[8][7], &6);

    let mut solver = Solver::new(var_set, prop_set);
    if let Some(var_set) = solver.next() {
        print_sudoku(&var_ids, &var_set);
    }
}

fn print_sudoku(var_ids: &Vec<Vec<VarId>>, var_set: &VarSet<Var>) {
    println!();
    for row in 0..9 {
        for col in 0..3 {
            print!("{}", var_set.var(var_ids[row][col]).value().unwrap());
        }
        print!(" ");
        for col in 3..6 {
            print!("{}", var_set.var(var_ids[row][col]).value().unwrap());
        }
        print!(" ");
        for col in 6..9 {
            print!("{}", var_set.var(var_ids[row][col]).value().unwrap());
        }
        println!();
        if row == 2 || row == 5 {
            println!("         ");
        }
    }
    println!();
}
