import React from "react";

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
}> = (props) => {
  const { submitting, onTrumpClick, selectedTrump } = props;
  return (
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
};

export default TrumpSelectionBox;
