[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://github.com/Wopple/bookcase-rs/blob/main/LICENSE.md)
[![CI](https://github.com/Wopple/bookcase-rs/actions/workflows/ci.yaml/badge.svg?branch=main)](https://github.com/Wopple/bookcase-rs/actions/workflows/ci.yaml)

# 📚 Bookcase - An Arena Allocator

I wanted to learn arenas, so I implemented my own. Then I noticed the existing arena crates were not
to my liking. So I created this project.

## 🕐 Project Status

Experimental, do not use unless you are a 🤡.

## 📖 Glossary

- Person: thread
- Notebook: arena
- Chapter: set of commonly aligned pages
- Page: fixed sized buffer of raw bytes
- Utensil: implementation of allocation

Notebooks start out blank. A person can write into a notebook. Depending on if they use pencil or
pen, they may or may not be able to erase their notes. A person can pass a notebook to another
person. Any person can write in the notebook, but only one at a time. Any number of people can read
from a notebook at the same time.

This analogy is not perfect, but it's way better than what 'arena' has to offer! 😁

## 🎯 Goals

- Thread safe
- Safe interface
- Ergonomic
- Fast
- Lean
- Configurable
- Building block for data structures
- No external dependencies
- Documented

## 🚫 Concessions

- Unsafe implementation details
- Lookup is out of scope (this is not an ECS library)

## 🚀 Progress

In rough priority order:

- [ ] CI
  - [x] Stable Tests
  - [x] Nightly Tests
  - [ ] Channel aware
- [ ] CD
  - [ ] Publish experimental versions
  - [ ] Publish beta versions
  - [ ] Publish stable versions
- [x] No dependencies
- [ ] Well documented
- [x] Thread-local notebooks
  - `Personal*Notebook: Send`
- [ ] Thread-safe notebooks
  - `Public*Notebook: Send + Sync`
  - [x] Implemented
  - [ ] Thread-safety ensured
- [x] Bump allocation
  - `Pen: Utensil`
- [ ] Deallocation
  - `Pencil: Utensil`
- [x] Compiles on stable rust
- [x] Publish first experimental version
- [ ] Publish first beta version
- [ ] Publish first stable version
- [x] Heterogeneous notebook
  - `*MultiNotebook: Notebook`
- [x] Homogeneous notebook
  - `*MonoNotebook<T>: TypedNotebook<T>`
- [x] All allocations are aligned
  - `*MultiNotebook: Allocator`
  - Requires nightly
- [x] Configurable base size of page
  - `SizeStrategy`
- [x] Configurable growth rate of page
  - `GrowthStrategy`
- [x] Non-dropping exclusive references
  - `alloc*() -> &mut T`
- [x] Auto-dropping handles
  - `new*() -> Handle<T>`
- [ ] Notebook merging

## 🌳 Versioning

### Philosophy

1. Backwards compatibility is falling asleep to the sound of ocean waves breaking on the beach.
2. *Assuming* backwards compatibility is torturing puppies.

**Conclusion: SemVer is 🐍🛢**

### Consequence

Unfortunately, cargo is tied to SemVer. Fortunately, SemVer is versioned. This means I can create my
own version of SemVer! 😈 Call it `SemVer Maggie.1.0`. Here's how it works:

### The Ironclad Rule

Assume all versions are breaking.

### The Motto

Strive to keep breaking changes to an absolute minimum.

### Release Channels

Each dot separated number represents a release channel:

###### Stable

This channel is the only one suitable for production use.

###### Beta

This channel is for collecting well baked ideas that are preparing for stabilization.

###### Experimental

This channel is the wild west where all bets are off and clown behavior is the norm.

#### Format: `stable.beta.experimental`

1. The version is in the experimental channel when `experimental > 0`, otherwise...
2. The version is in the beta channel when `beta > 0`, otherwise...
3. The version is in the stable channel when `stable > 0`, otherwise...
4. The version is invalid.

Therefore all stable releases are a bump to the "major" version suggesting to the downstream
developer they should consider The Ironclad Rule even if they are unfamiliar
with `SemVer Maggie.1.0`.

#### Examples

- `=1.0.0` - stable `v1`
- `=1.2.0` - beta `v2` based off stable `v1`
- `=1.2.3` - experimental `v3` based off beta `v2` based off stable `v1`
- `=2.0.1` - experimental `v1` based off stable `v2`
- `=0.0.0` - invalid

#### But I Like `bookcase_alloc = "^1"`!

Then get accustomed to compiler errors. A version within a release channel will only compile with
its respective feature enabled. The default feature is of course `stable`. Enabling more than one
will also fail to compile. This is to prevent accidental use of beta and experimental channels in
production user code.

### ... Maggie?

Maggie is my pet 🐷. She sucks at first, but you will eventually grow to love her.

### Exceptions

`bookcase_alloc_macros` is itself required to enforce the rules of `SemVer Maggie.1.0`. As such, it
will only have a stable release channel.

## Contribution

I created a [discord channel](https://discord.gg/VNjUtBh4UB). As of this writing it has a population
of me. Read the Goals, Concessions, and Progress sections for ideas on what to work on, and speak
with me about how to make changes I am likely to accept. You can also just ask questions and give me
feedback: feature requests, tell me my code is terrible or that I'm being too edgy. All feedback is
welcome!
