# libportalfigure

This is a library of data relating to figures from the Skylanders series of video games such as IDs, and names.

## Disclaimer

This library does not intend to infringe on any copyright of the intellectual property holders. No code from any games, promotional materials, or data from NFC tags is contained in this library. All data is purely for reference use only.

## Backends

This library produces artifacts for consumption by other applications. Many different artifacts are produced in order to support as many development ecosystems as possible.

### JavaScript
A JavaScript package is emitted and published to npm for consumption by JavaScript projects. TypeScript definitions are included.

Installation:

```sh
npm install portal-figure
```

Code example:
```typescript
import { Spyro, findFigure, type PortalFigure } from 'portal-figure';

const figure: PortalFigure = findFigure(0x1ce, 0x3000)!;
console.log(`Spyro (figureId='${Spyro.figureId.toString(16)}') (variantId='${Spyro.variantId.toString(16)}')`);
console.log(`${figure.name} (figureId='${figure.figureId.toString(16)}') (variantId='${Spyro.variantId.toString(16)}')`)
```

### Rust
A Rust crate is emitted and published to crates.io for consumption by Rust projects.

Installation

```sh
cargo add portal_figure
```

Code example:
```rust
use portal_figure::{SPYRO, find_figure, PortalFigure};

fn main() {
    let figure: &PortalFigure = find_figure(0x1ce, 0x3000).expect("Couldn't find figure");
    println!(
        "Spyro (figure_id='0x{:x}') (variant_id='0x{:x}')",
        SPYRO.figure_id, SPYRO.variant_id
    );
    println!(
        "{} (figure_id='0x{:x}') (variant_id='0x{:x}')",
        figure.name, figure.figure_id, figure.variant_id
    );
}
```

### JSON
A standalone JSON file is emitted containing figure data for consumption by any development environment that can parse JSON.

### SQLite
A standalone pre-populated SQLite .db file is emitted contaning figure data for consumption by any development environment that supports reading from a SQLite database.

## Project status
This project is a work-in-progress and will be for a while. Development is straightforward but I am not focusing all of my time on this project. Pull Requests and Issues are welcome for data fixes, additional backends, unimplemented figure data, new metadata, etc.

Not all figure data that has been documented has been implemented yet. The data needs to be well-structured, and I have yet to properly categorise all data documented in [Texthead1/Skylander-IDs](https://github.com/Texthead1/Skylander-IDs). Unimplemented data lives in `src/00-unimplemented` and is _not_ emitted by the build process.

As it stands, data from character figures should largely be correct although Superchargers and Imaginators data is untested. Data from traps, magic items etc are likely documented by not yet "implemented" as I haven't yet studied how they work.

### Roadmap

Eventually this data will have a useful and accurate taxonomy of all data across all of the Skylanders games, including all figures and figure types, as well as any metadata that may be useful for application development (for example, images of each figure).

## Credits

 - [Texthead1/Skylander-IDs](https://github.com/Texthead1/Skylander-IDs) - Cataloging Skylander figure and variant IDs. This represents the majority of the work in creating this library.

## Backlog
  - C headers
  - Potentially more IDs
    - https://github.com/Texthead1/Custom-Mixed-Variant-Skylander-IDs
  - Set a license on this repo
  - Can we copy rustdoc from `FigureData` into emitted javascript, rust types?
    - Can we centralise "description" fields somewhere