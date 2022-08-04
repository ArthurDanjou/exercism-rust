pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let valid_range = |x: usize, max_count: usize| {
        (x.saturating_sub(1))..(x + 1).min(max_count.saturating_sub(1)).saturating_add(1)
    };

    let find_mines = |r: usize, c: usize| -> usize {
        valid_range(r, minefield.len())
            .map(|r1| {
                minefield
                    .get(r1 as usize)
                    .map(|&row| {
                        //println!("{} for {} {}", r1, r, row);
                        row.get(valid_range(c, row.len()))
                            .map(|ns| ns.chars().filter(|c| c == &'*').count())
                            .unwrap_or(0)
                    })
                    .unwrap_or(0)
            })
            .sum::<usize>()
    };

    minefield
        .iter()
        .enumerate()
        .map(|(row, &rowstr)| {
            rowstr
                .chars()
                .enumerate()
                .map(|(col, ch)| match ch {
                    '*' => '*',
                    _ => match find_mines(row, col) {
                        0 => ' ',
                        k => char::from_digit(k as u32, 10).unwrap(),
                    },
                })
                .collect::<String>()
        })
        .collect::<Vec<String>>()
}
