import type {Player, Suit, HandState} from "./Game.ts";

export type BidFormData = { bid: number, player: Player };
export type TrumpFormData = { trump: Suit };
export type MeldFormData = { us_meld: number, them_meld: number};
export type TricksFormData = { us_tricks: number, them_tricks: number};
export type CompletedFormData = {};

export type HandFormData = {
    [K in keyof HandState]: { state: K; data: FormData[K] }
};

export type FormData = {
    WaitingForBid: BidFormData,
    WaitingForTrump: TrumpFormData,
    WaitingForMeld: MeldFormData,
    WaitingForTricks: TricksFormData
    Completed: CompletedFormData,
    NoMarriage: MeldFormData
}
