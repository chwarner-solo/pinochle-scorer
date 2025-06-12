import React, { useState, useEffect } from "react";
import { useGame } from "../hooks/useGame";
import { realGameApi } from "../services/api";
import GameHandAdminPanel from '../components/GameHandAdminPanel';
import HandsTableCard from '../components/HandsTableCard';
import { ErrorBoundary } from '../components/ErrorBoundary';
import { TableSeatButton } from '../components/TableSeatButton';
import BidEntry from '../components/BidEntryBox';
import { MeldEntryBox } from '../components/MeldEntryBox';
import { TricksEntryBox } from '../components/TricksEntryBox';
import type { Player, Suit } from '../types/Game';
import BidEntryBox from "../components/BidEntryBox";

// --- Types for Seat Mappings ---
type Seat = 'N' | 'E' | 'S' | 'W';
type PlayerToSeatMap = { [K in Player]: Seat };
type SeatToPlayerMap = { [K in Seat]: Player };

// Helper to map full player name to seat code
const playerToSeatMap: PlayerToSeatMap = {
  North: 'N',
  East: 'E',
  South: 'S',
  West: 'W',
};
const playerToSeat = (player?: string) => player && playerToSeatMap[player as Player] ? playerToSeatMap[player as Player] : '';

// Helper to map seat code to full player name
const seatToPlayerMap: SeatToPlayerMap = {
  N: 'North',
  E: 'East',
  S: 'South',
  W: 'West',
};
const seatToPlayer = (seat: string): Player => {
  if (seatToPlayerMap[seat as Seat]) return seatToPlayerMap[seat as Seat];
  throw new Error('Invalid seat');
};

// --- Helper to get seat code for a player, fallback to '' ---
const getBidderSeat = (bidder: string | undefined): Seat | '' => {
  if (!bidder) return '';
  // Defensive: Accept both seat code ('S') or player name ('South')
  if (['N', 'E', 'S', 'W'].includes(bidder)) return bidder as Seat;
  return playerToSeatMap[bidder as Player] || '';
};

// --- Suit Icon Map ---
const SuitIconMap: { [K in Suit]: React.ReactNode } = {
  Spades: <span className="text-black">♠️</span>,
  Hearts: <span className="text-red-500">♥️</span>,
  Diamonds: <span className="text-pink-400">♦️</span>,
  Clubs: <span className="text-green-700">♣️</span>,
  NoMarriage: (
    <svg width="20" height="20" viewBox="0 0 28 28">
      <circle cx="14" cy="14" r="11" stroke="gray" strokeWidth="2.5" fill="none" />
      <line x1="7" y1="21" x2="21" y2="7" stroke="gray" strokeWidth="2.5" />
    </svg>
  ),
};

// --- Trump Suit Icon Component ---
const TrumpSuitIcon: React.FC<{ suit?: string }> = ({ suit }) => {
  if (!suit || !(suit in SuitIconMap)) return <span>-</span>;
  return <span className="inline-block align-middle text-2xl">{SuitIconMap[suit as keyof typeof SuitIconMap]}</span>;
};

// --- Trump Box Component ---
interface TrumpBoxProps {
  trump: string;
}

const TrumpBox = ({ trump }: TrumpBoxProps) => (
  <div className="flex flex-col items-center justify-start w-36 h-36 bg-white rounded-2xl shadow-lg border border-gray-300 mr-6 mt-2 relative">
    <div className="absolute -top-4 left-1/2 -translate-x-1/2 bg-white px-3 py-0.5 rounded-t-xl border-b border-gray-300 text-center text-base font-semibold text-gray-700 shadow-sm">
      Trump
    </div>
    <div className="flex items-center justify-center h-full w-full">
      <TrumpSuitIcon suit={trump} />
    </div>
  </div>
);

// --- Bidder Box Component ---
interface BidderBoxProps {
  bidder: string;
  bid: number;
  requiredTricks: number;
}

