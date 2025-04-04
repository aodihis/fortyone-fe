# Fortyone-FE

A 41-rummy card multiplayer game.

This is the frontend for the backend available at [fortyone-be](https://github.com/aodihis/fortyone-be).

![Rummy Game](https://raw.githubusercontent.com/aodihis/fortyone-fe/main/asset/thumbnail.gif)

Built using Yew and Rust.

## Setup and Run

1. **Set the API URL:**
    - For Windows:
      ```powershell
      \$env\:API_URL = "http://127.0.0.1:3000"
      ```
    - For Unix-based systems (Linux, macOS):
      ```bash
      export API_URL="http://127.0.0.1:3000"
      ```

2. **Run using Trunk:**
    - Development mode:
      ```bash
      trunk serve
      ```
    - Release mode:
      ```bash
      trunk serve --release
      ```

## Credits

Thanks to Sharm for the rummy card design available at [OpenGameArt](https://opengameart.org/content/playing-cards).
