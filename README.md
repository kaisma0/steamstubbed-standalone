# Steam Stubbed

Plug n Play DLL that allows to patch SteamStub on the fly. Can be loaded by simply renaming the built DLL to `version.dll` or `winmm.dll` and placing it in the game's executable directory. It acts as a standalone proxy DLL, seamlessly forwarding requests to the original system DLLs while patching SteamStub in the background.

Tested against various basic SteamStub games and also works in games where [Steamless](https://github.com/atom0s/Steamless) fails to execute, such as Hi-Fi Rush.

## Download

Grab the latest build from GitHub:

- **Stable releases**: [Releases](https://github.com/kaisma0/steamstubbed-standalone/releases)
- **Nightly builds**: [Actions](https://github.com/kaisma0/steamstubbed-standalone/actions) (artifacts from the latest workflow run)

## Building from Source

```bash
cargo build --release
```

The output DLL will be in `target/release/steam_stubbed.dll`. Rename it to `version.dll` or `winmm.dll` and place it alongside the game executable.

## Disclaimer

This project is for educational and research purposes only. Use responsibly and respect software licenses.

## Credits

- [denuvosanctuary/steam-stubbed](https://github.com/denuvosanctuary/steam-stubbed): For the original SteamStub patching logic and core payload.
- [denuvosanctuary/coldloader-proxy](https://github.com/denuvosanctuary/coldloader-proxy): For the DLL proxying implementation.
