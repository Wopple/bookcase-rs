# 📚 Bookcase - An Arena Allocator

I wanted to learn arenas, so I implemented my own. Then I noticed the existing arena crates were not
to my liking.

## Project Status

Experimental, do not use unless you are a 🤡.

## Goals

- Thread safe
- Safe interface
- Ergonomic
- Fast
- Lean
- Configurable
- Building block for data structures
- Documented

## Concessions

- Unsafe implementation
- Data structure implementations are out of scope (e.g. ECS store)

## Progress

- [ ] CI
- [ ] CD
- [ ] Arenas are confidently `Send`
- [ ] Arenas are confidently `Sync`
- [ ] Arenas are `Allocator`
- [x] Bump allocation
- [ ] Deallocation
- [x] Typed arena
- [x] Untyped arena
- [x] Aligned allocation
- [x] Configurable page size
- [x] Configurable page growth
- [x] No drop
- [x] Auto drop

## Versioning

1. Backwards compatibility is fantastic.
2. *Assuming* backwards compatibility is puppy murder.

**Conclusion: SemVer is 🐍🛢**

Unfortunately, cargo is tied to SemVer. Fortunately, SemVer is versioned. This means I can create my
own version of SemVer! 😈 Call it `SemVer Maggie.1.0`. Here's how it works:

### The Ironclad Rule

Assume all versions are breaking.

### The Motto

Strive to keep breaking changes to an absolute minimum.

### Release Channels

Each dot separated number represents a release channel:

###### Stable - exclusively suitable for production

###### Beta - trying to find stability

###### Experimental - all bets are off

#### Format: `stable.beta.experimental`

1. The version is in the experimental channel when `experimental > 0`, otherwise...
2. The version is in the beta channel when `beta > 0`, otherwise...
3. The version is in the stable channel when `stable > 0`, otherwise...
4. The version is invalid.

#### Examples

- `=1.0.0` - stable `v1`
- `=1.2.0` - beta `v2` based off stable `v1`
- `=1.2.3` - experimental `v3` based off beta `v2` based off stable `v1`
- `=1.0.2` - experimental `v2` based off stable `v1`
- `=0.0.0` - invalid

#### But I Like `bookcase = "^1"`!

Then get accustomed to compiler errors. A version within a release channel will only compile with
its respective feature enabled. The default feature enabled is of course `stable`. This is to
prevent accidental use of beta and experimental channels in production user code.
