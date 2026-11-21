# Rust Lab

Ruta de estudio de Rust: del lenguaje a estructuras de datos y algoritmos, concurrencia, async y sistemas.
Los proyectos de backend (Fases 11-13) y los mini-proyectos grandes viven en repos aparte.

## Mapa: fase → crate

| Fase | Tema | Crate |
|---|---|---|
| 1-4 | Lenguaje, ownership, tipos, errores, módulos, tests | `lang` |
| 5 | Estructuras de datos y algoritmos | `dsa` |
| 1, 5 | LeetCode, Codewars, Codeforces, Advent of Code | `practice` |
| 7 | Concurrencia y paralelismo | `concurrency` |
| 8 | Async y Tokio | `async_tokio` |
| 9 | Sistemas y redes | `systems` |
| 10 | Serde, rendimiento, observabilidad, calidad | `ecosystem` |
| 1-5 | Mini-proyectos pequeños (un crate cada uno) | `projects/` |

## Uso

```bash
cargo test -p dsa                 # tests de un crate
cargo test two_sum                # un ejercicio por nombre
cargo test -- --nocapture         # ver los println!
cargo run -p lang --example calculator
python3 scripts/gen_mods.py       # registra archivos/carpetas nuevos
```

## Agregar un ejercicio

1. Crea el archivo, por ejemplo `practice/src/leetcode/easy/p0001_two_sum.rs`, con la función y su `#[cfg(test)]`.
2. Ejecuta `python3 scripts/gen_mods.py` (genera los `pub mod`).
3. `cargo test p0001`.

## Progreso

- [ ] Fase 1 · [ ] Fase 2 · [ ] Fase 3 · [ ] Fase 4 · [ ] Fase 5
- [ ] Fase 7 · [ ] Fase 8 · [ ] Fase 9 · [ ] Fase 10
