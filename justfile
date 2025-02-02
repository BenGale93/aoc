alias n := new

new year day:
    cargo new y{{year}}/d{{day}} --name y{{year}}-d{{day}}
    cd y{{year}}/d{{day}} && aoc download --day {{day}} --year {{year}}
    echo 'aoc_utils = { workspace = true }' >> y{{year}}/d{{day}}/Cargo.toml

lint day:
    cargo clippy --bin {{day}} -- -W clippy::nursery -W clippy::pedantic

alias t := test

test year day:
    cargo t -p y{{year}}-d{{day}}

alias s := submit

[no-cd]
submit part answer:
    day=$(basename $(pwd) | cut -c2-3) && year=$(basename $(dirname $(pwd)) | cut -c2-5) && aoc submit -d $day -y $year {{part}} {{answer}}

alias d := download

[no-cd]
download:
    day=$(basename $(pwd) | cut -c2-3) && year=$(basename $(dirname $(pwd)) | cut -c2-5) && aoc download -d $day -y $year -o

alias r1 := run1

[no-cd]
run1:
    cargo r --release

alias r2 := run2

[no-cd]
run2:
    cargo r --release -- -p

[no-cd]
done:
    git add -A
    day=$(basename $(pwd) | cut -c2-3) && year=$(basename $(dirname $(pwd)) | cut -c2-5) && git commit -m "Completed $year d$day"