const BidderBox = ({ bidder, bid, requiredTricks }: BidderBoxProps) => {
  return (
    <div className="flex flex-col items-center justify-start w-36 h-36 bg-white rounded-2xl shadow-lg border border-gray-300 ml-6 mt-2 relative">
      <div className="absolute -top-4 left-1/2 -translate-x-1/2 bg-white px-3 py-0.5 rounded-t-xl border-b border-gray-300 text-center text-base font-semibold text-gray-700 shadow-sm">
        Bidder
      </div>
      <div className="flex flex-col items-center justify-center h-full w-full gap-2">
        <div className="flex items-center gap-2">
          <span className="block w-8 h-8 rounded-full border-2 border-blue-500 flex items-center justify-center text-blue-600 font-bold text-lg shadow">{bidder}</span>
          <span className="text-xs text-gray-500">(Contract)</span>
        </div>
        <div className="text-xl font-semibold text-gray-700">Bid: <span className="text-blue-700">{bid}</span></div>
        <div className="text-md text-gray-600">Req. Tricks: <span className="font-bold text-gray-900">{requiredTricks}</span></div>
      </div>
    </div>
  );
};

// --- Score Box Component ---
const ScoreBox = ({ label, value, color }: { label: string; value: number; color: string }) => (
  <div className="flex flex-col items-center justify-start w-36 h-24 bg-white rounded-2xl shadow-lg border border-gray-300 mt-2 relative">
    <div className="absolute -top-4 left-1/2 -translate-x-1/2 bg-white px-3 py-0.5 rounded-t-xl border-b border-gray-300 text-center text-base font-semibold text-gray-700 shadow-sm" style={{ color }}>
      {label}
    </div>
    <div className="flex-1 flex items-center justify-center w-full">
      <span className="text-3xl font-bold" style={{ color }}>{value}</span>
    </div>
  </div>
);

// --- ScoreBlocks using ScoreBox ---
const ScoreBlocks = ({ usScore, themScore }: { usScore: number; themScore: number }) => (
  <>
    <ScoreBox label="US" value={usScore} color="#60a5fa" />
    <ScoreBox label="THEM" value={themScore} color="#f87171" />
  </>
);

// --- TableSeats ---
interface TableSeatsProps {
  dealerSeat?: string;
  bidderSeat?: string;
  onSeatClick?: (seat: string) => void;
  selectedSeat?: string;
}

const seats = ["N", "E", "S", "W"];
const seatPositions = [
  { top: 0, left: "50%", translate: "-50%, -80%" },   // N
  { top: "50%", right: 0, translate: "80%, -50%" },   // E
  { bottom: 0, left: "50%", translate: "-50%, 80%" }, // S
  { top: "50%", left: 0, translate: "-80%, -50%" },   // W
];

const TableSeats: React.FC<TableSeatsProps> = ({ dealerSeat, bidderSeat, onSeatClick, selectedSeat }) => {
  return (
    <div className="relative" style={{ height: 300, width: 300 }}>
      <div className="absolute inset-0 flex items-center justify-center">
        <div className="w-56 h-56 bg-green-200 rounded-full flex items-center justify-center shadow-inner">
          <span className="text-gray-500">Table</span>
        </div>
      </div>
      {seats.map((seat, idx) => (
        <TableSeatButton
          key={seat}
          seat={seat}
          pos={seatPositions[idx]}
          isDealer={seat === dealerSeat}
          isBidder={seat === bidderSeat}
          isSelected={seat === selectedSeat}
          onClick={onSeatClick}
        />
      ))}
    </div>
  );
};

// --- Table Visualization Layout ---
interface PinochleTableVisualizationProps {
  trump: string;
  bidder: string;
  bid: number;
  requiredTricks: number;
  dealerSeat?: string;
  bidderSeat?: string;
  onSeatClick?: (seat: string) => void;
  selectedSeat?: string;
}

