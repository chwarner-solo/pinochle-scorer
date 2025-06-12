# ADR 9: Frontend State Management with Custom Hook and State-Driven MAP

## Status
Accepted

## Context
The frontend requires dynamic UI rendering and logic that adapts to the current game or hand state. To manage this, we use a custom React hook (`useGame`) and a MAP structure keyed by game/hand states. This centralizes state logic and UI/component mapping, making the codebase more maintainable and extensible.

## Decision
- Implement a custom React hook (`useGame`) to encapsulate game state logic, transitions, and side effects.
- Use a MAP (object or ES6 Map) where each key is a game/hand state, and each value is a function, component, or handler for that state.
- The hook exposes state, actions, and derived data to components, enabling declarative UI updates.

## Consequences
- Centralizes state-specific logic and UI mapping.
- Adding/modifying states is straightforward—update the MAP and hook.
- Improves testability and separation of concerns.
- Easier to reason about UI flows and state transitions.

## Example
```typescript
const stateComponentMap = {
  WaitingForBid: HandEntryBid,
  WaitingForCards: HandEntryCard,
  // ...other states
};

const CurrentComponent = stateComponentMap[gameState];
return <CurrentComponent {...props} />;
```

## Alternatives Considered
- Inline conditional rendering (less maintainable).
- Global state management libraries (overkill for this use case).

## Related
- See `src/hooks/useGame.ts` and `src/components/` for details.
