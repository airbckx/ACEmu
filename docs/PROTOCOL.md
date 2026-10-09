# The wire protocol

Notes on BatteryNet, the lobby protocol Arctic Combat uses between the client
and the login/channel servers. This is the part that was not documented
anywhere; everything below was established by watching a client talk to a
socket and by decoding what it sent back.

The match itself is plain Unreal Engine networking between the players and
does not involve ACEmu.

## Framing

```text
offset  size  field
     0     4  int32  Length    total packet size, this field included
     4     4  int32  Magic     always 200001 (0x00030D41)
     8     4  int32  Opcode
    12     n  payload
```

A stock client wraps that in a further `int32 length | ciphertext padded to 16
| 0x08` envelope. ACEmu does not implement the cipher: the launcher patches the
two branches that would turn it on, and the client then speaks the plaintext
framing above. This is the only reason a patched client is required.

TCP gives no message boundaries, so the reader buffers until `Length` bytes are
present and then takes one packet at a time. A length larger than 1 MiB is
refused rather than trusted.

## Payload values

Little-endian throughout.

| type | encoding |
|---|---|
| `byte`, `int16`, `int32`, `float`, `double` | as you would expect, little-endian |
| `bool` | one byte, 0 or 1 |
| `string` | UTF-16 code units terminated by `0x0000` |
| array | `int32` byte length, then the elements packed back to back |

Arrays carry no element count. The reader has to divide the byte length by the
element size, which also means an array of strings is only parseable by walking
it. Parallel arrays are the norm: a packet describing eight players is eight
one-element-per-player arrays, not an array of structs.

## Opcodes

Request and reply come in pairs and the reply is the request plus one. Both are
the index of the corresponding `rf*` function inside its link class, so they
are dense, start at zero, and differ between the login and channel servers -
opcode 12 means two different things depending on which socket it arrived on.

Naming follows the client: `rfReqLogin` is what the client sends,
`rfAckLogin` what it expects back. Pushes without a request exist too
(`rfAckNotify*`, and anything the server volunteers, like a new room appearing
in the list).

`acemu-proto/src/generated.rs` holds a struct per packet with the fields in
wire order. It is generated, and the generator and its inputs - tables read out
of a client install - are deliberately not part of this repository; the
generated file is checked in because nothing builds without it. Treat it as
read-only: the handlers are where behaviour belongs.

## Values that are not obvious

- **Dates** are OLE automation dates: days since 1899-12-30 as a `double`. An
  expiry of `0` means "no expiry", except on skills, where the client renders a
  zero expiry as *expired* - so permanent skills go out with a date far in the
  future instead.
- **Durability** runs 0..100000 and is displayed divided by 1000.
- **Unique ids** of item instances are `double`, not integers. Skills use their
  own id as the unique id.
- **Quick-slot display order** is 1-based. Zero means "unset" and the pawn
  spawns without that weapon.
- **Weapon parts** are one packed int: `sight | barrel << 10 | silencer << 17 |
  gunstock << 24`.
- **A sell reply** reports the stack size that remains, not the number sold.
  Zero means the client removes the row - which is what you want for a
  one-off item, and a bug that is easy to spend an evening on.
- **Money** comes in two currencies: points, earned by playing, and cash
  coins, which were bought. Prices carry both; zero means "not for sale in this
  currency".

## Things the client decides on its own

Worth knowing, because no amount of server work changes them:

- Item names, models, icons, prices and shop categories come from the client's
  own tables. The server sends ids. See `GAME-DATA.md`.
- `rfAckMyEquipedItemList` must only be sent when asked for. The handler that
  receives it is part of the tutorial chain, so pushing it unprompted drops the
  client into tutorial mode.
- The room list widget compares rows by the wrong column, so an update to an
  existing row is ignored. Pushing a live refresh means removing the row and
  sending it again.
- Buy, sell and inventory replies have to arrive in the order the client's page
  code expects, because each handler triggers the next request. An otherwise
  correct reply sent early is simply dropped.
- The UI positions itself in script and addresses its texture atlases by pixel
  coordinate, which is why neither widescreen layout nor resized UI art is
  possible from outside.
