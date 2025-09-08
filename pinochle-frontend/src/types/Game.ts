export type Player = 'North' | 'South' | 'East' | 'West';
export type Suit = 'Spades' | 'Hearts' | 'Diamonds' | 'Clubs' | 'NoMarriage';

export const HandStateValues = ['NoHand', 'WaitingForBid', 'WaitingForTrump', 'WaitingForMeld', 'WaitingForTricks', 'Completed', 'NoMarriage'] as const;

export type HandState = typeof HandStateValues[number];

export type GameState = 'NoGame' | 'WaitingToStart' | 'InProgress' | 'Completed';

export interface Game {
    game_id?: string;
    game_state?: GameState;
    dealer?: Player;
    hand_state?: HandState;
    bidder?: Player | null;
    bid_amount?: number | null;
    trump?: Suit | null;
    us_meld?: number | null;
    them_meld?: number | null;
    us_tricks?: number | null;
    them_tricks?: number | null;
    us_score?: number | null;
    them_score?: number | null;
    // Add required_tricks to match backend
    required_tricks?: number;
    us_hand_score?: number;
    them_hand_score?: number;
    hands?: Hand[];
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
    us_score?: number;
    them_score?: number;
    dealer?: Player;
    required_tricks?: number;
}

export interface RunningTotal {
    us_total: number;
    them_total: number;
}