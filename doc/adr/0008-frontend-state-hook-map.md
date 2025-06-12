# ADR 8: Frontend State Management with Custom Hook and State-Driven MAP

## Status
Proposed

## Context
The Pinochle Scorer frontend requires dynamic UI rendering and logic that adapts to the current game or hand state. To manage this complexity, we adopted a custom React hook (`useGame`) and a MAP structure keyed by game/hand states. This approach centralizes state logic and UI/component mapping, making the codebase more maintainable and extensible.

## Decision
- Implement a custom React hook (`useGame`) to encapsulate game state logic, transitions, and side effects.
- Use a MAP (JavaScript object or ES6 Map) where each key is a specific game/hand state, and each value is a function, component, or handler relevant to that state (e.g., rendering, validation, or event handling).
- The hook exposes state, actions, and derived data to consuming components, enabling them to react to state changes declaratively.

## Consequences
- Centralizes all state-specific logic and UI mapping, reducing boilerplate and conditional rendering in components.
- Adding or modifying states is straightforward—update the MAP and the hook, with minimal changes elsewhere.
- Improves testability and separation of concerns.
- Developers can reason about UI flows and state transitions more easily.

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
- Inline conditional rendering in components (less maintainable, more error-prone).
- Global state management libraries (overkill for this use case).

## Related
- See `src/hooks/useGame.ts` and `src/components/` for implementation details.
