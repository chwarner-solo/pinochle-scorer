import React from "react";
import BidEntryBox from "../components/BidEntryBox";
import { MeldEntryBox } from "../components/MeldEntryBox";
import { TricksEntryBox } from "../components/TricksEntryBox";
import TrumpSelectionBox from "./TrumpSelectionBox";
import type { Game } from "../types/Game";

interface UserInteractionZoneProps {
  gameState: string;
  handState: string;
  loading: boolean;
  onStartGame: () => void;
  onStartHand: () => void;
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
  game: Game | null;
}

const UserInteractionZone: React.FC<UserInteractionZoneProps> = (props) => {
  const {
    gameState,
    handState,
    loading,
    onStartGame,
    onStartHand,
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
  } = props;

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
  if (gameState === 'InProgress') {
    const handStateMap: Record<string, React.JSX.Element> = {
      WaitingForBid: (
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
      ),
      WaitingForMeld: (
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
      ),
      NoMarriage: (
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
      ),
      WaitingForTricks: (
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
      ),
      WaitingForTrump: (
        <div className="flex flex-col items-center justify-center w-full px-6 py-3">
          <TrumpSelectionBox
            selectedTrump={selectedTrump}
            onTrumpClick={onTrumpClick}
            submitting={submittingTrump}
          />
        </div>
      ),
      Completed: (() => {
        const hands = game?.hands || [];
        const lastHand = hands[hands.length - 1];
        const prevUs = (game?.us_score ?? 0) - (lastHand?.us_score ?? 0);
        const prevThem = (game?.them_score ?? 0) - (lastHand?.them_score ?? 0);
        return (
          <div className="flex flex-col items-center justify-center py-12">
            <div className="text-3xl font-bold text-blue-700 mb-2">Hand Complete!</div>
            <div className="text-lg mb-6">Here's how the last hand went:</div>
            <div className="flex flex-row gap-12 mb-4">
              <div className="flex flex-col items-center">
                <div className="font-semibold text-gray-700 mb-1">North-South</div>
                <div className="mb-1">Previous: <span className="font-mono">{prevUs}</span></div>
                <div className="mb-1">Hand: <span className="font-mono">{lastHand?.us_score ?? 0}</span></div>
                <div className="font-bold text-green-700">Total: {game?.us_score}</div>
              </div>
              <div className="flex flex-col items-center">
                <div className="font-semibold text-gray-700 mb-1">East-West</div>
                <div className="mb-1">Previous: <span className="font-mono">{prevThem}</span></div>
                <div className="mb-1">Hand: <span className="font-mono">{lastHand?.them_score ?? 0}</span></div>
                <div className="font-bold text-green-700">Total: {game?.them_score}</div>
              </div>
            </div>
            <button
              className="bg-green-600 hover:bg-green-700 text-white font-bold py-2 px-8 rounded-lg shadow text-lg mt-4"
              onClick={onStartHand}
            >
              Start Hand
            </button>
            <div className="mt-6">
              <span className="text-gray-500">Start the next hand when ready.</span>
            </div>
          </div>
        );
      })(),
    };
    return handStateMap[handState] || <div className="h-12" />;
  }
  if (gameState === 'Completed') {
    const usScore = game?.us_score ?? 0;
    const themScore = game?.them_score ?? 0;
    let winner = '';
    if (usScore > themScore) {
      winner = 'North-South Team Wins!';
    } else if (themScore > usScore) {
      winner = 'East-West Team Wins!';
    } else {
      winner = "It's a tie!";
    }
    return (
      <div className="flex flex-col items-center justify-center py-16">
        <div className="text-4xl font-bold text-green-600 mb-4 animate-bounce">Game Complete!</div>
        <div className="text-2xl font-semibold text-gray-800 mb-8">{winner}</div>
        <button
          className="px-8 py-4 bg-blue-500 text-white rounded-xl shadow-lg text-xl hover:bg-blue-600 transition"
          onClick={onStartGame}
        >
          Start New Game
        </button>
        <div className="mt-8 text-6xl animate-pulse">🎉🏆</div>
      </div>
    );
  }
  // Placeholder for other states
  return <div className="h-12" />;
};

export default UserInteractionZone;