const PinochleTableVisualization: React.FC<PinochleTableVisualizationProps> = ({ trump, bidder, bid, requiredTricks, dealerSeat, bidderSeat, onSeatClick, selectedSeat }) => (
  <div className="flex flex-row items-stretch justify-center w-full min-h-[380px]">
    {/* Trump box on the far left */}
    <TrumpBox trump={trump} />
    {/* Table and seats centered vertically and horizontally in available space */}
    <div className="flex flex-1 items-center justify-center min-h-[380px]">
      <TableSeats dealerSeat={dealerSeat} bidderSeat={bidderSeat} onSeatClick={onSeatClick} selectedSeat={selectedSeat} />
    </div>
    {/* Bidder box on the far right */}
    <BidderBox bidder={bidder} bid={bid} requiredTricks={requiredTricks} />
  </div>
);

// --- Trump Selection UI ---
const SUITS = [
  { suit: 'Spades', icon: '♠️', color: 'text-black' },
  { suit: 'Hearts', icon: '♥️', color: 'text-red-500' },
  { suit: 'Diamonds', icon: '♦️', color: 'text-pink-400' },
  { suit: 'Clubs', icon: '♣️', color: 'text-green-700' },
  { suit: 'NoMarriage', icon: null, color: 'text-gray-500' },
];

const TrumpSelectionBox: React.FC<{
  submitting: boolean;
  onTrumpClick: (suit: string) => void;
  selectedTrump: string;
}> = ({ submitting, onTrumpClick, selectedTrump }) => (
  <div className="flex flex-col items-center gap-2">
    <span className="text-lg font-semibold text-gray-700 mb-2">Select Trump Suit:</span>
    <div className="flex flex-row gap-4 mb-2">
      {SUITS.map(({ suit, icon, color }) => (
        <button
          key={suit}
          className={`rounded-full border-2 px-4 py-2 text-2xl font-bold focus:outline-none focus:ring-2 focus:ring-blue-400 transition-all duration-150 ${selectedTrump === suit ? 'bg-blue-200 border-blue-600' : 'bg-white border-gray-300'} ${color}`}
          aria-pressed={selectedTrump === suit}
          onClick={() => !submitting && onTrumpClick(suit)}
          disabled={submitting}
        >
          {icon || (
            // NoMarriage: circle with line through it
            <svg width="28" height="28" viewBox="0 0 28 28">
              <circle cx="14" cy="14" r="11" stroke="gray" strokeWidth="2.5" fill="none" />
              <line x1="7" y1="21" x2="21" y2="7" stroke="gray" strokeWidth="2.5" />
            </svg>
          )}
        </button>
      ))}
    </div>
  </div>
);

// --- Bid Data Form Component ---
interface BidDataFormProps {
  bid: number;
  setBid: (amt: number) => void;
  submitting: boolean;
  onSubmit: () => void;
}

const BidDataForm: React.FC<BidDataFormProps> = ({ bid, setBid, submitting, onSubmit }) => (
  <form
    className="flex flex-col items-center gap-2"
    onSubmit={e => {
      e.preventDefault();
      onSubmit();
    }}
  >
    <label className="text-lg font-semibold text-gray-700">
      Bid Amount:
      <input
        type="number"
        min={50}
        max={250}
        step={1}
        value={bid}
        onChange={e => setBid(Number(e.target.value))}
        className="ml-2 border px-2 py-1 rounded w-24 text-center"
        disabled={submitting}
      />
    </label>
    <button
      type="submit"
      className="bg-blue-600 hover:bg-blue-700 text-white font-bold py-1 px-6 rounded shadow text-base mt-2"
      disabled={submitting}
    >
      Submit Bid
    </button>
  </form>
);

