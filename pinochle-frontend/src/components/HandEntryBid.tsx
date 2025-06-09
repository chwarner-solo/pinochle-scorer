import React, { useState } from 'react';
import type { BidFormData } from '../types/form_types';
import type { Player } from '../types/Game';
import { PlayerIcons } from './IconMaps';

interface HandEntryBidProps {
  formData?: BidFormData;
  onHandSubmit: (data: BidFormData) => void;
  loading?: boolean;
  error?: string | null;
}

const players: Player[] = ['South', 'West', 'North', 'East'];

// Table positions for absolute placement (S at bottom, N at top, W at left, E at right)
const playerCirclePositions: Record<Player, { top: string; left: string }> = {
  North: { top: '10%', left: '50%' }, // top center
  South: { top: '90%', left: '50%' }, // bottom center
  East:  { top: '50%', left: '90%' }, // right center
  West:  { top: '50%', left: '10%' }, // left center
};

export const HandEntryBid: React.FC<HandEntryBidProps> = ({ formData, onHandSubmit: onSubmit, loading, error }) => {
  const [bid, setBid] = useState(formData?.bid ?? 50);
  const [player, setPlayer] = useState<Player>(formData?.player ?? 'South');

  const handlePlayerClick = (p: Player) => setPlayer(p);
  const handleBidChange = (e: React.ChangeEvent<HTMLInputElement>) => setBid(Number(e.target.value));

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();

    console.log("Submitting bid:", { bid, player });
    onSubmit({ bid, player });
  };

  return (
    <>
      <div className="text-xl font-semibold mb-2">Declare Bid</div>
      <form onSubmit={handleSubmit} className="flex flex-col items-center gap-4 bg-white p-6 rounded shadow">
        <div className="relative w-40 h-40 flex items-center justify-center mb-2">
          {/* Table reference (large circle) */}
          <div className="absolute rounded-full border-2 border-gray-300 w-32 h-32 left-1/2 top-1/2" style={{ transform: 'translate(-50%, -50%)' }} />
          {/* Player buttons positioned around the table */}
          {players.map((p) => {
            const Icon = PlayerIcons[p];
            const pos = playerCirclePositions[p];
            return (
              <button
                type="button"
                key={p}
                className={`absolute w-12 h-12 flex items-center justify-center rounded-full border-2 text-xl font-bold transition-colors shadow ${player === p ? 'bg-blue-600 text-white border-blue-700 z-10' : 'bg-white text-blue-700 border-blue-300 hover:bg-blue-100 z-0'}`}
                style={{
                  top: pos.top,
                  left: pos.left,
                  transform: 'translate(-50%, -50%)',
                }}
                onClick={() => handlePlayerClick(p)}
                aria-label={`Select ${p}`}
              >
                <Icon />
              </button>
            );
          })}
        </div>
        <input
          type="number"
          min={50}
          value={bid}
          onChange={handleBidChange}
          className="border rounded px-2 py-1 w-32 text-center text-lg"
          placeholder="Bid Amount"
          required
        />
        <button
          type="submit"
          className="bg-blue-600 hover:bg-blue-700 text-white font-bold py-2 px-6 rounded transition-colors text-lg shadow"
          disabled={loading}
        >
          Declare Bid
        </button>
        {error && <div className="text-red-600 mt-2">{error}</div>}
      </form>
    </>
  );
};
