# ACEmu

A server for **Arctic Combat**, the WebZen shooter from 2012 that ran on the
Unreal Engine 2.5 and was switched off worldwide in 2013. ACEmu speaks the
game's lobby protocol well enough that an unmodified retail client logs in,
sits in a channel, buys gear, creates a room and plays a match with friends.

It started as a reverse-engineering exercise: the client is still on Steam, the
servers are gone, and the protocol is documented nowhere. It ended up good
enough to run a LAN party on. It is an educational project, with no connection
to WebZen - see the disclaimer below.

**This repository contains no game files.** No art, no maps, no sounds, no
tables lifted out of the client, and no tooling for taking them apart. You need
your own copy of the game: Steam app `212370`
(<https://steamdb.info/app/212370/>). The game is delisted, so Steam will only
install it for accounts that already own it.

## What works

- Login, accounts, character creation, the channel lobby and its room list.
- Starting a match: one player's client hosts as a listen server, the others
  are handed its endpoint after a NAT handshake. On a LAN this just works.
- Inventory and shop: items, prices, rentals, repairs, random boxes, weapon
  parts, quick-slot loadouts, skills.
- Progression: experience, levels, promotion rewards, career statistics.
- Social: friends, block lists, whispers, room invites, mail, coupon codes,
  quests and daily rewards.
- Bookkeeping: every finished match is stored with its scoreboard, kill feed
  and objective events, and shows up in the admin web app, the leaderboard and
  a fullscreen party screen for a TV.

## Disclaimer

This is an educational project: an exercise in reverse engineering a network
protocol and writing a server for it, published so other people can read it.

It is **not affiliated with, endorsed by, sponsored by or connected to WebZen
in any form**, nor to Dragonfly or any other company involved in Arctic
Combat. "Arctic Combat", "WebZen" and all related names, logos and artwork
belong to their respective owners and are used here only to say what this
software talks to.

No game content is distributed here: no executables, art, maps, sounds, text or
data tables. Everything in this repository was written from scratch. Running it
needs your own legally obtained copy of the game.

There is no anti-cheat, no account security worth the name, and no attempt at
reproducing the original service feature for feature. The design target is a
living room with a switch in it, not the public internet.

## Pieces

```
server/crates/acemu-login     login server, TCP 55000, plus a small HTTP endpoint on 55001
server/crates/acemu-channel   the game server: lobby, rooms, shop, matches, UDP 55997/55998
server/crates/acemu-admin     REST API and the web app host, HTTP 55990
server/crates/acemu-db        SQLite storage shared by all three
server/crates/acemu-proto     the wire protocol (framing, packet structs)
server/crates/acemu-paths     finds data/ and save/ at runtime
server/webapp                 Next.js admin app: players, items, matches, events, party screen
launcher/                     Windows launcher: patches the client, points it here, starts it
tools/                        two small PowerShell helpers for testing
```

## Running it

Build and start the three servers (Rust stable, any platform):

```bash
cd server
cargo build --release
./target/release/acemu-login &
./target/release/acemu-channel &
./target/release/acemu-admin &
```

Build the web app once so the admin service has something to serve:

```bash
cd server/webapp
npm install
npm run build
```

Then open <http://127.0.0.1:55990/>. The admin service generates an
administrator password on first start and writes it to
`server/save/admin-password.txt`; `ACEMU_ADMIN_PASSWORD` overrides it.

For anyone else to connect, set `ACEMU_PUBLIC_IP` to the machine's LAN address
before starting the servers, and point the players' launchers at it. There is a
Dockerfile and a compose file in `server/` that bundle all three services; they
need host networking, because the UDP responders hand the client back the
address they saw.

## The client side

`launcher/` builds a small Windows launcher with the C# compiler that ships
with the .NET Framework, no SDK required:

```powershell
.\launcher\build.ps1
```

It applies the three byte patches the emulator needs to an install you already
have, writes the login redirect, creates or verifies your account on the login
server and starts the game. It can also drop in a Direct3D 9 wrapper and a few
engine settings that help on modern Windows; both are optional, both are
reversible, and neither ships here. See `launcher/README.md`.

## Game data

The catalogue the shop shows - items, prices, boxes, quests, the level curve -
lives in the client's own parameter package, not in this repository.

Running without it is a supported way to use this. A fresh character gets a
built-in starter kit, progression falls back to a built-in level curve, and the
shop starts empty: *Items → Add item* in the admin app puts any id the client
knows on sale with a price of your choosing, and buying, selling, boxes,
coupons and mail all work off what you added. A dozen rows is a LAN party.

`docs/GAME-DATA.md` has the details, plus the shape of every table the server
will read if you do produce them from your own install.

## Documentation

- `docs/ARCHITECTURE.md` - how the three services fit together and what the
  database holds.
- `docs/PROTOCOL.md` - the wire protocol, and the client quirks that shaped the
  handlers.
- `docs/GAME-DATA.md` - the data tables the server expects, and what running
  without them looks like.
- `docs/CLIENT-TWEAKS.md` - the optional renderer wrapper and ini settings.
- `server/README.md` - storage, the admin API, Docker.
- `launcher/README.md` - what the launcher does to a client install.

## Licence

MIT, see `LICENSE`. That covers the code in this repository and nothing else:
Arctic Combat itself, its assets and its trademarks belong to their owners.
