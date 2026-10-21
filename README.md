# Mars Swap Desktop

A desktop app for trading Bitcoin and Marscoin through atomic swaps, with no exchange and no custody. It looks like [swaps.marscoin.org](https://swaps.marscoin.org) and does what a web page can't do safely: it holds your keys, checks the seller's lock on your own copy of the Marscoin chain, and finishes or refunds every swap by itself.

> **Status: just started.** See [ARCHITECTURE.md](ARCHITECTURE.md) for the design and milestones. Until there's a release, use [Electrum-Mars](https://github.com/marscoin/electrum-mars/releases/latest) to swap, and [swaps.marscoin.org](https://swaps.marscoin.org) to watch the market.

## How it fits together

- **Engine:** [atomicswapd](https://github.com/marscoin/atomicswapd), the Electrum-Mars swap engine packaged for servers, here running locally in desktop mode.
- **Interface:** the same React app as swaps.marscoin.org, with actions enabled.
- **Shell:** [Tauri](https://tauri.app), a small native window around the system web view.

## License

MIT
