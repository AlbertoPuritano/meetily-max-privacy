# Meetily – Max Privacy Fork

A desktop app that records meetings and transcribes them to text on your own computer, with optional on-device summaries.

This is a fork of [Meetily](https://github.com/Zackriya-Solutions/meetily) by Zackriya Solutions. For the full-featured original, its documentation, and official releases, go there.

## Why this fork exists

The original Meetily is privacy-focused, but it still talks to the internet: usage analytics, an auto-updater, cloud AI providers, and various external links. This fork strips all of that out so the app is as barebones as possible. Once the AI models are downloaded, it works fully offline and nothing about your meetings leaves your machine.

## What has been removed

- **Usage analytics:** PostHog telemetry and its consent settings.
- **Auto-updater:** update checks against GitHub releases.
- **Cloud summary providers:** OpenAI, Anthropic (Claude), Groq, OpenRouter, and custom OpenAI-compatible servers, along with all API-key storage.
- **Remote content:** the seasonal sidebar icon and the editor's emoji picker, which loaded data from a CDN.
- **External links:** the feedback page (GitHub, Discord, socials), website and contact links, and the "Download Ollama" link.
- **Legacy backend calls:** leftover requests to the old local backend server.
- **GitHub pipelines:** CI workflows, issue templates, and release/update scripts.

To catch anything missed, the app's content security policy now only lets the interface talk to the app itself.

## What still uses the network

- **One-time model downloads:** on first launch, the transcription model (Parakeet or Whisper) and the built-in summary model are downloaded from Hugging Face. After that, no connection is needed.
- **Ollama (optional):** you can use an Ollama server for summaries, but only one running on this computer (`localhost`, `127.0.0.1` or `::1`). Remote addresses are rejected. Downloading a model through Ollama uses Ollama's own registry.
- **Building:** compiling the app downloads dependencies (npm, crates.io) and an FFmpeg binary. The finished app makes none of these requests.

## Installation (build it yourself)

No prebuilt downloads are provided. I'm not a verified Apple developer, so I can't sign or notarize builds for distribution, and you have to build the app on your own Mac. These instructions are for macOS on Apple Silicon, the only platform tested.

### 1. Install the tools

Install [Homebrew](https://brew.sh) and the Xcode Command Line Tools (`xcode-select --install`), then:

```bash
brew install node pnpm cmake rustup
export PATH="$(brew --prefix rustup)/bin:$PATH"
rustup toolchain install 1.99.1 --target aarch64-apple-darwin
```

Rust 1.99.1 is required: Rust 1.100 and newer break the build of two pinned dependencies (`tauri-build` 2.6.1 and `ort-sys` 2.0.0-rc.10). The commands below select 1.99.1 with `RUSTUP_TOOLCHAIN`, so any other Rust installation is left untouched.

### 2. Get the code

```bash
git clone https://github.com/AlbertoPuritano/meetily-max-privacy.git
cd meetily-max-privacy
```

### 3. Build the summary sidecar

From the repository root:

```bash
export PATH="$(brew --prefix rustup)/bin:$PATH" RUSTUP_TOOLCHAIN=1.99.1
CMAKE_DISABLE_FIND_PACKAGE_OpenSSL=TRUE cargo build --release -p llama-helper --features metal
mkdir -p frontend/src-tauri/binaries
cp target/release/llama-helper frontend/src-tauri/binaries/llama-helper-aarch64-apple-darwin
```

### 4. Build the app

```bash
cd frontend
pnpm install --frozen-lockfile
pnpm tauri build --target aarch64-apple-darwin
```

The first build takes several minutes. When it finishes, you'll find:

- the app at `target/aarch64-apple-darwin/release/bundle/macos/meetily.app`
- an installer at `target/aarch64-apple-darwin/release/bundle/dmg/meetily_0.5.0_aarch64.dmg`

### 5. Install and run

Open the `.dmg` and drag Meetily into Applications, or copy `meetily.app` there directly.

The app is only ad-hoc signed. If macOS blocks it (likely if you copy it to another Mac), right-click the app and choose **Open**, or clear the quarantine flag:

```bash
xattr -dr com.apple.quarantine /Applications/meetily.app
```

On first launch, Meetily downloads its models and asks for Microphone and Audio Capture permission (Audio Capture lets it record the other participants through your system audio).

## License

MIT, same as the original. See [LICENSE.md](LICENSE.md). All credit for Meetily goes to [Zackriya Solutions](https://github.com/Zackriya-Solutions/meetily).
