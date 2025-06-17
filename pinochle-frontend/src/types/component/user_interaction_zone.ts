import React from "react";
import type {GameState, HandState, Player} from "../Game.ts";
import type { HandCompleteProps } from '../../components/handcomplete/HandComplete';

// --- Per-state prop bundles ---
export type NoGameProps = { 
  onSubmit: () => void;
  dealer?: Player;
};

export type WaitingToStartProps = { 
  onSubmit: () => void 
};

export type CompletedProps = { 
  onSubmit: () => void 
};

export type HandEntryBidProps = { 
  bidEntryBoxProps: BidEntryBoxProps; 
  onSubmit: () => void 
};

export type HandEntryTrumpProps = { 
  onSubmit: () => void 
};

export type HandEntryMeldProps = { 
  onSubmit: () => void 
};

export type HandEntryTricksProps = { 
  onSubmit: () => void 
};

export type BidEntryBoxProps = {
  selected: string;
  onSelect: (seat: string) => void;
  bid: number;
  setBid: (amt: number) => void;
  onSubmit: () => void;
  submitting: boolean;
};

export type StartHandProps = {
  onSubmit: () => void;
};

export type GameStateProps = {
  NoGame: NoGameProps;
  WaitingToStart: WaitingToStartProps;
  Completed: CompletedProps;
};
// 'NoHand', 'WaitingForBid', 'WaitingForTrump', 'WaitingForMeld', 'WaitingForTricks', 'Completed', 'NoMarriage'
export type HandStateProps = {
  NoHand: StartHandProps;
  WaitingForBid: HandEntryBidProps;
  WaitingForTrump: HandEntryTrumpProps;
  WaitingForMeld: HandEntryMeldProps;
  WaitingForTricks: HandEntryTricksProps;
  Completed: HandCompleteProps;
  NoMarriage: HandEntryMeldProps;
};

export interface UserInteractionZoneProps {
  gameState: GameState;
  handState: HandState;
  views: {
    [K in Exclude<GameState, "InProgress">]: GameStateProps[K];
  } & {
    InProgress: {
      [H in HandState]: HandStateProps[H];
    };
  };
}