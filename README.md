# UxPlay-TUI
A Terminal User Interface to manage the popular open iOS utility UxPlay
> [!NOTE]
> This project is a WIP, please treat with care. Things may break or work unexpectedly  
### AirPlay-Mirror and AirPlay-Audio server for Linux, macOS, and Unix (also runs on Windows).
## Features
Up-to-date visual implementation of the popular open iOS utility [UxPlay](https://github.com/FDH2/UxPlay)
- A TUI to use UxPlay in a more convenient way
- Works with all common **terminals** and via **SSH**
- Fully compatible with [iHub](https://github.com/iOpenInterconnect/ihub-cli)
- No loss of functionality: All UxPlay flags implemented
*Have a look at [#Highlights](https://github.com/FDH2/UxPlay#highlights) to get an image of how UxPlay works and what it does*
## Install
### Prerequisites
- A working installation of UxPlay (see [#packaging](https://github.com/FDH2/UxPlay#packaging-status-linux-and-bsd-distributions) or [#build](https://github.com/FDH2/UxPlay#building-uxplay-from-source) to build from source)
### Linux
1. Clone this repo and build it using:
    ```bash
    git clone --branch main --depth 1 https://github.com/iOpenInterconnect/UxPlay-TUI.git
    cd UxPlay-TUI
    cargo install --path .
    ```
</br>

2. If not done yet, add `~/.cargo/bin/` to PATH. If using Bash:
    ```bash
    echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
    source ~/.bashrc
    ```
3. Now you can run `uxplay-tui` in your terminal
### Windows
1. Install Linux
2. Follow the steps above
## Contributing
See [CONTRIBUTING.md](https://github.com/iOpenInterconnect/UxPlay-TUI/blob/main/CONTRIBUTING.md)
## Branch Overview
- Stable: [iOpenInterconnect/UxPlay-TUI/main](https://github.com/iOpenInterconnect/UxPlay-TUI/tree/main)
- Development: [iOpenInterconnect/UxPlay-TUI/dev](https://github.com/iOpenInterconnect/UxPlay-TUI/tree/dev)

## AI Contribution Disclosure
![Level 2](https://badgen.net/badge/AI%20Assistance/Level%202?color=cyan)
> [!NOTE]
> This project uses [Level 2 AI assistance](https://www.visidata.org/blog/2026/ai/) — a human wrote nearly all the code, with occasional copy-paste of small boilerplate snippets, regexes, or other minor incantations. The code was generated at the speed of human thought.

