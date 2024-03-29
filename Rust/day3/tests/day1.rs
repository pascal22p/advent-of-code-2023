use day3::Symbol;


#[test]
fn part1_read_table() {
    let expected: Vec<Vec<char>> = vec![
        vec!['4','6','7','.','.','1','1','4','.','.'],
        vec!['.','.','.','*','.','.','.','.','.','.'],
        vec!['.','.','3','5','.','.','6','3','3','.'],
        vec!['.','.','.','.','.','.','#','.','.','.'],
        vec!['6','1','7','*','.','.','.','.','.','.'],
        vec!['.','.','.','.','.','+','.','5','8','.'],
        vec!['.','.','5','9','2','.','.','.','.','.'],
        vec!['.','.','.','.','.','.','7','5','5','.'],
        vec!['.','.','.','$','.','*','.','.','.','.'],
        vec!['.','6','6','4','.','5','9','8','.','.']
    ];

    assert_eq!(expected, day3::read_table("test-input"));
}

#[test]
fn part1_get_symbols_position() {
    let input: Vec<Vec<char>> = vec![
        vec!['4','6','7','.','.','1','1','4','.','.'],
        vec!['.','.','.','*','.','.','.','.','.','.'],
        vec!['.','.','3','5','.','.','6','3','3','.'],
        vec!['.','.','.','.','.','.','#','.','.','.'],
        vec!['6','1','7','*','.','.','.','.','.','.'],
        vec!['.','.','.','.','.','+','.','5','8','.'],
        vec!['.','.','5','9','2','.','.','.','.','.'],
        vec!['.','.','.','.','.','.','7','5','5','.'],
        vec!['.','.','.','$','.','*','.','.','.','.'],
        vec!['.','6','6','4','.','5','9','8','.','.']
    ];

    let expected: Vec<Symbol> = vec![
        Symbol { symbol: '*', row: 1, col: 3 }, 
        Symbol { symbol: '#', row: 3, col: 6 }, 
        Symbol { symbol: '*', row: 4, col: 3 }, 
        Symbol { symbol: '+', row: 5, col: 5 }, 
        Symbol { symbol: '$', row: 8, col: 3 }, 
        Symbol { symbol: '*', row: 8, col: 5 }
    ];

    assert_eq!(expected, day3::get_symbols_position(input));
}

#[test]
fn part1_read_and_delete_number() {
    let input: Vec<Vec<char>> = vec![
        vec!['4','6','7','.','.','1','1','4','.','.'],
        vec!['.','.','.','*','.','.','.','.','.','.'],
        vec!['.','.','3','5','.','.','6','3','3','.'],
        vec!['.','.','.','.','.','.','#','.','.','.'],
        vec!['6','1','7','*','.','.','.','.','.','.'],
        vec!['.','.','.','.','.','+','.','5','8','.'],
        vec!['.','.','5','9','2','.','.','.','.','.'],
        vec!['.','.','.','.','.','.','7','5','5','.'],
        vec!['.','.','.','$','.','*','.','.','.','.'],
        vec!['.','6','6','4','.','5','9','8','.','.']
    ];

    let expected: Vec<Vec<char>> = vec![
        vec!['.', '.', '.', '.', '.', '1', '1', '4', '.', '.'], 
        vec!['.', '.', '.', '*', '.', '.', '.', '.', '.', '.'], 
        vec!['.', '.', '3', '5', '.', '.', '6', '3', '3', '.'], 
        vec!['.', '.', '.', '.', '.', '.', '#', '.', '.', '.'], 
        vec!['6', '1', '7', '*', '.', '.', '.', '.', '.', '.'],
        vec!['.', '.', '.', '.', '.', '+', '.', '5', '8', '.'], 
        vec!['.', '.', '5', '9', '2', '.', '.', '.', '.', '.'], 
        vec!['.', '.', '.', '.', '.', '.', '7', '5', '5', '.'], 
        vec!['.', '.', '.', '$', '.', '*', '.', '.', '.', '.'], 
        vec!['.', '6', '6', '4', '.', '5', '9', '8', '.', '.']
    ];

    let input_copy = input.clone();

    assert_eq!(467, day3::read_and_delete_number(input_copy.clone(), 0, 2).0);
    assert_eq!(expected, day3::read_and_delete_number(input_copy, 0, 2).1);
}

#[test]
fn part1_get_neighbours() {
    let input: Vec<Vec<char>> = vec![
        vec!['4','6','7','.','.','1','1','4','.','.'],
        vec!['.','.','.','*','.','.','.','.','.','.'],
        vec!['.','.','3','5','.','.','6','3','3','.'],
        vec!['.','.','.','.','.','.','#','.','.','.'],
        vec!['6','1','7','*','.','.','.','.','.','.'],
        vec!['.','.','.','.','.','+','.','5','8','.'],
        vec!['.','.','5','9','2','.','.','.','.','.'],
        vec!['.','.','.','.','.','.','7','5','5','.'],
        vec!['.','.','.','$','.','*','.','.','.','.'],
        vec!['.','6','6','4','.','5','9','8','.','.']
    ];

    let symbols: Vec<Symbol> = vec![
        Symbol { symbol: '*', row: 1, col: 3 }, 
        Symbol { symbol: '#', row: 3, col: 6 }, 
        Symbol { symbol: '*', row: 4, col: 3 }, 
        Symbol { symbol: '+', row: 5, col: 5 }, 
        Symbol { symbol: '$', row: 8, col: 3 }, 
        Symbol { symbol: '*', row: 8, col: 5 }
    ];

    let expected: Vec<i32> = vec![
        467, 35, 633, 617, 592, 664, 755, 598
    ];

    let input_copy = input.clone();

    assert_eq!(expected, day3::get_neighbours(input_copy,symbols));
}

#[test]
fn part2_get_gears() {
    let input: Vec<Vec<char>> = vec![
        vec!['4','6','7','.','.','1','1','4','.','.'],
        vec!['.','.','.','*','.','.','.','.','.','.'],
        vec!['.','.','3','5','.','.','6','3','3','.'],
        vec!['.','.','.','.','.','.','#','.','.','.'],
        vec!['6','1','7','*','.','.','.','.','.','.'],
        vec!['.','.','.','.','.','+','.','5','8','.'],
        vec!['.','.','5','9','2','.','.','.','.','.'],
        vec!['.','.','.','.','.','.','7','5','5','.'],
        vec!['.','.','.','$','.','*','.','.','.','.'],
        vec!['.','6','6','4','.','5','9','8','.','.']
    ];

    let symbols: Vec<Symbol> = vec![
        Symbol { symbol: '*', row: 1, col: 3 }, 
        Symbol { symbol: '#', row: 3, col: 6 }, 
        Symbol { symbol: '*', row: 4, col: 3 }, 
        Symbol { symbol: '+', row: 5, col: 5 }, 
        Symbol { symbol: '$', row: 8, col: 3 }, 
        Symbol { symbol: '*', row: 8, col: 5 }
    ];

    let expected: Vec<i32> = vec![
        16345, 451490
    ];

    let input_copy = input.clone();

    assert_eq!(expected, day3::get_gears(input_copy,symbols));
}
