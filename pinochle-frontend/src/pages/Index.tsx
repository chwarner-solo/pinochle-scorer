import React, { useEffect } from "react";
import { useGame } from "../hooks/useGame";
import { realGameApi } from "../services/api";
import GameHandAdminPanel from '../components/GameHandAdminPanel';
import HandsTableCard from '../components/HandsTableCard';
import { ErrorBoundary } from '../components/ErrorBoundary';
import { TableSeatButton } from '../components/TableSeatButton';
import type { Player, Suit } from '../types/Game';
import UserInteractionZone from "../components/user_interaction_zone";

// --- Types for Seat Mappings ---
type Seat = 'N' | 'E' | 'S' | 'W';
type PlayerToSeatMap = { [K in Player]: Seat };

// Helper to map full player name to seat code
const playerToSeatMap: PlayerToSeatMap = {
  North: 'N',
  East: 'E',
  South: 'S',
  West: 'W',
};
const playerToSeat = (player?: string) => player && playerToSeatMap[player as Player] ? playerToSeatMap[player as Player] : '';


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


// --- Main Page Refactor: manage bid/bidder/trump state at top level ---
const PinochleUXExperimentPage: React.FC = () => {
  const gameHook = useGame(realGameApi);
  const { game, state, handState, loading, onGameSubmit, onHandSubmit, onResetHand, trump, completedHands, selectedSeat, setSelectedSeat, bid, setBid, submittingBid, handleSubmitBid, selectedTrump, submittingTrump, onTrumpClick, usMeld, themMeld, setUsMeld, setThemMeld, submittingMeld, handleSubmitMeld, usTricks, setUsTricks, themTricks, setThemTricks, handleSubmitTricks, submittingTricks } = gameHook;
  // ...other state and handlers

  // Fetch running total and completed hands when Start Hand is clicked
  const handleStartHand = async () => {
    if (!game?.game_id) return;
    // Skipping running total and completed hands API calls for now
    await onHandSubmit({});
  };

  // --- Reset tricks values when entering WaitingForTricks ---
  useEffect(() => {
    if (handState === 'WaitingForTricks') {
      setUsTricks(0);
      setThemTricks(0);
    }
  }, [handState, setUsTricks, setThemTricks]);

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
            submitting={submittingBid}
            handleSubmitBid={handleSubmitBid}
            selectedTrump={selectedTrump}
            submittingTrump={submittingTrump}
            onTrumpClick={onTrumpClick}
            usMeld={usMeld}
            themMeld={themMeld}
            setUsMeld={setUsMeld}
            setThemMeld={setThemMeld}
            submittingMeld={submittingMeld}
            handleSubmitMeld={handleSubmitMeld}
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
