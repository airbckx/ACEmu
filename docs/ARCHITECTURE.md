# How ACEmu is put together

Three small services and one database. Nothing talks to anything else over a
network except the client.

```
        client  ──TCP 55000──►  acemu-login    ─┐
           │                                    │
           │    ──HTTP 55001──►  (launcher)     ├── SQLite  server/save/acemu.db
           │                                    │
           └────TCP 55999──►   acemu-channel   ─┤
                └──UDP 55997/55998 (NAT probe)  │
                                                │
     browser ──HTTP 55990──►   acemu-admin     ─┘
```

## acemu-login (port 55000)

The cold-boot path: version and checksum handshakes, the login itself, the
server list, and the handover to the channel server. It also answers the
character info the client shows before you reach the lobby.

Next to the game port it runs a tiny HTTP endpoint (55001, `ACEMU_LOGIN_HTTP`)
for the launcher: `/ping`, `/verify`, `/ensure` for accounts, and an optional
`extras` file pack the launcher can install into a client (see
`CLIENT-TWEAKS.md`). It is hand-rolled on `std`, because three JSON calls do
not justify an async runtime.

## acemu-channel (port 55999)

Everything after login. One module per area of the protocol under
`handlers/`, tried in turn by `handlers::dispatch`:

| module | what it answers |
|---|---|
| `login` | handshake, keepalive, logout, the client's own error reports |
| `lobby` | channel list, user list, the room list and its live refresh |
| `room` | create, enter, leave, settings, teams, ready state |
| `netgame` | match start, UDP endpoint exchange, hole punching, late join |
| `items` | inventory, equipping, quick slots, weapon remodelling |
| `shop` | buying, selling, random boxes, item boxes, the cash feed |
| `results` | score table, kill log, wear, booty, the tutorial bracket |
| `matchlog` | objective events: rounds, bombs, points, chopper kills |
| `quests`, `daily` | quests, events, coupons, attendance and puzzle rewards |
| `mail`, `friends`, `ranking` | mailbox, friends and blocks, rankings |
| `upkeep` | repairs, durability warnings, consumables |
| `stubs` | the rest, answered with defaults so the client moves on |

Two UDP responders (55997, 55998) answer the client's NAT probe with the
address they saw it come from. That is what lets the engine work out its own
endpoint, which is why the Docker setup uses host networking.

The match itself does not run here. One client hosts it as a listen server and
the others connect to it directly; the channel server only arranges the
introduction and records what the host reports afterwards.

## acemu-admin (port 55990)

An axum REST API under `/api` plus the static export of the web app. Sign-in is
a JWT; the password is generated on first start into `server/save/` unless
`ACEMU_ADMIN_PASSWORD` is set. The web app is where you edit players, items,
boxes, quests, coupons and mail, look at match history and the leaderboard, and
open `/party` on a television.

## The database

One SQLite file, `server/save/acemu.db`, in WAL mode, opened by all three
services. Nothing is cached across requests, so an edit in the admin app is
visible to the game on its next packet. The schema is created on first open and
seeded from the tables in `server/data/` (see `GAME-DATA.md`) when the catalogue
is empty.

Tables, roughly in the order they matter: `accounts`, `players`, `items`,
`skills`, `qslots`, `catalog`, `prices`, `box_entries`, `matches`,
`match_players`, `match_kills`, `match_events`, `quests`, `quest_progress`,
`daily_*`, `mail`, `coupons`, `friends`, `blocks`, `live`.

## The protocol, briefly

The client's lobby stack frames every packet as

```
int32 length | int32 magic (200001) | int32 opcode | payload
```

Payload fields are little-endian; strings are null-terminated UTF-16; arrays
are a byte length followed by packed elements with no count. Request and reply
opcodes come in pairs, and the reply is almost always the request plus one.

`acemu-proto` holds the generated structs for those packets and the framing
code, with tests that decode real captured bytes. The client's own packet
encryption is skipped: the launcher patches the two branches that would enable
it, which is also why ACEmu only talks to a patched client.

## Conventions worth knowing

- The server trusts the host client's match report. It is a LAN project.
- Item definitions (names, prices, models) live in the client. The server only
  ever sends ids and per-instance state.
- Rentals and skills carry an expiry as an OLE date; zero means permanent, and
  the client treats a zero expiry on a skill as "expired", so permanent skills
  go out with a far-future stamp instead.
- Durability is 0..100000, where the client displays 100000 as 100 %.
