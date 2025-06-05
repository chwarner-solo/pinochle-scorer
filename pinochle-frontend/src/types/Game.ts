export type Player = 'North' | 'South' | 'East' | 'West';
export type Suit = 'Spades' | 'Hearts' | 'Diamonds' | 'Clubs' | 'NoMarriage';

export interface Game {
    id: string;
    state: 'WaitingForPlayers' | 'InProgress' | 'Completed';
    current_dealer: 'North' | 'South' | 'East' | 'West';
    completed_hands: Hand[];
    current_hand?: Hand
}

export interface Hand {
    id: string;
    state: 'WaitingForBid' | 'WaitingForTrump' | 'WaitingForMeld' | 'WaitingForTricks' | 'Completed' | 'NoMarriage';
    bidder?: Player;
    bind_amount?: number;
    trump?: Suit;
}

export interface RunningTotal {
    us_total: number;
    them_total: number;
}