# 📚 Bookcase - An Arena Allocator

I wanted to learn arenas, so I implemented my own. Then I noticed the existing arena crates were not
to my liking.

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
- Minimal dependencies
- Documented

## 🚫 Concessions

- Unsafe implementation details
- Lookup is out of scope (this is not an ECS library)

## 🚀 Progress

In rough priority order:

- [ ] CI
- [ ] CD
- [x] Thread-local notebooks (`Personal*Notebook: Send`)
- [ ] Thread-safe notebooks (`Public*Notebook: Send + Sync`, needs more rigor)
- [x] Bump allocation (`Pen`)
- [ ] Deallocation
- [x] Compiles on stable rust
- [x] Publish first experimental version
- [ ] Publish first beta version
- [ ] Publish first stable version
- [x] Heterogeneous notebook (`MultiNotebook`)
- [x] Homogeneous notebook (`MonoNotebook`)
- [x] All allocations are aligned
- [ ] `MultiNotebook` is `Allocator` (requires nightly)
- [x] Configurable page base size (`SizeStrategy`)
- [x] Configurable page growth rate (`GrowthStrategy`)
- [x] Non-dropping exclusive references (`alloc*() -> &mut T`)
- [x] Auto-dropping handles (`new*() -> Handle<T>`)
- [ ] Notebook merging

## 🌳 Versioning

1. Backwards compatibility is falling asleep to the sound of ocean waves breaking on the beach.
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

This channel is for collecting well baked ideas that are preparing for stabilization.

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

### ... Maggie?

Maggie is my pet 🐷. She sucks at first, but you will eventually grow to love her.

### Exceptions

`bookcase_macros` is itself required to enforce the rules of `SemVer Maggie.1.0`. As such, it will
only have a stable release channel.

## Contribution

I created a [discord channel](https://discord.gg/VNjUtBh4UB). As of this writing it has a population
of me. Read the Goals, Concessions, and Progress sections for ideas on what to work on, and speak
with me about how to make changes I am likely to accept. You can also just give me feedback, tell me
my code is terrible, or that I'm being too edgy. All feedback is welcome!
