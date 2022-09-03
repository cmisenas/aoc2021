use std::collections::HashMap;
use std::collections::HashSet;

pub fn main() {
    let player1_pos = 3;
    let player2_pos = 10;
    let answer1 = solve1(player1_pos, player2_pos);
    let answer2 = solve2(player1_pos, player2_pos);
    println!("Day 21 answers");
    println!("Answer 1 {}", answer1);
    println!("Answer 2 {}", answer2);
}

fn solve1(mut player1_pos: usize, mut player2_pos: usize) -> usize {
    let mut det_dice = 0;
    let mut player1_score = 0;
    let mut player2_score = 0;
    let mut i = 0;

    loop {
        if player1_score >= 1000 || player2_score >= 1000 {
            break;
        }
        let mut curr_score = 0;
        for _ in 0..3 {
            det_dice = match (det_dice + 1) > 100 {
                true => (det_dice + 1) % 100,
                _ => det_dice + 1,
            };
            curr_score += det_dice;
        }
        if i % 2 == 0 {
            // Player 1
            curr_score = (player1_pos + curr_score) % 10;
            if curr_score == 0 {
                curr_score = 10;
            }
            player1_score += curr_score;
            player1_pos = curr_score;
        } else {
            // Player 2
            curr_score = (player2_pos + curr_score) % 10;
            if curr_score == 0 {
                curr_score = 10;
            }
            player2_score += curr_score;
            player2_pos = curr_score;
        }

        i += 1;
    }

    let loser = match player1_score < player2_score {
        true => player1_score,
        _ => player2_score,
    };

    loser * i * 3
}

fn solve2(player1_pos: usize, player2_pos: usize) -> usize {
    // 3 * 3 * 3 = 27
    let possible_rolls = vec![
        vec![1, 1, 1], // 3
        vec![1, 1, 2], // 4
        vec![1, 1, 3], // 5
        vec![1, 2, 1], // 4
        vec![1, 2, 2], // 5
        vec![1, 2, 3], // 6
        vec![1, 3, 1], // 5
        vec![1, 3, 2], // 6
        vec![1, 3, 3], // 7
        vec![2, 1, 1], // 4
        vec![2, 1, 2], // 5
        vec![2, 1, 3], // 6
        vec![2, 2, 1], // 5
        vec![2, 2, 2], // 6
        vec![2, 2, 3], // 7
        vec![2, 3, 1], // 6
        vec![2, 3, 2], // 7
        vec![2, 3, 3], // 8
        vec![3, 1, 1], // 5
        vec![3, 1, 2], // 6
        vec![3, 1, 3], // 7
        vec![3, 2, 1], // 6
        vec![3, 2, 2], // 7
        vec![3, 2, 3], // 8
        vec![3, 3, 1], // 7
        vec![3, 3, 2], // 8
        vec![3, 3, 3], // 9
    ];
    let possible_rolls = vec![
        vec![1, 1, 1], // 3
        vec![1, 1, 2], // 4
        vec![1, 2, 1], // 4
        vec![2, 1, 1], // 4
        vec![1, 1, 3], // 5
        vec![1, 2, 2], // 5
        vec![1, 3, 1], // 5
        vec![2, 1, 2], // 5
        vec![2, 2, 1], // 5
        vec![3, 1, 1], // 5
        vec![1, 2, 3], // 6
        vec![1, 3, 2], // 6
        vec![2, 1, 3], // 6
        vec![2, 2, 2], // 6
        vec![2, 3, 1], // 6
        vec![3, 1, 2], // 6
        vec![3, 2, 1], // 6
        vec![1, 3, 3], // 7
        vec![2, 2, 3], // 7
        vec![2, 3, 2], // 7
        vec![3, 1, 3], // 7
        vec![3, 2, 2], // 7
        vec![3, 3, 1], // 7
        vec![2, 3, 3], // 8
        vec![3, 2, 3], // 8
        vec![3, 3, 2], // 8
        vec![3, 3, 3], // 9
    ];

    // Keep track of
    // - player 1 pos
    // - player 2 pos
    // By keeping track of
    // - how many multiverses have pos of x (1 - 10)
    // -
    0
}
