# chunk-your-skills (C)

C integration for the root Rust crate (`ffi` feature) — SKILL.md pageindex and skinny-skill recomposition.

Links `libchunk_your_skills` / `chunk_your_skills.dll` generated from the repo-root crate.

## Prebuilt releases

Precompiled libraries for Linux, macOS, and Windows are attached to GitHub Releases as
`chunk-your-skills-ffi-<triplet>.tar.gz`.

```bash
VERSION=1.1.0
TRIPLET=x86_64-unknown-linux-gnu
curl -LO "https://github.com/qdrddr/chunk-your-skills/releases/download/v${VERSION}/chunk-your-skills-ffi-${TRIPLET}.tar.gz"
mkdir -p cys-ffi && tar -xzf "chunk-your-skills-ffi-${TRIPLET}.tar.gz" -C cys-ffi
gcc -std=c11 -o myapp main.c -I cys-ffi -L cys-ffi -lchunk_your_skills
```

## Build from source

```bash
./sdk/c/scripts/build-c-lib.sh
```

Copies the generated header to `sdk/c/include/chunk_your_skills.h`.

## CMake

```cmake
find_package(CYS REQUIRED)
target_link_libraries(myapp PRIVATE CYS::chunk_your_skills)
```

Include `chunk_your_skills.h` and call `cyt_build_skills_index`, etc.
