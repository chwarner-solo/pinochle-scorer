import React from 'react';
import type { GameState, HandState } from '../types/Game';
import { HandEntryStartGame } from './HandEntryStartGame';
import type { HandEntryStartGameProps } from './HandEntryStartGame';
import { HandEntryStartHand } from './HandEntryStartHand';
import type { HandEntryStartHandProps } from './HandEntryStartHand';
import { HandEntryBid } from './HandEntryBid';
import { HandEntryTrump } from './HandEntryTrump';

interface HandEntryCardProps {
  state: GameState;
  handState: HandState | null;
  onGameSubmit: (data?: any) => void;
  onHandSubmit: (data?: any) => void;
  formData?: any;
  loading?: boolean;
  error?: string | null;
}

const WaitingForBid: React.FC<any> = (props) => (
  <HandEntryBid {...props} />
);
const WaitingForTrump: React.FC<any> = (props) => (
  <HandEntryTrump {...props} />
);
const WaitingForMeld: React.FC<any> = (props) => <>WaitingForMeld</>;
const WaitingForTricks: React.FC<any> = (props) => <>WaitingForTricks</>;
const Bidding: React.FC<any> = (props) => <>Bidding</>;
const Playing: React.FC<any> = (props) => <>Playing</>;
const Scoring: React.FC<any> = (props) => <>Scoring</>;
const NoMarriage: React.FC<any> = (props) => <>NoMarriage</>;

type GameStateComponents = {
  [K in Exclude<GameState, 'InProgress'>]: React.FC<any>;
} & {
  InProgress: HandStateComponents;
};

type HandStateComponents = {
  [K in HandState]: React.FC<{onHandSubmit: (data?: any) => void, onGameSubmit: (data?: any) => void}>;
};

const HAND_STATE_COMPONENTS : HandStateComponents = {
  NoHand: HandEntryStartHand,
  WaitingForBid,
  WaitingForTrump: WaitingForTrump,
  WaitingForMeld,
  WaitingForTricks,
  Bidding,
  Playing,
  Scoring,
  Completed: HandEntryStartHand,
  NoMarriage,
};

const GAME_STATE_COMPONENTS: Record<Exclude<GameState, 'InProgress'>, React.FC<any>> = {
  NoGame: (props) => <HandEntryStartGame {...props} />, // still uses onGameSubmit
  WaitingToStart: (props) => <HandEntryStartHand {...props} />, // still uses onGameSubmit
  Completed: (props) => <>Completed</>,
};

const HandEntryCard: React.FC<HandEntryCardProps> = (props) => {
  const { state, handState } = props;
  let Content: React.ReactNode = null;

  if (state === 'InProgress' && handState) {

    const HandStateComponent = HAND_STATE_COMPONENTS[handState];
    Content = HandStateComponent ? <HandStateComponent {...props} /> : <div>Unknown Hand State</div>;
  } else {

    const GameStateComponent = GAME_STATE_COMPONENTS[state as Exclude<GameState, 'InProgress'>];
    Content = GameStateComponent ? <GameStateComponent {...props} /> : <div>Unknown Game State</div>;
  }
  // Common layout wrapper for all hand entry views
  return (
    <div className="bg-white rounded shadow p-6 mb-4 text-center border border-gray-200 text-gray-900 flex flex-col items-center justify-center gap-4">
      {Content}
    </div>
  );
};

export default HandEntryCard;
