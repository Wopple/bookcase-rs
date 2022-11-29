# 📚 Bookcase - An Arena Allocator

I wanted to learn arenas, so I implemented my own. Then I noticed the existing arena crates were not
to my liking.

## 🕐 Project Status

Experimental, do not use unless you are a 🤡.

## 📖 Glossary

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
- Documented

## 🚫 Concessions

- Unsafe implementation details
- Lookup is out of scope (this is not an ECS library)

## 🚀 Progress

In rough priority order:

- [ ] CI
- [ ] CD
- [ ] Thread-local notebooks
- [ ] Thread-safe notebooks (`Send` and `Sync`)
- [ ] Notebooks are `Allocator`
- [x] Bump allocation
- [ ] Deallocation
- [x] Typed notebook
- [x] Untyped notebook
- [x] Aligned allocation
- [x] Configurable page size
- [x] Configurable page growth
- [x] No drop
- [x] Auto drop
- [ ] Notebook merging

## 🌳 Versioning

1. Backwards compatibility is listening to ocean waves break on the beach.
2. *Assuming* backwards compatibility is torturing puppies.

**Conclusion: SemVer is 🐍🛢**

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

This channel is for collecting well baked ideas that want to be stabilized.

###### Experimental

This channel is the wild west where all bets are off.

#### Format: `stable.beta.experimental`

1. The version is in the experimental channel when `experimental > 0`, otherwise...
2. The version is in the beta channel when `beta > 0`, otherwise...
3. The version is in the stable channel when `stable > 0`, otherwise...
4. The version is invalid.

#### Examples

- `=1.0.0` - stable `v1`
- `=1.2.0` - beta `v2` based off stable `v1`
- `=1.2.3` - experimental `v3` based off beta `v2` based off stable `v1`
- `=2.0.1` - experimental `v1` based off stable `v2`
- `=0.0.0` - invalid

#### But I Like `bookcase = "^1"`!

Then get accustomed to compiler errors. A version within a release channel will only compile with
its respective feature enabled. The default feature is of course `stable`. Enabling more than one
will also fail to compile. This is to prevent accidental use of beta and experimental channels in
production user code.

## Contribution


