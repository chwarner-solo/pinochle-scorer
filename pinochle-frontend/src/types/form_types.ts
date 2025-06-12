import type {Player, Suit, HandState} from "./Game.ts";

export type BidFormData = { bid: number, player: Player };
export type TrumpFormData = { trump: Suit };
export type MeldFormData = { us_meld: number, them_meld: number};
export type TricksFormData = { us_tricks: number, them_tricks: number};
export type CompletedFormData = {};

export type FormSchema = {
    "NoHand": {},
    "WaitingForBid": BidFormData,
    "WaitingForTrump": TrumpFormData,
    "WaitingForMeld": MeldFormData,
    "WaitingForTricks": TricksFormData,
    "Completed": CompletedFormData,
    "NoMarriage": MeldFormData
}

export type FormData = {
    [K in HandState]: FormSchema[K];
}

export type AnyFormData = (FormSchema[keyof FormSchema]) & { resetForm?: () => void };
