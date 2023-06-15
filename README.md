# Symono

![Workflow - Rust](https://github.com/TWoolhouse/symono/actions/workflows/rust.yml/badge.svg)

A custom maths markup language and parser.
Try it out at: https://woolhouse.uk/symono/

The langauge is meant to be simpler and more intuitive than [LaTeX](https://www.latex-project.org/) by making use of symbols rather than a command system.

## Syntax

The syntax is defined in the [`symono.pest`](src/symono.pest) file, a PEG style grammar langauge.

```symono
@alpha => n {i |N {| { -1 }
```

```latex
$\alpha \Rightarrow n \in \mathbb{N} \cup \set{ -1 }$
```

## Transpiling

The language can be transpiled into [LaTeX](https://www.latex-project.org/) with support for [MathML](https://developer.mozilla.org/en-US/docs/Web/MathML) coming soon!
