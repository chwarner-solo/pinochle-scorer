import React, { useState } from 'react';
import type {
    HandEntryStartGameProps
} from "../types/component/user_interaction_zone.ts";

const DEALER_SEATS = ["North", "East", "South", "West"];

export const HandEntryStartGame: React.FC<HandEntryStartGameProps> = ({ onSubmit }) => {
  // Local state for selected dealer, default to 'South' if not provided
  const [selectedDealer, setSelectedDealer] = useState("South");

  const handleDealerSelect = (seat: string) => setSelectedDealer(seat as typeof selectedDealer);

  const handleStart = () => {
    // Optionally, pass selectedDealer to onSubmit if needed
    // Example: onSubmit(selectedDealer)
    onSubmit && onSubmit();
  };

  return (
    <>
      <div className="text-xl font-semibold mb-2">Ready to Play Pinochle?</div>
      <div className="flex flex-row gap-2 mb-4 justify-center items-center">
        {DEALER_SEATS.map(seat => (
          <button
            key={seat}
            type="button"
            className={`px-4 py-2 rounded-lg font-bold border-2 focus:outline-none transition-all duration-150 ${
              selectedDealer === seat
                ? "bg-blue-500 text-white border-blue-700 shadow"
                : "bg-white text-gray-700 border-gray-300 hover:bg-blue-100"
            }`}
            onClick={() => handleDealerSelect(seat)}
          >
            {seat}
          </button>
        ))}
        <button
          className="ml-4 bg-blue-600 hover:bg-blue-700 text-white font-bold py-2 px-6 rounded transition-colors text-lg shadow"
          onClick={handleStart}
        >
          Start Game
        </button>
      </div>
    </>
  );
};
