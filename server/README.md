# The servers

Everything the servers need is in this folder, so a copy of it - the built
binaries next to `data/` and `save/` - runs anywhere.

```
server/
  crates/
    acemu-proto     the wire protocol: framing, a struct per packet
    acemu-paths     finds this folder at runtime (data/ and save/)
    acemu-db        SQLite storage: accounts, players, inventories, catalogue, matches
    acemu-login     login server, TCP 55000, launcher HTTP 55001
    acemu-channel   channel server, TCP 55999, UDP 55997/55998
    acemu-admin     admin REST API and web app host, HTTP 55990
  data/             the game's tables - not in this repository, see ../docs/GAME-DATA.md
  save/             acemu.db and the generated admin password
  webapp/           Next.js admin app (npm run build -> webapp/out, served by acemu-admin)
```

## Running

Build once, then start the three servers. They locate this folder themselves
(see `acemu-paths`), so it does not matter where you start them from; the
channel server logs `home=` so you can check. `ACEMU_HOME` overrides it.

```powershell
cargo build --workspace
.\target\debug\acemu-login.exe
.\target\debug\acemu-channel.exe
.\target\debug\acemu-admin.exe
```

Then `http://127.0.0.1:55990/` for the admin app, after one `npm install` and
`npm run build` in `webapp/`. While working on the web app, `npm run dev`
serves it on port 3000 against the same API.

For other machines to connect, set `ACEMU_PUBLIC_IP` to this machine's LAN
address before starting anything. It is what the login server hands out as the
channel server's address, and what the channel server uses for the NAT
responders and room owner addresses. Point the players' launchers at the same
address.

| variable | default | what it does |
|---|---|---|
| `ACEMU_PUBLIC_IP` | the first non-loopback local address | address handed to clients |
| `ACEMU_HOME` | the folder containing `data/` and `save/` | overrides the lookup |
| `ACEMU_LOGIN_HTTP` | `0.0.0.0:55001` | the launcher endpoint; `off` disables it |
| `ACEMU_ADMIN_BIND` | `127.0.0.1` | `0.0.0.0` exposes the admin app to the LAN |
| `ACEMU_ADMIN_PORT` | `55990` | admin port |
| `ACEMU_ADMIN_USER` / `ACEMU_ADMIN_PASSWORD` | `admin` / generated | the admin account |
| `ACEMU_JWT_SECRET` / `ACEMU_JWT_HOURS` | random, stored / `24` | admin token signing and lifetime |
| `ACEMU_GUEST_NAT` | `1` | `0` skips the NAT probe for clients on this machine |
| `ACEMU_EXTRAS_DIR` | `client/extras` | the optional launcher file pack |
| `ACEMU_ITEMS`, `ACEMU_SHOP`, … | `data/<name>.json` | one per data table, see `../docs/GAME-DATA.md` |

## Docker

`Dockerfile`, `docker-compose.yml` and `docker-entrypoint.sh` build the three
servers and the web app into one image. The build context is `server/`, and
only `crates/`, `data/` and `webapp/` go in; `save/` lives on a volume.

```bash
cd server
ACEMU_PUBLIC_IP=192.168.1.50 docker compose up -d --build
docker compose logs -f
```

Without compose:

```bash
docker build -t acemu .
docker run -d --name acemu --network host --restart unless-stopped \
  -e ACEMU_PUBLIC_IP=192.168.1.50 -v acemu-save:/app/save acemu
```

**Host networking is not optional.** The UDP responders answer each client with
the source address they saw, and the engine takes that as its own public
endpoint for the match. Published ports (and Docker Desktop in general) replace
it with the proxy's address and the match never connects. Use `--network host`
on a Linux host. The ports in play: 55000 and 55001 (login and launcher HTTP),
55999 (channel), 55997 and 55998 UDP, 55990 (admin).

