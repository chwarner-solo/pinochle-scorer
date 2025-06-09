export type Player = 'North' | 'South' | 'East' | 'West';
export type Suit = 'Spades' | 'Hearts' | 'Diamonds' | 'Clubs' | 'NoMarriage';

export const HandStateValues = ['NoHand', 'WaitingForBid', 'WaitingForTrump', 'WaitingForMeld', 'WaitingForTricks', 'Completed', 'NoMarriage'] as const;

export type HandState = typeof HandStateValues[number];

export type GameState = 'NoGame' | 'WaitingToStart' | 'InProgress' | 'Completed';

export interface Game {
    game_id: string;
    game_state: GameState;
    current_dealer: 'North' | 'South' | 'East' | 'West';
    completed_hands: Hand[];
    hand?: Hand
}

export interface Hand {
    hand_id: string;
    state: HandState;
    bidder?: Player;
    bid_amount?: number;
    trump?: Suit;
    us_meld?: number;
    them_meld?: number;
    us_tricks?: number;
    them_tricks?: number;
    us_total?: number;
    them_total?: number;
    dealer?: Player;
}

export interface RunningTotal {
    us_total: number;
    them_total: number;
}