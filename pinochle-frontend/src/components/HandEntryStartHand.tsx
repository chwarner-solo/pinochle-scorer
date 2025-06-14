import React from 'react';
import type {HandEntryStartHandProps} from '../types/component/user_interaction_zone';

export const HandEntryStartHand: React.FC<HandEntryStartHandProps> = ({ onSubmit }) => (
  <>
    <div className="text-xl font-semibold mb-2">Ready to Start a New Hand?</div>
    <button
      className="bg-green-600 hover:bg-green-700 text-white font-bold py-2 px-6 rounded transition-colors text-lg shadow"
      onClick={onSubmit}
    >
      Start New Hand
    </button>
  </>
);
