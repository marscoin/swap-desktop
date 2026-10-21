# Architecture

Mars Swap Desktop is a desktop wallet for BTC ⇄ MARS atomic swaps. The interface is the [swaps.marscoin.org](https://swaps.marscoin.org) dashboard, plus everything a web page can't safely do:
- hold the buyer's keys
- verify the seller's lock on the user's own copy of the Marscoin chain
- follow a swap through to the claim or the refund, hours later if necessary

## One engine, one interface

```
┌───────────────────────── Mars Swap (Tauri) ──────────────────────────┐
│                                                                      │
│  window (system web view)                                            │
│    the swaps.marscoin.org React app, with actions enabled            │
│            │  http://127.0.0.1:<port>, header X-Swap-Token           │
│            ▼                                                         │
│  atomicswapd --desktop        (bundled binary, started by the shell) │
│    Electrum-Mars swap engine: wallet, swap worker, Auto-Maker        │
│    read API (same as the server) + write API (desktop only)          │
│                                                                      │
└──────────────────────────────────────────────────────────────────────┘
           │                                     │
   Marscoin relays (ElectrumX)          mempool.space (Bitcoin)
```

- **The engine is [atomicswapd](https://github.com/marscoin/atomicswapd),** which runs the Electrum-Mars swap code. The desktop app bundles it as a standalone binary (PyInstaller, the same toolchain as the Electrum-Mars releases) and runs it as a background process. No swap logic is written twice. A bug fixed in Electrum-Mars is fixed here.
- **The interface is atomicswapd's React app.** On the public server it is read-only. In desktop mode it shows the same screens, plus actions:
  - create or restore a wallet
  - accept an offer
  - pay with the verified address and QR code
  - watch a swap claim itself
  - refund
  - run the Auto-Maker
- **The shell is [Tauri](https://tauri.app).** It uses the operating system's web view instead of shipping Chromium: an app of about 10 MB plus the engine, and less code next to the keys. The shell only:
  - starts and stops the engine
  - generates the session token
  - opens the window at the engine's local address

## Security model

The keys live in the engine's wallet file, encrypted with the user's password. Nothing else on the machine should be able to make the engine move money.

- **Local only.** The engine binds to `127.0.0.1` on a random free port. It is never reachable from the network.
- **Per-launch token.** The shell generates 32 random bytes at every launch and passes them to the engine (by environment, not on the command line). The window opens at `http://127.0.0.1:<port>/#token=…`; the fragment is never sent over HTTP.
  1. The interface trades the token once for a session cookie: `POST /api/session` with the `X-Swap-Token` header. The cookie is `HttpOnly` and `SameSite=Strict`.
  2. In desktop mode **every** API request needs that session, reads included, because the snapshot contains your balance and addresses.
  3. Requests that move money also need the token header.
  4. The engine sends no CORS headers in desktop mode and refuses preflights. A web page you visit can't read responses from it, can't send it the token header, and doesn't get the strict cookie.
- **No DNS rebinding.** The engine rejects requests whose `Host` isn't `127.0.0.1:<port>`.
- **No secrets in the UI.** The seed is shown once at wallet creation, in the window, and never stored by the interface. No API returns keys or swap secrets.
- **Signed builds.**
  - macOS: Developer ID signing and notarization.
  - Windows: Authenticode signing.
  - Updates are signed with Tauri's updater key.

  Until those certificates exist, builds are unsigned and labelled as such.

## Swap flows

**Buying MARS (taker).** These are the protections from Electrum-Mars 4.4.2+, unchanged:
1. Pick an offer.
2. The engine checks the refund times and accepts the offer.
3. It waits for the seller's MARS lock and verifies it with an SPV proof.
4. Only then does the window show the BTC address and QR code, with the exact amount.
5. The engine watches Bitcoin, extracts the secret when the seller claims, and claims the MARS.

If anything stalls, the refund opens after 24 Bitcoin blocks and the app does it, or offers it.

**Selling MARS (maker).** The Auto-Maker with atomicswapd's limits:
- an exposure cap
- one swap per buyer
- no guessed prices
- no late claims

Controls: on/off, spread, sizes, payout address.

## Repository layout

```
src-tauri/        Rust shell: engine lifecycle, token, window
sidecar/          PyInstaller build of atomicswapd for each platform
.github/workflows CI: build the engine and the app for macOS, Windows, Linux
```

The interface and the engine's desktop mode live in the atomicswapd repository, so the server dashboard and the desktop app never drift apart.

## Milestones

1. **Shell + engine, watch only.** The app starts the bundled engine and shows the live market from the user's machine. Unsigned CI builds for macOS, Windows and Linux.
2. **Wallet.** Create or restore, balance, receive. Token-protected write API.
3. **Buy MARS.** The full taker flow, including refunds.
4. **Sell MARS.** Auto-Maker controls.
5. **Signed and notarized releases, with automatic updates.**

Later: the same engine in the Martian Republic mobile app.
