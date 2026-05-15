# bellforge — Arch Linux Kettlebell Training Timer & Obsidian Logger
**Implementation Plan for a native Arch Linux GUI application**

**Status**: Final — decisions locked  
**Target**: Built using Grok Build + the `arch-linux-gui-app-builder` persona (`agent.md`)  
**Primary Stack**: Rust + egui/eframe (confirmed by user)  
**App Name**: bellforge (binary: `bellforge`)  
**Date**: 2026-05-14

> **Locked User Decisions** (recorded 2026-05-14)
> - App name: **bellforge**
> - Audio: Always bundle `rodio` + embedded pleasant .ogg chime ("it just works")
> - Rest completion behavior: **Require explicit "Continue" confirmation** (safer, deliberate pacing)
> - Live weight editing: **Per-exercise override** supported during session
> - Template editor repeats (v1): **Simple top-level repeats only** + duplicate block (full nesting later)
> - Chime: Bundled high-quality short .ogg file (embedded)

---

## 1. Overview

**bellforge** is a focused, lightweight, native-feeling desktop application that helps users design kettlebell (and bodyweight) training templates, run live guided sessions with precise, configurable rest timers, and automatically export every completed session as a clean, queryable Markdown file with rich Obsidian-compatible YAML frontmatter.

The app is purpose-built for Arch Linux power users who train with kettlebells and maintain their training log inside an Obsidian vault. It follows "The Arch Way": minimal dependencies, excellent Wayland/HiDPI support, keyboard-first operation, trivial packaging via `makepkg` or AUR, and a beautiful but lightweight GUI that feels at home on GNOME, KDE, Hyprland, or any modern DE.

Core loop:
1. User creates or loads a **WorkoutTemplate** (flexible exercise flow + rests).
2. User starts a **Session**.
3. During the session the UI shows the current exercise + target reps in very large, high-contrast text. A prominent **Rest** button (or Space key) starts the appropriate countdown timer (different durations for intra-exercise vs inter-round rests, with per-exercise overrides).
4. Timer completion triggers a desktop notification + optional chime and advances to the next cue.
5. On session end (or early stop), the app writes a dated `.md` file into a user-specified Obsidian folder with proper frontmatter and a readable body.

---

## 2. Goals & Non-Goals

### Goals (v1 — shippable in 4–6 focused PRs)
- Define, save, duplicate, and edit flexible workout templates.
- Run a live guided session with two classes of rest timers (exercise vs round/set) + per-exercise overrides.
- Accurate, reliable countdown that survives window minimize / focus changes.
- Excellent during-workout UX: huge readable text, minimal clicks, full keyboard support, optional always-on-top mode.
- One-click (or auto) export of a completed session to Obsidian-flavored Markdown with rich frontmatter + body.
- Proper Arch Linux desktop integration: `.desktop`, scalable SVG icon, XDG paths, notifications, optional sleep inhibition, system tray quick-start.
- Clean `PKGBUILD` + `makepkg` instructions + AUR-ready metadata from day one.
- Zero-config first run (sensible defaults + 4–5 popular built-in templates: Simple & Sinister, Rite of Passage, etc.).

### Non-Goals (v1)
- No accounts, cloud sync, or mobile companion (desktop-first, local-only).
- No video embedding or form library (links to external resources are fine).
- No automatic rep counting via camera or sensors.
- No advanced periodization / programming engine (user designs the templates).
- No Flathub/AppImage/AppStream submission in v1 (PKGBUILD + AUR is the focus).

---

## 3. Confirmed Requirements from User

- **Workout structure**: Flexible / custom (not limited to simple round-robin or traditional per-exercise sets). Must support both complexes (different exercises repeated in rounds) and classic "3×8 with rest between sets of the same move".
- **Rest timers**: Both global defaults per template **and** per-exercise override capability.
- **During session**: Show exercise on screen + "rest button" that starts the timer. When timer finishes, next exercise appears. User controls when they finish the work portion by pressing Rest.
- **Export**: Markdown file written to a user-specified location (global setting + per-export override) containing proper Obsidian frontmatter (at minimum `date`, `workout_name`, `#kettlebell #workout` tags, plus detailed exercise data).
- **Tech**: egui + eframe (Rust) — chosen for custom high-quality timer UI, low resource usage, fast iteration, and excellent Arch Linux fit.
- **Polish items** (all of the above): system tray, notifications, sleep inhibit, global hotkeys, in-app history browser with basic stats, keyboard-centric operation.

