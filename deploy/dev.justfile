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

tauri-run:
    npm run tauri dev
