create:
    npm create tauri-app@latest
tauri-dependencies:
    sudo apt install libwebkit2gtk-4.1-dev \
      build-essential \
      curl \
      wget \
      file \
      libxdo-dev \
      libssl-dev \
      libayatana-appindicator3-dev \
      librsvg2-dev

    sudo apt install pkg-config
    sudo apt install libglib2.0-dev libgtk-3-dev

tauri-run:
    npm run tauri dev
