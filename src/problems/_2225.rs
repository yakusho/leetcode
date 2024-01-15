use std::collections::BTreeMap;

struct Player {
    wins: i32,
    losses: i32,
}

struct Solution;

impl Solution {
    pub fn find_winners(matches: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut players_map: BTreeMap<i32, Player> = BTreeMap::new();

        matches.iter().for_each(|m| {
            let winner = m.get(0).expect("Couldn't get winner from Vec");
            let loser = m.get(1).expect("Couldn't get loser from Vec");

            players_map
                .entry(*winner)
                .or_insert(Player { wins: 0, losses: 0 })
                .wins += 1;

            players_map
                .entry(*loser)
                .or_insert(Player { wins: 0, losses: 0 })
                .losses += 1;
        });


        let players_with_no_losses: Vec<i32> = players_map
            .iter()
            .filter(|&(_, player)| player.losses == 0)
            .map(|(&key, _)| key)
            .collect();

        let players_with_one_loss: Vec<i32> = players_map
            .iter()
            .filter(|&(_, player)| player.losses == 1)
            .map(|(&key, _)| key)
            .collect();

        return vec![players_with_no_losses, players_with_one_loss];
    }
}

#[test]
fn test() {
    let t1 = vec![
        vec![1, 3],
        vec![2, 3],
        vec![3, 6],
        vec![5, 6],
        vec![5, 7],
        vec![4, 5],
        vec![4, 8],
        vec![4, 9],
        vec![10, 4],
        vec![10, 9],
    ];

    assert_eq!(
        Solution::find_winners(t1),
        vec![vec![1, 2, 10], vec![4, 5, 7, 8]]
    );
}
