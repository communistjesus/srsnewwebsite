# SRS Design System

## TODO

- split out into pages
- add button page
- implement SRS website without individual blogs
- generalize into template that makes instantiating national website easy
- national website

Note: This is not to be used directly by any non developer, as this functionality will be exposed via markdown templates.

Half assed PRs will piss me off.

## Design Principles

This is with respect to technical design and the technical realization of any graphical design guidance.

Our mission is to propagandize for the seeds of a vanguard that will build the consciousness American working class to seize the means of production.

The revolutionary moment is currently far, so individual comrade capacity is nowhere near full-time, so capacity is limited.

All technical solutions must be "right-sized" for the current moment, but also as simple as possible. This generally means keeping code DRY as possible and using the latest useful technology.

All technical solutions must minimize busy work for all comrades

## Why? How does this fit into our digital infrastructure?

For interested parties, our websites are essentially templates. These give us the capability to trivially create new website instances, consistent styling across aforementioned instances and enable easy article creation for non technical folks via markdown.

There are additional benefits like having all of our articles versioned via git, enabling granular access control to the repository and automating style and grammar checks.

Authors smoothly make propaganda.

Developers can focus on building further infrastructure.

Editor work is streamlined.

### Specifics

Propagandists use accessible markdown editor to create propaganda

They submit this to the bot, which checks their grammar and writing style

Once this passes, editors can do their checks

The website uses `Zola`, which exposes the `tera` (jinja-like) template engine along with a CommonMarkdown-like interface for generating static websites. We use templates and `tailwindcss` via the `@apply` to "apply" tailwind classes and some custom styles to generated HTML elements.

The bot uses the `harper` grammar checker, accessible via pipeline "gate" and `harper-wasm` for the grammar enabled article editing UI. The discord bot uses `Rust` and `discord_sdk` for developer accessibility and consistency with other projects (all of `Zola`, `harper`, `tera`, `tailwindcss` use `Rust`).

#### Dependencies

You must install `zola` (via cargo) and `tailwindcss` (via nix). In the future, this will all be pushed into a `Dockerfile` and `podman-compose.yml` so that a rootless `podman` install is all that is required. This work should preserve the continuous watch/build tooling described below.

For basic developer flow, run `zola serve` in one terminal tab and `./scripts/build-styles.sh` in another terminal tab. These respectively watch and build templates, markdown and styles to enable rapid iteration. Visit [http://127.0.0.1:1111/](http://127.0.0.1:1111/) to see the website update as you edit files. I recommend you have the `build-styles` tab active as your browser will tell you where the non-style build fails.

TODO: document language servers/dev setup

#### Generating website

## Considerations

### Unknowns

- left link at bottom to more recent article, right link at bottom to less recent article

### SRS Design Guidance

main 1 color: black
main 2 color: #F2161E
accent color: white


#### buttons

resting state: white/black text on red background
hover: black on white
click: white on grey (#737373)

#### font

title font for titles, Anton (~5x size of body size, vary size for mobile)
subheading between title and body

subheading/accent is always all caps, only important information, poppins bold

body is poppins

#### general guidance

sharp angles

negative space

TODO: grab svgs