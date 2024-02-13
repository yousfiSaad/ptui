
# ptui - Prayer Terminal UI (or Progress Terminal UI)

## Introduction

ptui is a simple TUI that shows the remaiming time to the next prayer.


## Build

```
cargo build --release
```

## Input data
Data is fed to the executable through a data file containing line separated pairs of timestamps and labels.

### Example
`./data/prayer_times.txt` contains prayer time for casablanca morocco

## Usage
```
DATA_FILE="/path/to/data/file" ./target/release/ptui
```
The default value of data file is `./data/prayer_times.txt`

### keyboard shortcuts
- `t` : switch between time and remaining time to prayer
- `q` : quit

### Data Generation
- not yet published

