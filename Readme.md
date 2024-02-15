
# ptui - Prayer Terminal UI (or Progress Terminal UI)

## Introduction

ptui is a simple TUI that shows the remaiming time for the next prayer.


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
- `t` : switch between time and remaining time for prayer
- `f` : switch between next prayer time and fasting end time (Maghrib)
- `q` : quit

### Data Generation
You can use [generate-prayer-time](https://github.com/yousfiSaad/generate-prayer-time) to generate data according to your location.

## Screenshots

Screenshots taken in small and large tmux panes 

![Small pane remaining time](https://raw.githubusercontent.com/yousfiSaad/ptui/main/screenshots/Small%20pane%20remaining%20time.png "Small pane remaining time")

![Small pane prayer time](https://raw.githubusercontent.com/yousfiSaad/ptui/main/screenshots/Small%20pane%20prayer%20time.png "Small pane prayer time")

![Large pane prayer time](https://raw.githubusercontent.com/yousfiSaad/ptui/main/screenshots/Large%20pane%20prayer%20time.png "Large pane prayer time")

![Large pane remaining time](https://raw.githubusercontent.com/yousfiSaad/ptui/main/screenshots/Large%20pane%20remaining%20time.png "Large pane remaining time")


