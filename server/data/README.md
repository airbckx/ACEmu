# data/

The game's own tables go here: `items.json`, `shop.json`, `products.json`,
`boxes.json`, `availability.json`, `levels.json`, `promotions.json`,
`itemboxes.json`, `weapon-parts.json`, `quests.json`, `daily-quests.json`,
`daily-rewards.json`, `capsule-rewards.json`, `icons.json`.

None of them are in this repository, and `*.json` here is in `.gitignore` -
they are the game's data, not ours. The servers start without them, warn once
per missing file, and play fine with an empty shop.

`../../docs/GAME-DATA.md` documents the shape of every file, so you can produce
them from your own install or write your own.

This file only exists so the folder does, because the Dockerfile copies it.
