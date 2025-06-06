export type Player = 'North' | 'South' | 'East' | 'West';
export type Suit = 'Spades' | 'Hearts' | 'Diamonds' | 'Clubs' | 'NoMarriage';

export const HandStateValues = ['WaitingForBid', 'WaitingForTrump', 'WaitingForMeld', 'WaitingForTricks', 'Completed', 'NoMarriage'] as const;

export type HandState = typeof HandStateValues[number];

export type GameState = 'WaitingForPlayers' | 'InProgress' | 'Completed';

export interface Game {
    game_id: string;
    state: GameState;
    current_dealer: 'North' | 'South' | 'East' | 'West';
    completed_hands: Hand[];
    current_hand?: Hand
}

export interface Hand {
    hand_id: string;
    state: HandState;
    bidder?: Player;
    bind_amount?: number;
    trump?: Suit;
}

export interface RunningTotal {
    us_total: number;
    them_total: number;
}