export type BidFormData = { bid: number, player: Player | '' };
export type TrumpFormData = { trump: Suit };
export type MeldFormData = { us: u32, them: u32};
export type TricksFormData = { us: u32, them: u32};

export type HandFormData =
    | { state: "WaitingForBid", data: BidFormData }
    | { state: "WaitingForTrump", data:TrumpFormData }
    | { state: "WaitingForMeld", data: MeldFormData }
    | { state: "WaitingForTricks", data:TricksFormData};