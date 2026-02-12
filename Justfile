build:
    cargo build

clean:
    just tui/clean
    just iOS/clean
    cargo clean

tui: 
    just tui/run

app:
    just iOS/dev

test:
    cargo test
