import React from 'react';
import type { TrumpFormData } from '../types/form_types';
import type { Suit } from '../types/Game';
import { SuitIcons } from './IconMaps';

interface HandEntryTrumpProps {
  formData?: TrumpFormData;
  onHandSubmit: (data: TrumpFormData) => void;
  loading?: boolean;
  error?: string | null;
}

const suits: Suit[] = ['Spades', 'Hearts', 'Diamonds', 'Clubs'];

export const HandEntryTrump: React.FC<HandEntryTrumpProps> = ({ formData, onHandSubmit, loading, error }) => {
  const [selected, setSelected] = React.useState<Suit | undefined>(formData?.trump);

  const handleClick = (suit: Suit) => {
      console.log(`Selected suit: ${suit}`);
    setSelected(suit);
    onHandSubmit({ trump: suit });
  };

  return (
    <>
      <div className="text-xl font-semibold mb-2">Select Trump Suit</div>
      <div className="flex gap-4 justify-center mb-4">
        {suits.map((suit) => {
          const Icon = SuitIcons[suit];
          return (
            <button
              key={suit}
              type="button"
              onClick={() => handleClick(suit)}
              className={`w-16 h-16 flex items-center justify-center rounded-full border-2 text-3xl font-bold transition-colors shadow ${selected === suit ? 'bg-green-600 text-white border-green-700' : 'bg-white text-gray-700 border-gray-300 hover:bg-gray-100'}`}
              disabled={loading}
              aria-label={suit}
            >
              <Icon />
            </button>
          );
        })}
      </div>
      {error && <div className="text-red-600 mt-2">{error}</div>}
    </>
  );
};
