use aoc_utils::Cli;
use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
struct Stats {
    hp: isize,
    damage: isize,
    armor: isize,
}

fn fight(mut player: Stats, mut boss: Stats) -> bool {
    loop {
        let mut damage = player.damage - boss.armor;
        if damage <= 0 {
            damage = 1;
        }
        boss.hp -= damage;

        if boss.hp <= 0 {
            break;
        }

        let mut damage = boss.damage - player.armor;
        if damage <= 0 {
            damage = 1;
        }
        player.hp -= damage;
        if player.hp <= 0 {
            break;
        }
    }

    player.hp > 0
}

#[derive(Debug, Clone, Copy)]
struct Item {
    cost: isize,
    damage: isize,
    armor: isize,
}

fn create_player(items: &[Item]) -> (isize, Stats) {
    let player = Stats {
        hp: 100,
        damage: items.iter().map(|i| i.damage).sum(),
        armor: items.iter().map(|i| i.armor).sum(),
    };

    (items.iter().map(|i| i.cost).sum(), player)
}

fn create_setups() -> Vec<Vec<Item>> {
    let mut setups = vec![];
    for weapon in WEAPONS {
        for armor in ARMOR {
            setups.push(vec![weapon, armor]);
            for ring in RINGS {
                setups.push(vec![weapon, armor, ring]);
            }
            for (ring1, ring2) in RINGS.iter().tuple_combinations() {
                setups.push(vec![weapon, armor, *ring1, *ring2]);
            }
        }
    }
    setups
}

fn main() {
    let boss = Stats {
        hp: 109,
        damage: 8,
        armor: 2,
    };

    let mut setups = create_setups();
    let part_two = Cli::parse_args().part_two;
    setups.sort_by_key(|s| s.iter().map(|i| i.cost).sum::<isize>());

    if part_two {
        setups.reverse();
    }

    for setup in setups {
        let (cost, player) = create_player(&setup);
        let result = fight(player, boss);
        if result != part_two {
            println!("{cost}");
            break;
        }
    }
}

const WEAPONS: [Item; 5] = [
    Item {
        cost: 8,
        damage: 4,
        armor: 0,
    },
    Item {
        cost: 10,
        damage: 5,
        armor: 0,
    },
    Item {
        cost: 25,
        damage: 6,
        armor: 0,
    },
    Item {
        cost: 40,
        damage: 7,
        armor: 0,
    },
    Item {
        cost: 74,
        damage: 8,
        armor: 0,
    },
];

const ARMOR: [Item; 6] = [
    Item {
        cost: 0,
        damage: 0,
        armor: 0,
    },
    Item {
        cost: 13,
        damage: 0,
        armor: 1,
    },
    Item {
        cost: 31,
        damage: 0,
        armor: 2,
    },
    Item {
        cost: 53,
        damage: 0,
        armor: 3,
    },
    Item {
        cost: 75,
        damage: 0,
        armor: 4,
    },
    Item {
        cost: 102,
        damage: 0,
        armor: 5,
    },
];

const RINGS: [Item; 6] = [
    Item {
        cost: 25,
        damage: 1,
        armor: 0,
    },
    Item {
        cost: 50,
        damage: 2,
        armor: 0,
    },
    Item {
        cost: 100,
        damage: 3,
        armor: 0,
    },
    Item {
        cost: 20,
        damage: 0,
        armor: 1,
    },
    Item {
        cost: 40,
        damage: 0,
        armor: 2,
    },
    Item {
        cost: 80,
        damage: 0,
        armor: 3,
    },
];
