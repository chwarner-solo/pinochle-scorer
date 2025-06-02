# Pinochle Scorer Requirements

This document summarizes the current requirements for the Game and Hand entities, as inferred from the test cases.

## Game Entity
- A new game starts with a given dealer and is in the `WaitingToStart` state.
- Each game has a unique ID.
- A new game starts with no hands.
- Starting a new hand uses the current dealer and transitions the game to `InProgress`.
- The running total for both teams is zero at the beginning.
- Bids can only be placed on the current hand.
- Invalid bids (e.g., below minimum or invalid increments) are rejected and propagate errors.
- Trump can only be declared on the current hand and only after a valid bid.
- Melds and tricks are recorded on the current hand; errors are propagated as appropriate.
- Game state and dealer remain consistent across operations.

## Hand Entity
- A new hand starts in the `WaitingForBid` state with a given dealer and a unique ID.
- Only valid bids are accepted:
  - Minimum bid is 50.
  - Bids below minimum are rejected.
  - Valid bid increments:
    - Below 60: increments of 1.
    - 60 to 90: increments of 5.
    - 100 and above: increments of 10.
  - Invalid increments are rejected.
- Only the appropriate state allows bidding; bids in other states are rejected.
- Trump can be declared only after a valid bid and only by the bidder.
- If the bidder lacks a marriage, the hand transitions to `NoMarriage`.
- Melds are only accepted in the appropriate state and are validated:
  - Melds below 20 are forfeited.
  - Melds of 20 or more are recorded for the respective team.
- Tricks are recorded and validated; invalid trick totals result in errors.
- If the bidding team fails to make the contract, penalties are applied.
- Totals for both teams are only available in completed states; otherwise, they return zero.

---

These requirements are subject to change as the system evolves. See ADRs for architectural decisions.
