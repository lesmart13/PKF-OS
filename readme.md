# 🌴 Paradise Kumul Fly & Wanbel

Welcome to the beginning of a visionary operating system and next-generation programming language.

---

## 🧑‍💻 Wanbel — First Line of Code

```wanbel
say "Hello, world from Paradise Kumul Fly!"
```

### Language Philosophy
- **Natural syntax**: intent-driven, clean, no noise.
- **Safe by default**: memory safety, thread safety, and sandboxed execution.
- **Built for the future**: AI-native, distributed by design, no build system required.

### Compiler Bootstrap Plan
- Phase 1: Parser and Interpreter in Rust
- Phase 2: Type system and bytecode compiler
- Phase 3: Native Wanbel-to-Wanbel compiler (self-hosted)
- Phase 4: Integration with PKF kernel APIs and agent Kai

### Phase 1: Draft `wanbelc` Compiler in Rust

File structure:
```
wanbelc/
├── src/
│   ├── main.rs
│   ├── lexer.rs
│   ├── parser.rs
│   ├── ast.rs
│   ├── interpreter.rs
│   └── repl.rs
├── Cargo.toml
```

#### `main.rs`
```rust
mod lexer;
mod parser;
mod ast;
mod interpreter;
mod repl;

fn main() {
    repl::start();
}
```

#### `repl.rs`
```rust
use std::io::{self, Write};
use crate::parser::parse;
use crate::interpreter::eval;

pub fn start() {
    loop {
        print!("wanbel> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match parse(&input) {
            Ok(ast) => eval(ast),
            Err(e) => eprintln!("Parse error: {}", e),
        }
    }
}
```

#### `ast.rs`
```rust
#[derive(Debug)]
pub enum Expr {
    Say(String),
}
```

#### `parser.rs`
```rust
use crate::ast::Expr;

pub fn parse(input: &str) -> Result<Expr, String> {
    let input = input.trim();
    if input.starts_with("say ") {
        let quoted = input[4..].trim_matches('"');
        Ok(Expr::Say(quoted.to_string()))
    } else {
        Err("Unknown command".into())
    }
}
```

#### `interpreter.rs`
```rust
use crate::ast::Expr;

pub fn eval(expr: Expr) {
    match expr {
        Expr::Say(msg) => println!("{}", msg),
    }
}
```

---

## 🛠 Paradise Kumul Fly — Kernel Roadmap

### 🌱 Phase 1 — Foundation (v0.1 - v0.5)
- [ ] Rust-based microkernel
- [ ] Minimal scheduler and IPC
- [ ] Virtual file system abstraction
- [ ] Bootable ISO with simple shell interface

### 🛠 Phase 2 — System Layer (v0.5 - v1.0)
- [ ] Sandboxed services (userland modules)
- [ ] Capsule system for apps (no install needed)
- [ ] Secure driver abstraction
- [ ] Contract-based module interfaces

### 🌐 Phase 3 — Distributed OS (v1.0 - v2.0)
- [ ] Remote memory, file, and task handling
- [ ] Seamless multi-device communication
- [ ] Distributed process migration
- [ ] Cloud-first sync and config engine

### 🧠 Phase 4 — Intent OS (v2.0+)
- [ ] Intent compiler (goal → system actions)
- [ ] AI agent Kai for task orchestration
- [ ] Ambient input: voice, gesture, context
- [ ] Self-healing service mesh with rollback

---

## 🔗 Next Steps

### GitHub Repos
- `Paradise-Kumul-Fly/Kernel` — Microkernel and system services
- `Paradise-Kumul-Fly/Wanbel` — Language parser, compiler, stdlib

### Kickoff To-Do
- [ ] Create GitHub org + repo structure
- [ ] Bootstrap `wanbelc` (compiler) in Rust
- [ ] Setup `pkf_kernel` as a buildable project
- [ ] Define OS interface DSL for Wanbel
- [ ] Sketch logo and UI shell design (optional visual mockups)

---

**Let’s build the future.**
