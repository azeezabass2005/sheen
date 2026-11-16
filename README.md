# Sheen

Sheen is a terminal-native Markdown reader that renders Mermaid diagrams
as real terminal output: Unicode box-drawing characters and ANSI color,
in your own terminal font and background.

> Early stage: nothing is implemented yet. This README describes what
> Sheen is being built to do.

### Why Sheen?

Many developers keep architecture docs and ADRs in Markdown. Most terminal
readers show the Mermaid diagrams in those docs as plain code. Tools like
`glowm` that do render them spin up a headless browser, render the diagram
and screenshot it. That makes reading a short file take seconds, and it
only works in terminals that support image protocols.

### Features

- **Layout engine:** a layered-graph layout engine built for the character
  grid, with crossing minimization, orthogonal edge routing and label
  placement.
- **Wide diagrams:** compacts spacing or changes a diagram's direction to
  fit the screen, instead of breaking at the edge of the terminal.
- **Fallback:** unsupported diagram types and invalid input are shown as
  highlighted source with a clear label, not garbled output.
- **Speed:** renders a typical document, diagrams included, in milliseconds.
