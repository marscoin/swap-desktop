# Mars Swap Desktop

A desktop app for trading Bitcoin and Marscoin through atomic swaps, with no exchange and no custody. It looks like [swaps.marscoin.org](https://swaps.marscoin.org) and does what a web page can't do safely: it holds your keys, checks the seller's lock on your own copy of the Marscoin chain, and finishes or refunds every swap by itself.

> **Status: milestone 1 of 5.**
> - **Done:** the app starts its own swap engine, a Marscoin node on your computer, and shows the live market. Tested on macOS (Apple Silicon); Windows and Linux build in CI.
> - **Not yet:** wallets and swapping. Until then, use [Electrum-Mars](https://github.com/marscoin/electrum-mars/releases/latest) to swap, and [swaps.marscoin.org](https://swaps.marscoin.org) to watch the market.
>
> See [ARCHITECTURE.md](ARCHITECTURE.md) for the design and milestones.

## Try a test build

Every push builds unsigned test versions for macOS (Apple Silicon), Windows and Linux. Download them from the newest successful run under [Actions](https://github.com/marscoin/swap-desktop/actions/workflows/build.yml), in the run's *Artifacts* section.

These test builds aren't signed yet. On macOS, open the app once, then allow it under **System Settings → Privacy & Security → Open Anyway**. The first start downloads and checks Marscoin's block headers, which takes a while.

## How it fits together

- **Engine:** [atomicswapd](https://github.com/marscoin/atomicswapd), the Electrum-Mars swap engine packaged for servers, here running locally in desktop mode.
- **Interface:** the same React app as swaps.marscoin.org, with actions enabled.
- **Shell:** [Tauri](https://tauri.app), a small native window around the system web view.

## License

MIT