The container stops if any of the three servers exits, and `restart:
unless-stopped` brings the set back together. The database is created on first
start inside the `acemu-save` volume; to carry players over, stop the container
and `docker cp save/acemu.db acemu:/app/save/`.

## Storage

`save/acemu.db`, SQLite in WAL mode, shared by all three servers. Nothing is
cached across requests, so an admin edit is live on the next packet.

| table | holds |
|---|---|
| `accounts` | username, password hash (SHA-256, empty = none), linked character |
| `players` | balance (points, cash), career stats, next unique id |
| `items` | owned item instances: slot, stack, durability, parts, rental expiry |
| `skills` | owned skills and their expiry |
| `qslots` | the five quick-slot loadouts |
| `catalog` | every item id: name, type, kind, resale price, availability, shop order |
| `prices` | purchase options per item (hours, points, cash) |
| `box_entries` | random-box contents per box and reel, with a relative weight |
| `live` | what the channel server last published: sessions, rooms, heartbeat |
| `matches` | one row per finished match: map, mode, host, duration, scores, winner |
| `match_players` | per participant, bots included: team, score, kills, assists, deaths, EXP, points, headshots, won |
| `match_kills` | the kill feed: time into the match, killer, victim, weapon, body part |
| `match_events` | objective timeline: round ends, bomb plants and defuses, points taken, domination score, chopper kills |
| `host_priority` | the last host-priority point each player reported (lower is a better host) |
| `mail` | mailbox rows per owner and tab, with an attached item or points |
| `coupons`, `coupon_uses` | redeem codes and who used them; `ACEMU-WELCOME` is seeded (5000 points, once per player) |
| `player_extra` | community options, capsule counter and reward bits, booty seed, daily-quest day |
| `quests`, `quest_progress` | the quest table and per-player progress |
| `capsule_rewards` | capsule-machine tiers |
| `daily_quests`, `daily_rewards`, `daily_state` | the daily rotation, its rewards, and per-player check-ins and puzzles |
| `friends` | friend list, one row per direction |
| `blocks` | block list, one-directional: a blocked player cannot whisper, friend-invite or room-invite |
| `admin_users`, `admin_settings` | web app sign-in |

A match is recorded when the host reports the score table. The kill feed comes
from the host's reports during the match and the objective events from its
round and game-play reports (`handlers/matchlog.rs`); both are attached to the
match when it is recorded.

`availability` per catalogue item is `shop` (listed), `lootbox` (not listed,
boxes may still hand it out) or `hidden` (neither). Box odds are relative
weights per reel, editable on the Loot boxes page.

To add a coupon code by hand (the serial is upper-cased):

```sql
INSERT INTO coupons (serial, item_id, item_count, points, cash, uses_left, note)
VALUES ('MYCODE', 2120, 1, 0, 0, 10, 'a rifle');
```

Seeding happens on first start while a table is empty, from `data/` - see
`../docs/GAME-DATA.md`, which also covers what a server with no `data/` folder
at all can and cannot do (short version: you can play; the shop starts empty
and you fill it from the web app). After that the database is the truth. To start over,
stop the servers and run `acemu-admin --reset`: it moves `acemu.db` into
`save/backup-<timestamp>/` and creates a fresh one.

## Accounts

An account is created the first time a name logs in, with no password. Set one
in the admin app and the login server then requires it; without it the client
reports a login failure rather than letting you in.

The launcher does this for the player through the login server's own HTTP
endpoint (`crates/acemu-login/src/http.rs`), which is independent of the admin
service - you can play with the admin service switched off:

```text
GET  /ping                           {"ok":true,"service":"acemu-login"}
POST /verify  {username, password}   {"result": ok|wrong|no_password|no_such_account}
POST /ensure  {username, password}   {"result": ok|created|password_set|wrong|no_password}
GET  /extras                         the optional file pack's manifest, or 404
GET  /extras/<name>                  one file from it (Range: bytes=N- honoured)
```

