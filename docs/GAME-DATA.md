# The data tables

Arctic Combat keeps its catalogue in the client: item names, prices, rental
periods, box contents, level curve, quest text, shop icons. The retail server
only ever sent ids. ACEmu does the same, which means it needs the same tables
to know what id 10203 costs or how much experience level 12 takes.

**Those tables are not in this repository**, and `server/data/*.json` is in
`.gitignore` so they stay out of yours. They are WebZen's data, derived from a
client install. What follows is the shape the server expects, so you can
produce them from your own copy or write your own from scratch.

Everything here is optional - see *Running without the tables* below.

## Running without the tables

A server with an empty `data/` folder is a working server. Each missing file is
warned about once at startup and then the server carries on:

- **New characters are playable.** The starter inventory is built into the
  server, not read from a table: a rifle, a sidearm, a grenade, a flash bang
  and one quick-slot loadout, plus the starting balance
  (`ACEMU_START_POINTS` / `ACEMU_START_CASH`). You can log in, make a room and
  play a match.
- **Progression works.** Without `levels.json` the server uses a built-in
  ladder of 100 levels, each costing 10 % more experience than the last. The
  client draws the rank name from its own table, so only the thresholds matter.
- **The shop is empty, and you can fill it by hand.** Items, Add item in the
  admin web app (`POST /api/items`) adds a catalogue row: an id the client
  knows, an item type, a price, and it is in the shop. Everything downstream -
  buying, selling, granting, random boxes, coupons, mail attachments - works
  off the catalogue, so a dozen rows added by hand are enough for a LAN party.
- **Everything else degrades quietly.** No promotions means no level-up
  rewards, no quest table means an empty quest list, no `icons.json` means
  placeholder pictures in the web app. Nothing fails, nothing panics.

What you cannot do without the tables is reproduce the retail catalogue, with
its several thousand items and their original prices. That is the part that is
the game's own data.

## File formats

Each file is a JSON array of objects. Unknown fields are ignored, missing
optional fields take the default shown. Every file can be pointed elsewhere
with an environment variable. A UTF-8 BOM is tolerated.

The catalogue tables (`items`, `shop`, `products`, `boxes`, `availability`) are
read once, when the `catalog` table is empty, and written into the database;
after that the database is the truth and the files are ignored. The rest are
read on demand at runtime.

## items.json — `ACEMU_ITEMS`

Every item id the client knows, including the ones the shop never lists.

```json
[{ "id": 10100, "name": "K-2", "type": 1 }]
```

| field | meaning |
|---|---|
| `id` | item type id, what goes over the wire |
| `name` | display name, used by the admin app and the logs |
| `type` | the client's item type (slot class: primary, secondary, melee, grenade, gear, …) |

## shop.json — `ACEMU_SHOP`

The shop table. One row per listed item, in the order the client lists them.

```json
[{
  "id": 10100, "name": "K-2", "kind": 1, "category": "RIFLE",
  "sell_price": 480,
  "periods": [24, 168, 720, 0, 0],
  "points":  [900, 4500, 14000, 0, 0],
  "cash":    [0, 0, 0, 0, 0]
}]
```

`periods`, `points` and `cash` are parallel arrays of up to five purchase
options; a period of `0` hours means permanent, and an option with neither
points nor cash is dropped. `sell_price` is what the item resells for - `0`
makes it unsellable, which is how the retail table marks most event gear.
`category` decides whether an item is treated as box-only (spoils tiers and
the prizes printed on boxes are never put in the shop).

## products.json — `ACEMU_PRODUCTS`

The cash-shop feed, used as a fallback price for rows the shop table prices at
zero, and as the "featured" list the client requests separately.

```json
[{ "id": 10100, "cost": 4500, "sale_type": 3, "hours": 168 }]
```

`sale_type` 3 means the cost is in points, 2 means cash coins. Product numbers
are assigned by file position, so inserting a row renumbers everything after
it.

## boxes.json — `ACEMU_BOXES`

Random boxes. `featured` are the prizes printed on the box, `items` the filler
reel, `bonus` the second reel.

```json
[{
  "box": 17000,
  "featured": [10490, 10491],
  "items": [{ "id": 10100, "period": 168 }, { "id": 13020 }],
  "bonus": [19000]
}]
```

