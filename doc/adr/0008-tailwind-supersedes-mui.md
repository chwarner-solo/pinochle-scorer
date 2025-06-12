# ADR 8: Switch from Material UI to Tailwind CSS (Supersedes ADR 5)

## Status
Accepted

## Context
The initial frontend stack used Material UI (MUI) for component styling and layout. As the project evolved, we determined that Tailwind CSS would provide more flexibility, faster prototyping, and a smaller bundle size due to its utility-first approach. Tailwind also aligns better with our team’s preferences for custom design and rapid iteration.

## Decision
- Replace Material UI (MUI) with Tailwind CSS for all new and existing frontend components.
- Update the build and configuration to support Tailwind CSS with Vite and React.
- All future UI work will use Tailwind utility classes and patterns.

## Consequences
- More control over design and faster UI development.
- Reduced dependency on third-party UI libraries.
- ADR 0005 (Vite + React + TypeScript + Material UI) is now superseded by this ADR.

## Supersedes
- [ADR 5: Front End Stack: Vite + React + TypeScript + Material UI](0005-frontend-vite-react.md)
