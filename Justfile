build:
    cargo build

clean:
    just tui/clean
    just iOS/clean

tui: 
    just tui/run

app:
    just iOS/dev