---

## 4. Technology Stack & Arch Linux Rationale

**Primary**:
- **Rust** (2024 edition) + `egui 0.31` + `eframe` (wgpu backend by default, with glow fallback).
- Why egui wins here (user confirmed):
  - Immediate mode makes a beautiful custom timer (large text, circular progress arc, color state changes) trivial and extremely efficient (`ctx.request_repaint_after(Duration::from_millis(200))` during rest).
  - Tiny binary (5–12 MiB stripped), almost zero runtime dependencies on Arch.
  - First-class Wayland + fractional scaling + HiDPI.
  - Perfect for "tool" style apps (the persona ranks this highly for dashboards/timers).

**Key crates**:
- `serde`, `serde_json`, `toml` (templates human-editable)
- `chrono`, `uuid`
- `xdg` or `dirs-next` for XDG compliance
- `notify-rust` + `zbus` (desktop notifications + org.freedesktop.ScreenSaver inhibit)
- `rodio` (optional, feature-gated) for pleasant chime
- `tray-icon` + `winit` for system tray + global hotkeys (`global-hotkey` crate)
- `open` or `xdg-open` for "open in Obsidian / editor"

**Packaging** (non-negotiable per `agent.md`):
- `PKGBUILD` with proper `depends`, `makedepends`, `optdepends`
- `.desktop` file (validated)
- `bellforge.metainfo.xml` (AppStream)
- SVG icon in multiple sizes under `/usr/share/icons/hicolor/`
- `README.md` with exact `makepkg -si` and `yay -S bellforge` instructions
- `.SRCINFO` for AUR

**Why not the alternatives** (documented for future reference):
- Tauri: Rejected because WebKitGTK on Arch can be janky with animations/Wayland; heavier for a timer app.
- GTK4 + libadwaita: Excellent native feel, but more verbose for the custom painter-heavy timer UI we need. Easy migration path later if desired.
- Iced + libcosmic: Attractive, but smaller ecosystem and less mature wgpu story on all DEs today.

---

## 5. Data Models (Rust + Serde)

All persisted data lives under XDG directories and is plain JSON (or TOML for templates if preferred for hand-editing).

```rust
// Core domain objects (in src/workout.rs)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct WorkoutTemplate {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub tags: Vec<String>,                    // e.g. ["kettlebell", "strength", "complex"]
    pub default_weight_kg: Option<f32>,

    // Global rest defaults (seconds)
    pub rest_between_exercises_s: u32,
    pub rest_between_rounds_s: u32,

    /// The flexible flow definition
    pub flow: Vec<FlowItem>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum FlowItem {
    /// A single exercise that may be performed for multiple sets
    Exercise {
        name: String,
        reps: u32,
        weight_kg: Option<f32>,
        sets: u32,                    // 1 = single set, >1 = traditional sets
        rest_after_set_s: Option<u32>, // override for rest between sets of *this* exercise
        rest_after_exercise_s: Option<u32>, // override for rest after finishing all sets of this exercise
    },
    /// Explicit rest (used for custom placement)
    Rest {
        duration_s: u32,
        label: String,                // "Active recovery", "Round rest", etc.
    },
    /// Repeat a block of previous items (supports both styles elegantly)
    Repeat {
        count: u32,
        items: Vec<FlowItem>,
    },
}
```

**Compiled execution view (runtime only)**:
The runner never persists the compiled form. On session start we compile the `WorkoutTemplate` into a flat `Vec<SessionCue>` plus loop-stack metadata so the UI can always answer "what is step 7 of 24?" and "which round am I on?".

```rust
pub struct SessionCue {
    pub round: u32,
    pub step_in_round: u32,
    pub total_steps: u32,
    pub action: CueAction,
}

pub enum CueAction {
    PerformExercise { name: String, target_reps: u32, weight_kg: Option<f32> },
    Rest { duration_s: u32, label: String },
}
```

