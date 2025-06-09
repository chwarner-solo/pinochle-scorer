import React from 'react';
import { PlayerIcons, SuitIcons } from './IconMaps';
import type { Player, Suit, Hand } from '../types/Game';

interface GameStatusCardProps {
  usScore: number;
  themScore: number;
  trump: string;
  requiredTricks: number;
  hand?: Hand | null;
}

const GameStatusCard: React.FC<GameStatusCardProps> = ({ usScore, themScore, trump, requiredTricks, hand }) => {
  const TrumpIcon = SuitIcons[trump as Suit] || (() => <span>-</span>);
  const BidderIcon = hand?.bidder ? PlayerIcons[hand.bidder as Player] : undefined;
  const DealerIcon = hand?.dealer ? PlayerIcons[hand.dealer as Player] : undefined;

  return (
    <div className="bg-white rounded shadow p-6 mb-4 border border-gray-200 text-gray-900">
      <div className="grid grid-cols-3 grid-rows-6 border border-gray-400 rounded-lg overflow-hidden min-h-[200px] text-center">
        {/* US score, spans all rows left */}
        <div className="col-start-1 row-start-1 row-span-6 flex flex-col items-center justify-center border-r border-gray-400">
          <span className="text-4xl md:text-5xl text-blue-700 w-full block">{usScore}</span>
          <span className="text-xs text-gray-600 mt-1 uppercase tracking-wider w-full block">US</span>
        </div>
        {/* THEM score, spans all rows right */}
        <div className="col-start-3 row-start-1 row-span-6 flex flex-col items-center justify-center border-l border-gray-400">
          <span className="text-4xl md:text-5xl text-red-700 w-full block">{themScore}</span>
          <span className="text-xs text-gray-600 mt-1 uppercase tracking-wider w-full block">THEM</span>
        </div>
        {/* Trump, rows 1-3 center, label at top, trump centered vertically below */}
        <div className="col-start-2 row-start-1 row-span-3 flex flex-col items-center justify-between border-b border-gray-300 bg-gray-50 px-2 pt-2 pb-1 h-full">
          <span className="text-xs text-gray-500 uppercase tracking-wider mb-1">Trump</span>
          <div className="flex-1 flex items-center justify-center w-full">
            <TrumpIcon className="inline text-6xl md:text-7xl align-middle" />
          </div>
        </div>
        {/* Required Tricks, rows 4-5 center */}
        <div className="col-start-2 row-start-4 row-span-2 flex flex-col items-center justify-center border-b border-gray-300 bg-gray-50 px-1 py-0 min-h-0">
          <span className="text-[10px] text-gray-500 uppercase tracking-wider">Required Tricks</span>
          <span className="text-base font-bold text-blue-700">{requiredTricks ?? '-'}</span>
        </div>
        {/* Contract and Dealer, row 6 center */}
        <div className="col-start-2 row-start-6 flex flex-row items-center justify-center gap-4 bg-gray-50 px-1 py-0 min-h-0">
          <div className="flex flex-col items-center">
            <span className="text-[10px] text-gray-500 uppercase tracking-wider mb-0">Contract</span>
            <span className="flex flex-row items-center gap-1">
              {BidderIcon ? <BidderIcon className="inline text-sm align-middle" /> : <span>-</span>}
              <span className="text-sm font-bold">{hand?.bid_amount ?? '-'}</span>
            </span>
          </div>
          <div className="flex flex-col items-center">
            <span className="text-[10px] text-gray-500 uppercase tracking-wider mb-0">Dealer</span>
            {DealerIcon ? <DealerIcon className="inline text-sm align-middle" /> : <span>-</span>}
          </div>
        </div>
      </div>
    </div>
  );
};

export default GameStatusCard;
