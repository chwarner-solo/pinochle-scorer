import React from 'react';
import type {
    HandEntryStartGameProps
} from "../types/component/user_interaction_zone.ts";

export const HandEntryStartGame: React.FC<HandEntryStartGameProps> = ({ onSubmit }) => (
  <>
    <div className="text-xl font-semibold mb-2">Ready to Play Pinochle?</div>
    <button
      className="bg-blue-600 hover:bg-blue-700 text-white font-bold py-2 px-6 rounded transition-colors text-lg shadow"
      onClick={onSubmit}
    >
      Start Game
    </button>
  </>
);
