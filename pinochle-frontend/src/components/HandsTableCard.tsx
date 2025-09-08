import React from 'react';
import {PlayerIcons, SuitIcons} from "./IconMaps.tsx";
import type {Player, Suit, Hand} from "../types/Game.ts";

interface HandsTableCardProps {
  completedHands: Hand[];
}

const HandsTableCard: React.FC<HandsTableCardProps> = ({ completedHands }) => (
  <div className="bg-white rounded shadow p-6 mb-4 border border-gray-200 text-gray-900 overflow-x-auto">
    <table className="min-w-full text-sm text-center border border-gray-400 border-collapse">
      <thead>
        <tr className="bg-gray-100">
          <th className="px-2 py-2 border border-gray-400 align-middle">#</th>
          <th className="px-2 py-2 border border-gray-400 align-middle">US</th>
          <th className="px-2 py-2 border border-gray-400 align-middle">THEM</th>
          <th className="px-2 py-2 border border-gray-400 align-middle">Who</th>
          <th className="px-2 py-2 border border-gray-400 align-middle">Dealer</th>
        </tr>
      </thead>
      <tbody>
        {completedHands && completedHands.length > 0 ? completedHands.map((hand, idx) => {
          const whoPlayer = hand.bidder;
          const whoTrump = hand.trump;
          const PlayerIcon = whoPlayer ? PlayerIcons[whoPlayer as Player] : undefined;
          const DealerIcon = hand.dealer ? PlayerIcons[hand.dealer as Player] : undefined;
          const TrumpIcon = whoTrump ? SuitIcons[whoTrump as Suit] : undefined;

          // Helper for meld/tricks display
          const showMeld = (meld?: number | null) => (meld != null && meld >= 20 ? meld : '-');
          const showTricks = (tricks?: number | null) => (tricks != null && tricks >= 20 ? tricks : '-');
          const showTotal = (total?: number | null) => (total != null ? total : 0);

          return (
            <React.Fragment key={hand.hand_id}>
              <tr>
                <td className="px-2 py-1 border border-gray-400 align-middle" rowSpan={3} style={{ verticalAlign: 'middle' }}>{idx + 1}</td>
                <td className="px-2 py-1 border border-gray-400 align-middle">{showMeld(hand.us_meld)}</td>
                <td className="px-2 py-1 border border-gray-400 align-middle">{showMeld(hand.them_meld)}</td>
                <td className="px-2 py-1 border border-gray-400 align-middle" rowSpan={3} style={{ verticalAlign: 'middle' }}>
                  <span className="inline-flex items-center gap-1">
                    {hand.bid_amount ?? '-'}
                    {PlayerIcon && <PlayerIcon className="inline text-lg" />}
                    {TrumpIcon && <TrumpIcon className="inline text-lg" />}
                  </span>
                </td>
                <td className="px-2 py-1 border border-gray-400 align-middle" rowSpan={3} style={{ verticalAlign: 'middle' }}>
                  {DealerIcon && <DealerIcon className="inline text-lg" />}
                </td>
              </tr>
              <tr>
                <td className="px-2 py-1 border border-gray-400 align-middle">{showTricks(hand.us_tricks)}</td>
                <td className="px-2 py-1 border border-gray-400 align-middle">{showTricks(hand.them_tricks)}</td>
              </tr>
              <tr>
                <td className="px-2 py-1 border border-gray-400 align-middle font-semibold">{showTotal(hand.us_total)}</td>
                <td className="px-2 py-1 border border-gray-400 align-middle font-semibold">{showTotal(hand.them_total)}</td>
              </tr>
            </React.Fragment>
          );
        }) : (
          <tr>
            <td colSpan={5} className="py-4 text-gray-500">No completed hands yet.</td>
          </tr>
        )}
      </tbody>
    </table>
  </div>
);

export default HandsTableCard;
