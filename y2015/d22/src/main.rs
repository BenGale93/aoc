use std::collections::VecDeque;

use aoc_utils::Cli;

#[derive(Debug, Clone, Copy)]
struct Player {
    hp: isize,
    armor: isize,
    mana: isize,
}

#[derive(Debug, Clone, Copy)]
struct Boss {
    hp: isize,
    damage: isize,
}

#[derive(Debug, Clone, Copy)]
struct ActiveEffects {
    shield: (isize, bool),
    poison: (isize, bool),
    recharge: (isize, bool),
}

impl ActiveEffects {
    fn new() -> Self {
        Self {
            shield: (0, false),
            poison: (0, false),
            recharge: (0, false),
        }
    }
}

fn run_effect(player: &mut Player, boss: &mut Boss, effects: &mut ActiveEffects) {
    if effects.shield.1 {
        player.armor = 7;
        effects.shield.0 -= 1;
        if effects.shield.0 <= 0 {
            player.armor = 0;
            effects.shield.1 = false;
        }
    }

    if effects.poison.1 {
        boss.hp -= 3;
        effects.poison.0 -= 1;
        if effects.poison.0 <= 0 {
            effects.poison.1 = false;
        }
    }

    if effects.recharge.1 {
        player.mana += 101;
        effects.recharge.0 -= 1;
        if effects.recharge.0 <= 0 {
            effects.recharge.1 = false;
        }
    }
}

fn attack_player(player: &mut Player, boss: &mut Boss) {
    let mut damage = boss.damage - player.armor;
    if damage <= 0 {
        damage = 1;
    }
    player.hp -= damage;
}

fn magic_missile(boss: &mut Boss) {
    boss.hp -= 4;
}

fn drain(player: &mut Player, boss: &mut Boss) {
    boss.hp -= 2;
    player.hp += 2;
}

#[derive(Debug, Clone, Copy)]
enum Spell {
    MagicMissile,
    Drain,
    Shield,
    Poison,
    Recharge,
}

impl Spell {
    fn get_spell(index: usize) -> Self {
        match index {
            0 => Self::MagicMissile,
            1 => Self::Drain,
            2 => Self::Shield,
            3 => Self::Poison,
            4 => Self::Recharge,
            _ => panic!(),
        }
    }

    fn cast_spell_effect(self, player: &mut Player, boss: &mut Boss, effects: &mut ActiveEffects) {
        match self {
            Spell::MagicMissile => magic_missile(boss),
            Spell::Drain => drain(player, boss),
            Spell::Shield => effects.shield = (6, true),
            Spell::Poison => effects.poison = (6, true),
            Spell::Recharge => effects.recharge = (5, true),
        };
    }

    fn cost(&self) -> isize {
        match *self {
            Spell::MagicMissile => 53,
            Spell::Drain => 73,
            Spell::Shield => 113,
            Spell::Poison => 173,
            Spell::Recharge => 229,
        }
    }
}

enum GameState {
    Won,
    InProgress,
    Loss,
}

struct Game {
    player: Player,
    boss: Boss,
    effects: ActiveEffects,
    mana_cost: isize,
    next_spell: Spell,
}

impl Game {
    fn run(&mut self, hard: bool) -> GameState {
        if hard {
            self.player.hp -= 1;
            if self.player.hp <= 0 {
                return GameState::Loss;
            }
        }

        run_effect(&mut self.player, &mut self.boss, &mut self.effects);
        if self.boss.hp <= 0 {
            return GameState::Won;
        }

        // Check if spell is valid
        match self.next_spell {
            Spell::MagicMissile => (),
            Spell::Drain => (),
            Spell::Shield => {
                if self.effects.shield.1 {
                    return GameState::Loss;
                }
            }
            Spell::Poison => {
                if self.effects.poison.1 {
                    return GameState::Loss;
                }
            }
            Spell::Recharge => {
                if self.effects.recharge.1 {
                    return GameState::Loss;
                }
            }
        };

        let cost = self.next_spell.cost();

        // Check if the player has enough mana to cast
        if cost > self.player.mana {
            return GameState::Loss;
        }

        self.mana_cost += cost;
        self.player.mana -= cost;

        self.next_spell
            .cast_spell_effect(&mut self.player, &mut self.boss, &mut self.effects);

        if self.boss.hp <= 0 {
            return GameState::Won;
        }

        run_effect(&mut self.player, &mut self.boss, &mut self.effects);
        if self.boss.hp <= 0 {
            return GameState::Won;
        }

        attack_player(&mut self.player, &mut self.boss);
        if self.player.hp <= 0 {
            return GameState::Loss;
        }

        GameState::InProgress
    }
}

fn create_setup() -> (Player, Boss, ActiveEffects) {
    let player = Player {
        hp: 50,
        armor: 0,
        mana: 500,
    };

    let boss = Boss { hp: 71, damage: 10 };

    let effects = ActiveEffects::new();

    (player, boss, effects)
}

fn create_games(game: Game, games: &mut VecDeque<Game>) {
    for i in 0..5 {
        let game = Game {
            player: game.player,
            boss: game.boss,
            effects: game.effects,
            mana_cost: game.mana_cost,
            next_spell: Spell::get_spell(i),
        };
        games.push_back(game);
    }
}

fn main() {
    let hard = Cli::parse_args().part_two;
    let (player, boss, effects) = create_setup();

    let initial_game = Game {
        player,
        boss,
        effects,
        mana_cost: 0,
        next_spell: Spell::Poison, // random spell that will get overridden.
    };

    let mut all_games = VecDeque::new();
    create_games(initial_game, &mut all_games);

    let mut lowest_cost = isize::MAX;

    while let Some(mut active_game) = all_games.pop_front() {
        let result = active_game.run(hard);

        match result {
            GameState::Won => {
                if active_game.mana_cost < lowest_cost {
                    lowest_cost = active_game.mana_cost;
                }
            }
            GameState::InProgress => {
                if active_game.mana_cost < lowest_cost {
                    create_games(active_game, &mut all_games);
                }
            }
            GameState::Loss => (),
        };
    }

    println!("{lowest_cost}");
}
