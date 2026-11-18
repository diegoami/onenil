# Static faithful desktop compositions

[designed] These are design pages generated from identified original One-Nil
screens, not a game UI, approved departures or an implementation of campaign
behavior. The [plan](design.json) maps ten variants to the accepted screen families
and exact research records/images. The generator preserves the original logical
80×25,720×400 composition, wording, bars, selected-state context and footer.
Original artwork pixels are not embedded in the proposed SVGs. The reference
PNGs are read only to observe displayed cell colours; new text and vector
primitives produce the proposed pictures. This is a visual study, not a new port
of the original renderer or a claim of pixel-identical reconstruction.

[designed] The proposed font is **PxPlus IBM VGA 8x16**, by VileR, The Ultimate
Oldschool PC Font Pack v2.2, [int10h.org](https://int10h.org/oldschool-pc-fonts/),
CC BY-SA4.0. [FONT-LICENSE.TXT](FONT-LICENSE.TXT) is retained unchanged from the
pack. Font SHA256:
`c99fb740581471c2bd239a200c9f7a6c12f60a7099b0f0af795e405281bc2c5f`.
[T92] The font binary itself is now tracked beside the generator
([PxPlus_IBM_VGA_8x16.ttf](PxPlus_IBM_VGA_8x16.ttf)) and digest-checked against
the SHA above at every use — research #187's runner-egress fetch of the vendor
pack stopped matching the pinned pack sha (`21b3c0a3…`) five reruns running, so
the transport died and the licensed, digest-pinned bytes moved into the
repository. The vendor pack URL stays documented provenance; `--fetch-font`
keeps its signature for local reproducers and no longer touches the network.
The font binary is a build input, not an original One-Nil artifact. Its
independent provenance was cross-checked against goal2 at
`98860065cf928e52e43453a0c827e7ad2a564688`; no One-Nil font or palette file is used.

[designed] P01–P04 in the plan explain the proposed/unknown differences: font,
uniform scaling/letterboxing, provisional decoded roster-role aliases, and
independently drawn pitch figures/ball at captured text-cell locations. Their
coordinates are design positions, not native match-coordinate parity. Captured
scores, descriptive fields, bar colours and clock glyphs identify the source
scenario; they are not generated outcomes or default career state. The custom
role glyphs remain unresolved rather than inheriting goal2's role table.

[designed] SVG pages need the verified replacement font supplied by the renderer;
opening them without it can show a fallback font. The measured PNGs, with exact
source/asset/runtime hashes in the render receipt, are the visual inspection
artifacts. No original bitmap is pasted into a proposed page. Desktop delivery
is settled; using Chromium for static screenshot generation does not select a
browser game, HTTP architecture, Electron or another toolkit.

## Reproduce without private game files

```sh
# Build-only Python dependencies; use the project's existing virtual environment.
python -m pip install Pillow==12.0.0 playwright==1.58.0
python -m playwright install chromium

# A public research checkout supplies committed decoded records and PNGs only.
python docs/desktop-fidelity/mockups/build.py --research-root ../onenil-archaeology --fetch-font --build
# Commit producer/plan/pages before measured output, then:
python docs/desktop-fidelity/mockups/build.py --research-root ../onenil-archaeology --negative-checks --render
```

[designed] `--build` regenerates mutable design SVG sources under `pages/`.
Without it, source/output drift fails. Every measured render allocates a new
exclusive `records/T51/render-rNNNN.json` and image prefix, retaining failures.
Three documented viewports are checked:1440×900 at1×,960×640 at1× and720×450 at2×.
The full logical screen and text bounding boxes must remain unclipped; this is
geometry evidence, not desktop/OS/DPI/accessibility certification. Native toolkit
feasibility must still prove focus, interaction and engine/save lifecycle.
