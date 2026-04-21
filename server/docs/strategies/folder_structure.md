# Folder Structure Strategy

## Philosophy

Code is organized by **feature**, not by type. Rather than grouping all components together, all styles together, etc., each self-contained unit of the page owns its own components, data, and logic. This keeps related code co-located and makes each feature independently understandable.

---

## `src/components/` — Global Shell Only

`src/components/` is intentionally sparse. It only holds things that are truly app-wide with no feature affiliation:

- **`Base.astro`** — the HTML shell: `<head>`, meta tags, OG tags, analytics script, and the `<slot />`. Every page wraps its content in this.
- **`Footer.astro`** — the global footer, rendered by `Base.astro` unless `hideFooter` is passed.

Nothing else belongs here. If a component is only used by one feature, it lives inside that feature.

---

## `src/pages/` — Pages and Features

### Top-level pages

Standalone pages (e.g. `index.astro`, `thank-you.astro`) live directly in `src/pages/`. They are thin — they import `Base` for the shell, compose features, and add minimal page-level layout.

### `src/pages/_features/`

Each section of the page is a **feature** — a self-contained folder that owns everything it needs. Features are composed into pages from the outside; they don't know about each other.

Each feature folder follows this internal structure:

#### `index.astro` — Feature entry point

The public face of the feature. This is the only file a page imports. It:
- Pulls data from `api/`
- Renders markup and composes any sub-components from `components/`
- Passes data down to sub-components as props (including icon SVGs, paths, URLs, etc.)
- Includes a `<script>` block if the feature needs client-side behavior, which calls into `util/`

#### `api/index.ts` — Feature data

Holds the feature's static data and types. This is not a network API — it's where the content lives: profile info, link lists, FAQ entries, VCF path, etc. Keeping data here means the `index.astro` stays clean and the data is easy to find and edit.

For features with form submission, a separate `api/submit.ts` handles all HTTP logic (see `form_submission.md`).

#### `components/` — Feature-scoped components

Sub-components that are only used within this feature. A feature with a lot of markup breaks it into named steps or regions (e.g. `Sheet.astro`, `InstructionsStep.astro`, `PostSaveStep.astro`). These components receive all their data as props — they don't reach outside their feature.

#### `util/` — Client-side logic

TypeScript modules that run in the browser. Wired up via the `<script>` block in `index.astro`. Responsible for DOM interaction, state management, event handling, and calling into `api/submit.ts` when needed. Logic here is organized around a single `init` function that takes the feature's root DOM element.

---

## `src/constants/` — App-wide constants and enums

Holds things shared across features: environment-derived config (e.g. `FORM_SUBMISSION_URL`), enums (`ContactFormFields`), and the icon registry. Not a dumping ground — only truly cross-cutting values belong here.

## `src/util/` — App-wide utilities

Generic, stateless helper functions with no feature affiliation. Currently just `safelyGetEnvVar`.

## `src/styles/` — Global styles

`global.css` is imported once inside `Base.astro`. Feature-specific styles are handled via Tailwind classes inline, not separate stylesheets.