// --- User Interaction Zone ---
interface UserInteractionZoneProps {
  gameState: string;
  handState: string;
  loading: boolean;
  onStartGame: () => void;
  onStartHand: () => void;
  onResetHand: () => void;
  selectedSeat: string;
  setSelectedSeat: (seat: string) => void;
  bid: number;
  setBid: (amt: number) => void;
  submitting: boolean;
  handleSubmitBid: () => void;
  selectedTrump: string;
  submittingTrump: boolean;
  onTrumpClick: (suit: string) => void;
  usMeld: number;
  themMeld: number;
  setUsMeld: (val: number) => void;
  setThemMeld: (val: number) => void;
  handleSubmitMeld: () => void;
  submittingMeld: boolean;
  usTricks: number;
  themTricks: number;
  setUsTricks: (val: number) => void;
  setThemTricks: (val: number) => void;
  handleSubmitTricks: () => void;
  submittingTricks: boolean;
  game: Game;
}

const UserInteractionZone: React.FC<UserInteractionZoneProps> = ({
  gameState,
  handState,
  loading,
  onStartGame,
  onStartHand,
  onResetHand,
  selectedSeat,
  setSelectedSeat,
  bid,
  setBid,
  submitting,
  handleSubmitBid,
  selectedTrump,
  submittingTrump,
  onTrumpClick,
  usMeld,
  themMeld,
  setUsMeld,
  setThemMeld,
  handleSubmitMeld,
  submittingMeld,
  usTricks,
  themTricks,
  setUsTricks,
  setThemTricks,
  handleSubmitTricks,
  submittingTricks,
  game,
}) => {
  useEffect(() => {
    if (handState === 'WaitingForBid') setBid(50);
  }, [handState]);

  if (gameState === 'NoGame') {
    return (
      <div className="flex items-center justify-center w-full px-6 py-3">
        <button
          className="bg-blue-600 hover:bg-blue-700 text-white font-bold py-2 px-6 rounded-lg shadow disabled:opacity-60 disabled:cursor-not-allowed text-lg"
          onClick={onStartGame}
          disabled={loading}
        >
          {loading ? 'Starting...' : 'Start Game'}
        </button>
      </div>
    );
  }
  if (gameState === 'WaitingToStart') {
    return (
      <div className="flex items-center justify-center w-full px-6 py-3">
        <button
          className="bg-green-600 hover:bg-green-700 text-white font-bold py-2 px-6 rounded-lg shadow disabled:opacity-60 disabled:cursor-not-allowed text-lg"
          onClick={onStartHand}
          disabled={loading}
        >
          {loading ? 'Starting...' : 'Start Hand'}
        </button>
      </div>
    );
  }
  if (gameState === 'InProgress' && handState === 'WaitingForBid') {
    return (
      <div className="flex flex-col items-center justify-center w-full px-6 py-3">
        <span className="text-lg font-semibold text-gray-700 mb-2">Enter Bidder Seat & Bid:</span>
        <BidEntryBox
          selected={selectedSeat}
          onSelect={setSelectedSeat}
          bid={bid}
          setBid={setBid}
          onSubmit={handleSubmitBid}
          submitting={submitting}
        />
      </div>
    );
  }
  if (gameState === 'InProgress' && handState === 'WaitingForMeld') {
    return (
      <div className="flex flex-col items-center justify-center w-full px-6 py-3">
        <span className="text-lg font-semibold text-gray-700 mb-2">Enter Melds:</span>
        <MeldEntryBox
          usMeld={usMeld}
          themMeld={themMeld}
          setUsMeld={setUsMeld}
          setThemMeld={setThemMeld}
          onSubmit={handleSubmitMeld}
          submitting={submittingMeld}
        />
      </div>
    );
  }
  if (gameState === 'InProgress' && handState === 'WaitingForTricks') {
    return (
      <div className="flex flex-col items-center justify-center w-full px-6 py-3">
        <span className="text-lg font-semibold text-gray-700 mb-2">Enter Tricks:</span>
        <TricksEntryBox
          usTricks={usTricks}
          themTricks={themTricks}
          setUsTricks={setUsTricks}
          setThemTricks={setThemTricks}
          onSubmit={handleSubmitTricks}
          submitting={submittingTricks}
        />
      </div>
    );
  }
  if (gameState === 'InProgress' && handState === 'WaitingForTrump') {
    return (
      <div className="flex flex-col items-center justify-center w-full px-6 py-3">
        <TrumpSelectionBox
          selectedTrump={selectedTrump}
          onTrumpClick={onTrumpClick}
          submitting={submittingTrump}
        />
      </div>
    );
  }
  if (gameState === 'InProgress' && handState === 'Completed') {
    const showHandScore = handState === 'Completed';
    return (
      <div className="flex flex-col items-center justify-center w-full px-6 py-3">
        <button
          className="bg-green-600 hover:bg-green-700 text-white font-bold py-2 px-8 rounded-lg shadow text-lg mt-4"
          onClick={onStartHand}
        >
          Start Hand
        </button>
        {showHandScore && (
          <div className="my-4 p-4 rounded-xl bg-blue-50 border border-blue-200 text-center">
            <div className="text-lg font-bold mb-2">Hand Complete!</div>
            <div>
              US Hand Score: <span className="font-mono">{game.us_hand_score ?? 0}</span>
            </div>
            <div>
              THEM Hand Score: <span className="font-mono">{game.them_hand_score ?? 0}</span>
            </div>
            <div className="mt-2">
              Running Total: US <span className="font-mono">{game.us_score ?? 0}</span> - THEM <span className="font-mono">{game.them_score ?? 0}</span>
            </div>
          </div>
        )}
      </div>
    );
  }
  // Placeholder for other states
  return <div className="h-12" />;
};