**Session & Log**:
```rust
pub struct CompletedSession {
    pub id: Uuid,
    pub started_at: DateTime<Utc>,
    pub finished_at: DateTime<Utc>,
    pub template_id: Uuid,
    pub template_name: String,
    pub rounds_planned: u32,
    pub rounds_completed: u32,
    pub performed: Vec<PerformedExercise>,
    pub user_notes: String,
    pub rpe: Option<u8>,                 // 1-10 Rate of Perceived Exertion
    pub total_duration_min: u32,
}

pub struct PerformedExercise {
    pub name: String,
    pub weight_kg: Option<f32>,
    pub target_reps: u32,
    pub actual_reps: u32,                // user can edit during/after
    pub sets_completed: u32,
    pub rest_used_s: u32,
}
```

Templates are saved as individual `.json` files in `$XDG_DATA_HOME/bellforge/templates/`.

---

## 6. Architecture & Key Components

```
src/
├── main.rs                 # eframe entry, single instance handling
├── app.rs                  # top-level App state machine + routing between views
├── config.rs               # Settings (output_dir, default rests, audio enabled, etc.)
├── workout.rs              # models + compilation + validation
├── session.rs              # SessionRunner + RestTimer (the heart)
├── timer.rs                # precise RestTimer (Instant based, pause/resume)
├── export.rs               # to_obsidian_markdown(...) -> String
├── persistence.rs          # load/save templates, recent sessions, settings (XDG)
├── ui/
│   ├── dashboard.rs        # library + quick start
│   ├── template_editor.rs  # drag-and-drop flow builder, inline editing, visual rest indicators
│   ├── session_view.rs     # the big live workout screen (state-driven)
│   ├── history.rs          # scan log dir, parse frontmatter, show table + stats
│   └── settings.rs
├── sound.rs                # feature("audio"): rodio simple sine or ogg
├── desktop.rs              # notifications, inhibit sleep, tray, global hotkeys
└── icon.rs                 # embedded SVG or bytes for window icon
```

**State machine in `App`** (egui `App::update`):
- `View::Dashboard`
- `View::TemplateEditor { template: WorkoutTemplate, dirty: bool }`
- `View::ActiveSession { session: SessionRunner, timer: RestTimer }`
- `View::PostSession { summary: CompletedSession, exported_path: Option<PathBuf> }`
- `View::History`
- `View::Settings`

The `SessionRunner` owns the compiled cue list and current index + actual performance log. It exposes:
- `current_cue() -> Option<&SessionCue>`
- `start_rest(override_duration: Option<u32>)`
- `advance()`
- `toggle_pause()`
- `log_actual_reps(delta: i32)` (big + / − buttons or number editor)

**RestTimer** is deliberately simple and correct:
- Stores `target_duration`, `start_instant: Instant`, `paused_remaining: Option<Duration>`
- `remaining(&self) -> Duration` always computed from wall-clock `Instant::now()` — never trusts tick count.
- When it hits zero it emits a one-shot event (or the UI just sees `<= zero` on next frame).

---

## 7. User Interface & Experience

