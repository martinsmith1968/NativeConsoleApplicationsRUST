# SKILL: Rust Algorithm Module Organization

**Category:** Module Design, CLI Tools  
**Difficulty:** Intermediate  
**Applicability:** Multi-algorithm tools, plugin-like architectures

## Overview

Pattern for organizing related algorithms into clean module structures in Rust, enabling independent testing, clear separation of concerns, and easy extensibility.

## Problem This Solves

When implementing multiple related algorithms (hashing, compression, encoding, etc.), keeping them inline in a single file creates:
- Code duplication of dispatch logic
- Difficult-to-test individual algorithms
- Unclear separation between orchestration and implementation
- Hard to onboard new algorithms

## Solution Pattern

### Step 1: Create Module Directory
```
src/bin/myapp/
  ├── main.rs
  └── algorithms/
      ├── mod.rs         (dispatcher & exports)
      ├── algo1.rs       (implementation)
      ├── algo2.rs       (implementation)
      └── algo3.rs       (implementation)
```

### Step 2: Establish Uniform Interface
Each algorithm exports the same public function signature:
```rust
// algorithms/sha256.rs
pub fn compute(data: &[u8]) -> Result<String, String> {
    // implementation
}
```

**Why:** Enables polymorphic dispatch via match expression or trait objects. Simplifies testing.

### Step 3: Module Exports & Dispatch
```rust
// algorithms/mod.rs
pub mod sha256;
pub mod md5;

pub use self::sha256::compute as sha256;
pub use self::md5::compute as md5;

pub fn compute_hash(data: &[u8], algo: &str) -> Result<String, String> {
    match algo {
        "sha256" => self::sha256(data),
        "md5" => self::md5(data),
        _ => Err("Unknown algorithm".to_string()),
    }
}
```

**Key points:**
- Re-exports reduce namespace pollution
- Match on `algo` string is exhaustive at compile time
- New algorithms added with minimal changes

### Step 4: Main Binary Uses Module
```rust
// main.rs
mod algorithms;

fn main() {
    let result = algorithms::compute_hash(data, user_algorithm)?;
    println!("{}", result);
}
```

Main file now focuses purely on CLI orchestration.

## Advantages

| Aspect | Benefit |
|--------|---------|
| **Testing** | Test each algorithm independently in its own file |
| **Maintenance** | One algorithm per file; easy to find/modify |
| **Clarity** | Readers understand one algorithm without context switching |
| **Extensibility** | New algorithm = new file + 3 lines in mod.rs |
| **Reusability** | Algorithms can be used as a library in other binaries |
| **CI/CD** | Can run algorithm-specific tests in parallel |

## Implementation Checklist

- [ ] Create `mod.rs` with exports
- [ ] Create one file per algorithm
- [ ] Establish uniform function signature across all algorithms
- [ ] Implement dispatcher with exhaustive match
- [ ] Move algorithm logic from main.rs to modules
- [ ] Run tests to verify backward compatibility
- [ ] Update main.rs to use module API
- [ ] Document the pattern in team decision log

## Real-World Example: hashcalc

**Before:** All 5 algorithms inline in main.rs (110+ lines embedded in 1031-line file)

**After:**
```
src/bin/hashcalc/
  ├── main.rs (70 lines: CLI only)
  └── hashers/
      ├── mod.rs (25 lines: dispatcher)
      ├── sha1.rs (9 lines)
      ├── md5.rs (5 lines)
      ├── sha256.rs (9 lines)
      ├── sha512.rs (9 lines)
      └── base64.rs (30 lines)
```

**Result:** ✓ 67 tests pass, ✓ zero regressions, ✓ code more maintainable and extensible

**Quality Review (Kiefer, Lead):**
- Uniform function signatures across all hashers verified
- Central dispatcher handles algorithm routing cleanly
- Error handling comprehensive (no `.unwrap()` calls)
- Build clean (zero warnings), all tests pass in debug and release modes
- Backward compatibility maintained—CLI interface unchanged
- Pattern approved as production-ready for similar multi-algorithm tools

## Common Pitfalls

❌ **Don't:**
- Export individual functions without re-exporting in mod.rs (forces verbose paths)
- Use different signatures across algorithms (complicates dispatch)
- Put algorithm tests in mod.rs (test each in its own file)
- Put CLI logic in algorithm modules (breaks reusability)

✅ **Do:**
- Keep algorithms orthogonal (no cross-algorithm dependencies)
- Use Result types for error handling across all algorithms
- Document the uniform interface at the top of mod.rs
- Consider trait objects if algorithms need shared methods beyond compute

## Extending the Pattern

### With Traits (Advanced)

For more structured dispatch:

```rust
// algorithms/mod.rs
pub trait Algorithm {
    fn compute(&self, data: &[u8]) -> Result<Vec<u8>, String>;
    fn name(&self) -> &'static str;
}

pub struct Sha256Algo;
impl Algorithm for Sha256Algo {
    fn compute(&self, data: &[u8]) -> Result<Vec<u8>, String> {
        // ...
    }
    fn name(&self) -> &'static str { "sha256" }
}

pub fn get_algorithm(name: &str) -> Result<Box<dyn Algorithm>, String> {
    match name {
        "sha256" => Ok(Box::new(Sha256Algo)),
        _ => Err("Unknown".into()),
    }
}
```

Use trait objects when:
- Algorithms share many common methods
- Need to pass algorithms around as values
- Implementing dynamic algorithm registration

## Related Patterns

- **Plugin Architecture** (load algorithms from .so files at runtime)
- **Strategy Pattern** (wrap algorithms in trait objects for runtime selection)
- **Factory Pattern** (centralized algorithm creation with metadata)