// --- Main Page Refactor: manage bid/bidder/trump state at top level ---
const PinochleUXExperimentPage: React.FC = () => {
  const gameHook = useGame(realGameApi);
  const { game, state, handState, loading, onGameSubmit, onHandSubmit, onResetHand, trump, completedHands } = gameHook;
  // ...other state and handlers

  // Fetch running total and completed hands when Start Hand is clicked
  const handleStartHand = async () => {
    if (!game?.game_id) return;
    // Skipping running total and completed hands API calls for now
    await onHandSubmit({});
  };

  // --- UI State for Bidder, Bid, Trump, Meld, Tricks ---
  const [selectedSeat, setSelectedSeat] = useState('');
  useEffect(() => {
    if (!game?.bidder) setSelectedSeat('');
    else setSelectedSeat(playerToSeat(game.bidder));
  }, [game?.bidder]);
  const [bid, setBid] = useState(50);
  const [submitting, setSubmitting] = useState(false);
  const [selectedTrump, setSelectedTrump] = useState('');
  const [submittingTrump, setSubmittingTrump] = useState(false);
  const [usMeld, setUsMeld] = useState(0);
  const [themMeld, setThemMeld] = useState(0);
  const [submittingMeld, setSubmittingMeld] = useState(false);
  const [usTricks, setUsTricks] = useState(0);
  const [themTricks, setThemTricks] = useState(0);
  const [submittingTricks, setSubmittingTricks] = useState(false);

  // --- Reset trump selection when entering WaitingForTrump ---
  useEffect(() => {
    if (handState === 'WaitingForTrump') {
      setSelectedTrump('');
    }
  }, [handState]);

  // --- Reset trump selection when hand or game is complete ---
  useEffect(() => {
    if (handState === 'Complete' || state === 'Complete') {
      setSelectedTrump('');
    }
  }, [handState, state]);

  // --- Reset meld values when entering WaitingForMeld ---
  useEffect(() => {
    if (handState === 'WaitingForMeld') {
      setUsMeld(0);
      setThemMeld(0);
    }
  }, [handState]);

  // --- Reset tricks values when entering WaitingForTricks ---
  useEffect(() => {
    if (handState === 'WaitingForTricks') {
      setUsTricks(0);
      setThemTricks(0);
    }
  }, [handState]);

  // --- Handlers for Bid, Trump, Meld, Tricks ---
  const handleSubmitBid = async () => {
    setSubmitting(true);
    try {
      const bidForm = { bid, player: seatToPlayer(selectedSeat) };
      await onHandSubmit(bidForm);
    } finally {
      setSubmitting(false);
    }
  };
  const handleTrumpClick = async (suit: string) => {
    setSelectedTrump(suit);
    setSubmittingTrump(true);
    try {
      await onHandSubmit({ trump: suit });
    } finally {
      setSubmittingTrump(false);
    }
  };
  const handleSubmitMeld = async () => {
    setSubmittingMeld(true);
    try {
      await onHandSubmit({ us_meld: usMeld, them_meld: themMeld });
    } finally {
      setSubmittingMeld(false);
    }
  };
  const handleSubmitTricks = async () => {
    setSubmittingTricks(true);
    try {
      await onHandSubmit({ us_tricks: usTricks, them_tricks: themTricks });
    } finally {
      setSubmittingTricks(false);
    }
  };

  // Only show tags if we are not in 'NoGame' state
  const showTags = state !== 'NoGame';
  const bidValue = showTags && game?.bid_amount ? game.bid_amount : 0;
  // Pass bidder and bid from game state for contract view and table
  const tableBidderSeat = showTags && game?.bidder ? getBidderSeat(game.bidder) : '';

  // Use selectedSeat during bidding, otherwise use backend bidder
  const visualBidderSeat =
    state === 'InProgress' && handState === 'WaitingForBid'
      ? selectedSeat
      : tableBidderSeat;

  console.log('tableBidderSeat:', tableBidderSeat, 'game.bidder:', game?.bidder, 'visualBidderSeat:', visualBidderSeat);

  return (
    <div className="min-h-screen w-full flex flex-col items-center bg-gray-100 py-6">
      <div className="w-full max-w-3xl bg-white rounded-3xl shadow-xl border border-gray-200 flex flex-col divide-y divide-gray-200">
        {/* User Interaction Zone with Error Boundary */}
        <ErrorBoundary>
          <UserInteractionZone
            gameState={state}
            handState={handState as string}
            loading={loading}
            onStartGame={onGameSubmit}
            onStartHand={handleStartHand}
            onResetHand={onResetHand}
            selectedSeat={selectedSeat}
            setSelectedSeat={setSelectedSeat}
            bid={bid}
            setBid={setBid}
            submitting={submitting}
            handleSubmitBid={handleSubmitBid}
            selectedTrump={selectedTrump}
            submittingTrump={submittingTrump}
            onTrumpClick={handleTrumpClick}
            usMeld={usMeld}
            themMeld={themMeld}
            setUsMeld={setUsMeld}
            setThemMeld={setThemMeld}
            handleSubmitMeld={handleSubmitMeld}
            submittingMeld={submittingMeld}
            usTricks={usTricks}
            themTricks={themTricks}
            setUsTricks={setUsTricks}
            setThemTricks={setThemTricks}
            handleSubmitTricks={handleSubmitTricks}
            submittingTricks={submittingTricks}
            game={game}
          />
        </ErrorBoundary>
        {/* Table Visualization Zone */}
        <div className="py-6 px-2">
          <PinochleTableVisualization
            trump={trump}
            bidder={visualBidderSeat}
            bid={bidValue}
            requiredTricks={game?.required_tricks ?? 0}
            dealerSeat={playerToSeat(game?.dealer)}
            bidderSeat={visualBidderSeat}
            onSeatClick={setSelectedSeat}
            selectedSeat={state === 'InProgress' && handState === 'WaitingForBid' ? selectedSeat : undefined}
          />
        </div>
        {/* Scoring Zone */}
        <div className="flex flex-row items-start justify-between px-8 py-4">
          <ScoreBlocks usScore={game?.us_score ?? 0} themScore={game?.them_score ?? 0} />
        </div>
        {/* Admin/Stats Panel (bottom) */}
        <div className="px-4 pt-2 pb-6">
          <HandsTableCard completedHands={completedHands} />
          <GameHandAdminPanel game={game} completedHands={completedHands} />
        </div>
      </div>
    </div>
  );
};

export default PinochleUXExperimentPage;