### 7.1 Dashboard / Library
- Grid or list of saved templates (name, #exercises, rounds, last used).
- Big "Start Workout" primary action.
- "New Template", "Duplicate", "Edit", "Delete".
- Quick-start presets row (S&S 100 swings, etc.).

### 7.2 Template Editor (most important setup screen)
- Clean table or card list:
  - Reorderable rows (drag or buttons)
  - Columns: Order | Action (Exercise / Rest / Repeat block) | Details | Rest after | Weight
- Global defaults panel at top: "Default rest between exercises: ___s" and "Default rest between rounds: ___s". Per-item fields override when non-empty.
- Live "Estimated total time" calculation shown while editing.
- "Add Exercise", "Add Rest", "Wrap last N items in Repeat" helpers.
- "Save as new" vs "Update existing".

### 7.3 Active Session View (the star of the app)
**Design goals**: glanceable from 2 meters away, minimal cognitive load, sweaty-hands friendly.

Layout (full height, generous padding):
- Top bar: "Kettlebell Complex — Round 2/5" + elapsed session time + "Pause" | "Abort"
- **Huge centered exercise name** (egui `RichText::size(64.0).strong()` or larger; color = work vs rest state)
- Large target: "8 reps × 24 kg"
- **Massive countdown** (when resting): 01:17 in 120pt font + circular progress arc painted with `egui::Painter` (orange → yellow → green)
- Big friendly button row:
  - **REST / COMPLETE** (primary, Space key, big green/orange depending on state)
  - +1 / −1 actual reps (always visible, large)
  - "Skip rest" (dangerous but useful)
- Bottom strip: horizontal progress of the entire compiled cue list (tiny squares or thin bar with current position highlighted). Click any past step to jump back (optional power-user feature).
- Keyboard everywhere: Space = rest/complete, j/k = adjust reps, p = pause, Esc = confirm abort.

**States visually**:
- Exercising: green accents, "Do the work" subtle text
- Resting: warm orange, big timer, "Next up: Two-Hand Swing — 10 reps" preview
- Finished round: celebratory flash + longer rest

When rest timer hits zero:
- Play chime (if enabled)
- Show system notification: "Rest complete — Two-Hand Swing 10×24kg"
- If "auto-advance" setting on: immediately show next exercise
- Else: large "Continue" button (prevents accidental start of next set while user is still breathing)

### 7.4 Post-Session & Export
- Nice summary: total time, volume estimate, rounds completed, RPE selector (slider or 1–10 buttons)
- Large text area for free-form notes
- "Export to Obsidian" primary button (opens native directory picker on first use, remembers path)
- Also "Save without exporting" and "Discard"
- After export: "Opened in Obsidian" or "File written to: ~/vault/logs/2026-05-14_kettlebell-complex.md" + "Open file" button

---

## 8. Obsidian Markdown Export Format

Example output file: `2026-05-14_kettlebell-sinister.md`

```markdown
---
date: 2026-05-14
workout_name: "Simple & Sinister"
tags:
  - kettlebell
  - workout
  - strength
  - #simpleandsinister
started: 2026-05-14T18:42:00+02:00
finished: 2026-05-14T19:11:00+02:00
duration_min: 29
rounds_planned: 10
rounds_completed: 10
rpe: 8
total_volume_kg: 2400
notes: "Swings felt crisp. Got the 100th in 4:40 today."
exercises:
  - name: "One-Arm Swing"
    weight_kg: 32
    sets: 10
    target_reps_per_set: 10
    actual_reps_total: 100
  - name: "Turkish Get-Up"
    weight_kg: 32
    sets: 1
    target_reps_per_set: 1
    actual_reps_total: 10
---

# Simple & Sinister — 2026-05-14

**Duration**: 29 minutes  
**RPE**: 8/10  
**Volume**: 2400 kg

## Performed

- **One-Arm Swing** — 10 × 10 reps @ 32 kg (actual 100)
- **Turkish Get-Up** — 10 × 1 rep @ 32 kg (actual 10)

## Notes
Swings felt crisp. Got the 100th in 4:40 today. Left shoulder a little tight on last TGU — will mobilize tonight.

---
*Logged by bellforge on Arch Linux*
```

The exporter is deterministic and pretty. Frontmatter is always valid YAML. The `exercises` array + `total_volume_kg` makes excellent Dataview / Tasks / Kanban queries possible later.

User can edit the generated file freely — the app never touches it again.

---

## 9. Desktop Integration (Arch-specific)

- **System tray**: "Start Quick Session", "Resume last template", "Open bellforge", "Quit". Uses `tray-icon`.
- **Global hotkeys** (optional, behind feature): `Super+Shift+K` → start rest timer from anywhere (great when music is loud).
- **Notifications**: `notify-rust` with action buttons ("Skip rest", "Add 2 reps").
- **Sleep inhibition**: During active session, call `org.freedesktop.ScreenSaver.Inhibit` (and `org.freedesktop.login1` for lid close) so the machine doesn't sleep mid-workout. Release on session end or explicit pause.
- **XDG**:
  - Config: `~/.config/bellforge/config.toml`
  - Data: `~/.local/share/bellforge/templates/`
  - Logs default suggestion: `~/Documents/bellforge-logs/` or user picks their Obsidian vault subfolder
- **Icon**: Professional kettlebell SVG (bell + handle, bold modern line style) in hicolor theme. Window icon embedded.

---

## 10. Implementation Phases / PR Plan (Incremental & Reviewable)

Each PR must be independently buildable, run on a clean Arch box, and leave the app in a usable state.

**PR 1 — Foundation & Packaging**
- Cargo workspace, `eframe` "Hello bellforge" dark window with custom titlebar feel
- XDG paths + basic `Config` persistence (ron or toml)
- `WorkoutTemplate` + `FlowItem` models + serde + unit tests
- Skeleton `PKGBUILD`, `bellforge.desktop`, placeholder SVG icon, `metainfo.xml`
- `README.md` with exact build & run instructions for Arch users
- `agent.md` rules already present — we follow them

**PR 2 — Template Editor (MVP)**
- Full UI for creating/editing/saving/deleting templates
- Add Exercise, Add Rest, Reorder, Delete row
- Global rest defaults + per-exercise rest override fields
- Live "estimated session time" calculator
- 4–5 built-in example templates (including Simple & Sinister and a classic "3×5 Press + 3×8 Row" style)
- Drag-and-drop or up/down buttons + visual "Repeat block" helper (simple version first)

**PR 3 — Session Runner Core**
- Compile `WorkoutTemplate` → `Vec<SessionCue>` engine (handles `Repeat` by unrolling or using a cursor + stack)
- `RestTimer` implementation (correct wall-time math, pause/resume, remaining)
- Basic `ActiveSession` view showing current cue
- "REST" button that starts the correct rest duration (global or override)
- Auto-advance or manual continue when timer hits 0
- Actual reps editor (+1/−1 and direct input)

**PR 4 — Polish During-Workout UX**
- Huge typography + custom painted circular timer + color theming (work vs rest)
- Keyboard shortcuts (Space, arrows, digits for reps, P, Esc)
- Next-up preview + full sequence progress strip
- Pause entire session (timer stops, state saved)
- Abort with "partial log?" prompt
- Basic always-on-top + "focus mode" (hide chrome)

**PR 5 — Export + Obsidian Frontmatter**
- `export.rs` with excellent `to_obsidian_markdown` function + tests
- Post-session summary view with RPE, notes, volume calc
- Directory picker + persistent "last used export path"
- Write file with safe filename (`YYYY-MM-DD_slugified-name.md`)
- "Open in editor / Obsidian" button using `xdg-open`

**PR 6 — History, Stats & Settings**
- History view: scan the user's chosen log directory, parse frontmatter of all `.md` files, show sortable table (date, workout, duration, RPE, volume)
- Simple stats panel (monthly volume, most frequent exercise, longest session, etc.) using `egui-plot` if needed
- Settings dialog: default log path, audio on/off + volume, auto-advance behavior, notification preferences, theme accent color

**PR 7 — Desktop Integration & Sound**
- System tray + menu (using `tray-icon`)
- `notify-rust` notifications on rest complete + session finish
- Optional `rodio` chime (nice "ding" or soft bell — fallback to `canberra-gtk-play` or `paplay` if feature disabled)
- Sleep inhibition via zbus/dbus during active sessions
- Optional global hotkey registration (feature flag, `global-hotkey` crate)

**PR 8 — Packaging, Icon, Documentation & AUR Readiness**
- Final beautiful SVG icon + multiple PNG sizes
- `desktop-file-validate` clean `.desktop`
- `PKGBUILD` with proper `optdepends=( 'obsidian' 'kitty' 'alacritty' )`, post-install hooks for icon cache + desktop database
- `.SRCINFO`
- Comprehensive `README.md` (screenshots, troubleshooting Wayland, "how to contribute a new built-in template")
- End-to-end test on a clean Arch container or `distrobox` (documented)
- Release v0.1.0 tag + AUR submission instructions

**Stretch / v1.1 (after user feedback)**:
- Full recursive `Repeat` blocks with nesting in the editor
- Per-cue notes during session
- Volume charts in History
- Export to CSV/Strong/Hevy JSON
- Single-instance dbus activation

---

## 11. Key Decisions & Rationale

1. **egui chosen over Tauri/GTK** — Custom timer UI is 60% of the perceived quality. Immediate mode + `Painter` gives pixel-perfect control with almost no code. Resource usage and binary size are ideal for a "leave it running in the corner" tool on Arch.

2. **Flexible `FlowItem` with `Repeat` (even if v1 limits nesting)** — This single abstraction elegantly covers "10 rounds of (Swing + Clean + Press + Squat)" and "5 sets of Press with 90s between sets" without two different editors. The compiler step keeps the runner simple.

3. **Wall-clock `Instant`-based timer, not tick-based** — Survives suspend, lag spikes, window being iconified, etc. Correctness first for a training tool.

4. **Write-only Markdown export, never a database** — The user's Obsidian vault is the source of truth. bellforge is just a very good input device + timer. This matches how Arch users actually work.

5. **Packaging is a first-class deliverable** — Every PR must not regress the ability to `makepkg -si` on a fresh Arch install. The persona's #1 rule.

6. **Actual reps are always editable** — Real life happens. User may grind out 7 instead of 8 or do an extra set. The log must reflect reality, not the plan.

---

## 12. Risks & Mitigations

| Risk | Severity | Mitigation |
|------|----------|------------|
| egui styling feels "not native enough" on GNOME | Medium | Provide excellent dark "gym" theme + accent colors that match Adwaita. Offer a libadwaita port later as a separate binary (`bellforge-adw`). |
| Complex `Repeat` logic has off-by-one bugs in round counting | High | Heavy unit + property testing on the cue compiler. Start with non-nested Repeats in PR 2–3, add nesting only after solid tests. |
| Timer appears to "drift" after suspend/resume | Medium | Always compute from `Instant::now()`. On resume detect large jump and show warning banner "Session paused by system sleep — timer may be inaccurate". |
| New Arch users hit missing Vulkan / wgpu issues | Low | eframe gracefully falls back to `glow` (OpenGL). Document `WGPU_BACKEND=gl` env var. |
| User loses work if app crashes mid-session | Medium | Autosave `partial_session.json` every 10 seconds + on every state change. Offer "Resume interrupted session" on next launch. |

---

## 13. Resolved Decisions (Locked 2026-05-14)

All open questions have been answered by the user:

1. **Repeat nesting depth in v1 editor** → **Simple top-level repeats + "duplicate block" for v1**. Full visual nesting deferred to a later release. (Pragmatic, faster to ship reliably.)
2. **Auto-advance default** → **Require explicit "Continue" confirmation** after the rest timer hits zero. Safer and more deliberate for real training sessions. (Still configurable later.)
3. **Weight handling** → **Per-exercise override** supported in the live session UI. User can change the weight used for a specific exercise/set on the fly; the actual value is recorded in the exported Markdown.
4. **Chime sound** → **Bundled pleasant short .ogg file**, embedded via `include_bytes!`. Professional, consistent auditory cue (~150-300 KB binary cost). rodio is always included.
5. **App name** → **bellforge** (binary `bellforge`, AUR package `bellforge`, config dir `bellforge`).
6. **Audio packaging** → **Always bundle rodio + chime** in the main package ("it just works"). No feature flag needed. `alsa-lib` will be a `depends` in the PKGBUILD.

These decisions are now reflected throughout the plan and should be followed during implementation.

---

## 14. References & Prior Art

- Arch Wiki: [PKGBUILD](https://wiki.archlinux.org/title/PKGBUILD), [Desktop entries](https://wiki.archlinux.org/title/Desktop_entries), [XDG Base Directory](https://wiki.archlinux.org/title/XDG_Base_Directory)
- egui official examples: timer, custom painting, drag-and-drop
- Popular Rust timer apps: `pomodorust`, `egui_pomodoro` repos on GitHub
- Obsidian fitness logging community templates (r/ObsidianMD, r/kettlebell)
- `notify-rust`, `zbus`, `tray-icon`, `rodio` crate docs
- Simple & Sinister / Rite of Passage program details (for built-in templates)

---

## 15. Next Steps After Plan Approval

1. User approves this plan (or provides targeted feedback).
2. Create a new git repo (or monorepo under Projects) with the `agent.md` copied in.
3. Start with **PR 1** using Grok Build: "Implement PR 1 of the bellforge design plan at [path to this plan.md]. Follow the arch-linux-gui-app-builder persona strictly."
4. Iterate PR by PR, using the reviewer persona or `/check` after each significant chunk.

This plan is intentionally detailed enough that an experienced Rust + egui engineer (or Grok following the persona) can implement each PR with high confidence and minimal clarification.

---

**End of Plan**

*Written in plan mode for the Arch Linux GUI App Builder project. Ready for exit and user approval.*