The real odds are not in the client. On seeding, the featured prizes share half
the probability and the fillers the other half; edit the weights in the admin
app afterwards if you want something else. A filler `period` of `0` or `1` is
permanent.

## availability.json — `ACEMU_AVAILABILITY`

House rules, applied last, so a database reset does not undo them. This is the
one catalogue file worth writing by hand: it is how you decide what your shop
actually sells.

```json
[{ "id": 10490, "availability": "lootbox", "sell_price": 0,
   "prices": [{ "hours": 0, "points": 25000, "cash": 0 }] }]
```

`availability` is `shop`, `lootbox` or `hidden`. `prices` and `sell_price`
replace the shop table's values when present.

## levels.json — `ACEMU_LEVELS`

The rank ladder. `upper_exp` is the cumulative experience needed to *leave*
that level. Without this file nobody levels up.

```json
[{ "level": 1, "name": "Trainee", "upper_exp": 500 }]
```

## promotions.json — `ACEMU_PROMOTIONS`

What a player is given on reaching a level. Level 1 is the starter kit, which
is why a brand-new character already has a weapon. `hours` of `0` is permanent;
the retail table hands out 360/720/1080-hour rentals.

```json
[{ "level": 1, "item": 10100, "hours": 0 }]
```

## itemboxes.json — `ACEMU_ITEMBOXES`

The pick lists behind coupons and item boxes, where the player chooses rather
than rolls. `part` is an optional weapon-part id attached to the choice.

```json
[{ "box": 18100, "choices": [{ "id": 10100 }, { "id": 10200, "part": 1049 }] }]
```

## weapon-parts.json — `ACEMU_WEAPON_PARTS`

The attachments a weapon spawns with. The client has no table for this because
the retail server sent a part id with every item instance; without this file
the gold, dragon and custom weapon models appear with stock attachments.

```json
[{ "id": 10490, "default_part": 134480897 }]
```

`default_part` is a packed field: `sight | barrel << 10 | silencer << 17 |
gunstock << 24`.

## quests.json — `ACEMU_QUESTS`

Quests and timed events. Long, and the only table where the text matters,
because the client does not have the descriptions either.

```json
[{
  "id": 4001, "quest_type": 0, "link_id": 0, "link_step": 1,
  "create_level": 1, "display_order": 10,
  "category": "Combat", "description": "Kill 50 enemies",
  "mission_type": 1, "mission_count": 50,
  "reward_exp": 1000, "reward_point": 2000,
  "reward_items": [], "reward_parts": [],
  "next_quests": [4002],
  "event_start": 0, "event_end": 0, "enabled": true
}]
```

`quest_type` 1 is an event, 2 is a daily. `event_start` / `event_end` are OLE
dates (`0` = always). `mission_type` selects the counter the match report feeds
(kills, headshots, wins, rounds, …).

## daily-quests.json — `ACEMU_DAILY_QUESTS`

Which quest id backs each daily slot.

```json
[{ "daily_type": 2, "step": 1, "quest_id": 4101 }]
```

`daily_type` 2 is the attendance calendar, 3-5 the three puzzles, 7 the special
mission.

## daily-rewards.json — `ACEMU_DAILY_REWARDS`

```json
[{ "reward_type": 0, "num": 7, "item_id": 17000, "count": 1 }]
```

`reward_type` 0 pays out after `num` attendance check-ins, 1 for finishing
puzzle `num`, 2 is the special mission's reward.

## capsule-rewards.json — `ACEMU_CAPSULE_REWARDS`

The capsule-machine tiers: collect `need` of something, get `reward_id`.

```json
[{ "reward_type": 0, "index": 0, "need": 10, "reward_id": 17200 }]
```

## icons.json

Item id to shop-icon name, used only by the admin web app:

```json
[{ "id": 10100, "icon": "wp_k2" }]
```

The PNGs themselves go in `server/webapp/public/icons/<name>.png` (256x64) and,
optionally, `public/icons-hd/<name>.png` for the large picture on an item page.
Both folders are ignored by git for the same reason as the tables. With no
`icons.json` the web app shows a neutral placeholder everywhere and nothing
breaks.