`extras` is the only pack, it is empty unless you fill it, and it never
contains game files - see `../docs/CLIENT-TWEAKS.md`.

## Admin sign-in

The web app and every `/api` route need a signed-in admin: a JWT in
`Authorization: Bearer …`, issued by `POST /api/auth/login`. Admin users live
in `admin_users` (salted SHA-256) and have nothing to do with game accounts.

On first start the admin user is `ACEMU_ADMIN_USER` (default `admin`) with
`ACEMU_ADMIN_PASSWORD`. With no password set, one is generated, logged, and
written to `save/admin-password.txt`, which is deleted once you change the
password in the web app. From the shell:
`acemu-admin --set-admin-password <user> <password>`.

Tokens are signed with `ACEMU_JWT_SECRET`, or a random secret kept in
`admin_settings`, and last `ACEMU_JWT_HOURS` hours. The browser holds the token
in local storage, so the party screen on a television needs one sign-in there.

## Admin API

JSON under `/api`. Errors are `{"error": "…"}` with a 4xx status.

```text
POST   /api/auth/login                  {username, password} -> {token, username, expires_at}   (the only route without a token)
GET    /api/auth/me                     {username, expires_at}
POST   /api/auth/password               {current, new}
GET    /api/summary                     counts
GET    /api/live                        connected sessions and the open rooms
GET    /api/accounts                    POST {username, password}
POST   /api/accounts/verify             {username, password} -> {result, account}
PATCH  /api/accounts/{id}               {username?, password? ("" clears), character? (null unlinks)}
DELETE /api/accounts/{id}?with_player=true|false
GET    /api/players                     summaries
GET    /api/players/{name}              full record with items, skills, quick slots
PATCH  /api/players/{name}              {points?, cash?, stats?: {...}}
DELETE /api/players/{name}
POST   /api/players/{name}/items        {item_id, stack?}
PATCH  /api/players/{name}/items/{uid}  {stack?, durability?, slot?}
DELETE /api/players/{name}/items/{uid}
POST   /api/players/{name}/skills       {skill_id}
DELETE /api/players/{name}/skills/{uid}
POST   /api/players/{name}/repair       every item back to full durability, no charge
GET    /api/players/{name}/social       {friends, blocks, blocked_by}
POST   /api/players/{name}/friends      {name}   mutual
DELETE /api/players/{name}/friends/{other}
POST   /api/players/{name}/blocks       {name}   one-directional
DELETE /api/players/{name}/blocks/{other}
GET    /api/items?q=&kind=&availability=&limit=&offset=
POST   /api/items                       {id, item_type, name?, kind?, category?, sell_price?, availability?, prices?}
GET    /api/items/{id}
PATCH  /api/items/{id}                  {name?, sell_price?, availability?, prices?: [{hours, points, cash}]}
GET    /api/kinds
GET    /api/icons                       item id -> shop icon name (data/icons.json)
GET    /api/boxes                       boxes with entry counts
GET    /api/boxes/{id}                  entries with names and computed percentages
PATCH  /api/boxes/{id}                  {entries: [...]}   replaces the list
GET    /api/matches?limit=&offset=&player=
GET    /api/matches/{id}                scoreboard, kill feed, objective events
DELETE /api/matches/{id}
GET    /api/players/{name}/matches
GET    /api/leaderboard                 career stats plus match aggregates per player
```

## Channel server layout

`crates/acemu-channel/src`: `main.rs` (startup and the socket loop),
`session.rs` (connections and pushes to other players), `room.rs` (the game
rooms, keyed by room number; a player is in at most one), `nat.rs`,
`inventory.rs` (game logic over the database), `player.rs` and `loadout.rs`
(packet builders), `live.rs` (publishes state for the admin app), and
`handlers/` with one module per protocol area. `handlers::dispatch` tries them
in order; `handlers/stubs.rs` is last and answers whatever is left with
defaults.
