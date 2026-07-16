# chunk-your-skills (TypeScript)

TypeScript/Node bindings for the [chunk-your-skills](https://crates.io/crates/chunk-your-skills) Rust crate.

```bash
cd sdk/typescript
npm ci
npm run build
npm test
```

```typescript
import {
  buildSkillsIndex,
  defaultPageIndexConfig,
} from "chunk-your-skills";

const index = buildSkillsIndex(["/path/to/skills"], defaultPageIndexConfig());
console.log(Object.keys(index.documents));
```

Native `.node` binaries are built from the root crate (`../../Cargo.toml`) and published in the npm package.
