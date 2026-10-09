# Optional client tweaks

Two things the launcher can do to a client install beyond the patches it needs
to talk to ACEmu. Both are off by default, both are a file or an ini line, and
both come back off with RESTORE. Neither is required to play.

## Modern renderer

The client renders through `D3D9Drv.dll` on Direct3D 9, which modern drivers
run but no longer run well: 4K and ultrawide modes are missing, frame pacing
wobbles, and anisotropic filtering is whatever the driver decides. A D3D9
wrapper dropped into `System\` next to `ArcticCombat.exe` is loaded before the
system DLL and translates to a current API. [dgVoodoo2][dgv] works well here;
v2.87.5 was what this was tested with, using its `MS\x86\D3D9.dll` and a
`dgVoodoo.conf` with

```ini
VRAM = 1024
Filtering = 16
Antialiasing = 4x
dgVoodooWatermark = false
```

[dgv]: https://github.com/dege-diosg/dgVoodoo2

**The wrapper is not in this repository.** It is somebody else's software with
its own licence. To let the launcher install it, put the files in the login
server's extras folder:

```
server/client/extras/D3D9.dll
server/client/extras/dgVoodoo.conf
server/client/extras/dgVoodoo2-README.txt   (the upstream licence/readme)
server/client/extras/manifest.json
```

`manifest.json` is `{"version":"<anything>","files":[{"name","size","sha256"}]}`
and only files listed in it are served. `ACEMU_EXTRAS_DIR` moves the folder.
With no extras folder the login server simply answers 404 and the launcher
reports that the renderer is unavailable.

What the launcher then does, with "Modern renderer" ticked: download the pack
into `<GameDir>\Extras\`, copy its files into `System\`, move any pre-existing
foreign `System\D3D9.dll` to `Extras\backup\`, and record what it did in
`Extras\applied.json`. Unticking copies the backup back and deletes the rest.

## Performance tweaks

The game keeps its own `[Graphics]` and `[Controll]` sections in
`System\ArcticCombat.ini` and ships none of the UT2004 engine sections, but the
engine still reads class configuration from that file, so the sections can
simply be added:

```ini
[D3D9Drv.D3D9RenderDevice]
ReduceMouseLag=False            ; stop queueing frames on the GPU - this is the input lag
UseVSync=False
UseTripleBuffering=False

[Engine.LevelInfo]
MaxClientFrameRate=144.000000   ; the default is 90; above ~200 physics timing suffers

[Engine.PlayerController]
DefaultFOV=100.000000           ; UE2's FOV is horizontal at 4:3, so 16:9 crops the view
DesiredFOV=100.000000
```

`ReduceMouseLag=False` is the one worth having; the aiming feels like a
different game. The FOV keys are honoured until the weapon code sets its own
zoom, so expect it to snap back when you scope.

The launcher merges these by section and key when "Performance tweaks" is
ticked, and removes exactly those sections when it is not. It does not back the
ini up, because the game rewrites it on every exit and a backup would be stale
within one session.

## Deliberately not done

- No large-address-aware flag on the executable. It is one reversible bit, but
  it is still an edit to the exe for no measured gain.
- No widescreen HUD layout. The UI positions itself in script, so the HUD stays
  stretched on 16:9.
- No UI texture work. The UI addresses its atlases by pixel coordinates, so a
  resized atlas draws the wrong crops. Tested, reverted, left alone.
