alias n := new

new day year:
    cargo new y{{year}}/d{{day}} --name y{{year}}-d{{day}}
    cd y{{year}}/d{{day}} && aoc download --day {{day}} --year {{year}}
    echo 'aoc_utils = { workspace = true }' >> y{{year}}/d{{day}}/Cargo.toml

lint day:
    cargo clippy --bin {{day}} -- -W clippy::nursery -W clippy::pedantic

alias t := test

test day year:
    cargo t -p y{{year}}-d{{day}}