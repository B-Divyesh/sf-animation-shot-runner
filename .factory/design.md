# Shot Runner visual thesis

## Direction

**Monochrome typographic broadsheet.** Shot Runner is a production receipt, not an animation editor, so the site reads like the first and last page of a press proof: an oversized masthead, hard rules, narrow metadata columns, a numbered contact sheet, and terse status marks. The restraint separates it from neon creative-tool dashboards and makes provenance feel tangible.

## Palette

The direction is intentionally single-mode, painted explicitly in warm newsprint and carbon ink.

- `--paper #f1eee5`: warm uncoated stock; page background.
- `--paper-bright #faf8f1`: lifted proofing surface.
- `--ink #121210`: primary text and rules (15.8:1 on paper).
- `--ink-soft #5d5a52`: metadata (5.8:1 on paper).
- `--signal #c73a22`: a restrained crop-mark red for actions and warnings.
- `--success #1f6347`, `--danger #9f271a`: paired with words and symbols, never used alone.

No dark theme: the physical broadsheet metaphor is the identity, not a generic application skin.

## Type and spacing

- Display: self-hosted **Instrument Serif**, regular, for the masthead and editorial pull lines.
- Utility: self-hosted **IBM Plex Mono**, regular/medium, for commands, receipts, labels, and body copy.
- Scale: 14 / 16 / 20 / 30 / fluid 58–118 px. Body never below 16 px.
- Rhythm: 4 px base; primary gaps 8, 16, 24, 32, 48, 72, and 96 px.
- Rules provide alignment; cards appear only for independently actionable license/demo states.

## Interaction grammar

Links underline on approach. Buttons look like printer slugs: square corners, ink fill, 48 px minimum height, 2 px offset press state. Tabs behave like a mechanical index and use correct tab semantics plus arrow keys. Status changes appear as terse proof marks (`READY`, `CACHED`, `HELD`) with plain-language support. The phone layout drops decorative folio metadata, stacks columns, and keeps copy/paste controls full-width.

## Motion

One 220 ms reveal moves the proof image upward by 8 px and fades it in; command-copy feedback crossfades in 150 ms. No loops or parallax. Under `prefers-reduced-motion: reduce`, all transitions and movement are removed and state changes are immediate.

## Original asset plan and provenance

- `site/public/shot-proof.webp` and `shot-proof-768.webp`: generated specifically for Shot Runner with the factory `factory-image` deployment on 2026-08-27. Prompt: “Editorial monochrome contact-sheet illustration for a headless animation render CLI landing page; five sequential cinematic frames of a small geometric courier crossing a stark paper stage, visible frame perforations, crop marks, grease-pencil timing arcs, halftone and ink textures, warm off-white newsprint with near-black ink and one restrained vermilion proof mark; wide landscape composition, tactile analog print proof, sophisticated independent animation festival catalogue; no legible words, no logos, no UI mockup, no gradients, no watermark.” The generated PNG was inspected, locally converted into 146 KB and 29 KB responsive WebP derivatives, and kept below the 300 KB budget. Model license: generated for this product through the factory image service.
- Interface proof marks and icons are hand-authored CSS/SVG primitives in the repository; no stock assets.
