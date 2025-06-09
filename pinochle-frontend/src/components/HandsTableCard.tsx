import React from 'react';
import {PlayerIcons, SuitIcons} from "./IconMaps.tsx";
import type {Player, Suit} from "../types/Game.ts";

// Example/mock data for demonstration
const hands = [
  {
    hand_id: '1',
    bidder: 'North',
    bid_amount: 65,
    trump: 'Hearts',
    us_meld: 30,
    them_meld: 22,
    us_tricks: 27,
    them_tricks: 23,
    us_total: 57,
    them_total: 45,
    dealer: 'South',
  },
  {
    hand_id: '2',
    bidder: 'East',
    bid_amount: 70,
    trump: 'Clubs',
    us_meld: 25,
    them_meld: 20,
    us_tricks: 25,
    them_tricks: 25,
    us_total: 82,
    them_total: 65,
    dealer: 'West',
  },
];

function getPlayerIcon(player: string) : React.FC {
  return PlayerIcons[player as Player];
}

function getSuitIcon(suit: string): React.FC {
  return SuitIcons[suit as Suit];
}

const HandsTableCard: React.FC = () => (
  <div className="bg-white rounded shadow p-6 mb-4 border border-gray-200 text-gray-900 overflow-x-auto">
    <table className="min-w-full text-sm text-center border border-gray-400 border-collapse">
      <thead>
        <tr className="bg-gray-100">
          <th className="px-1 py-2 border border-gray-400 w-10">Hand #</th>
          <th className="px-2 py-2 border border-gray-400">US</th>
          <th className="px-2 py-2 border border-gray-400">THEM</th>
          <th className="px-2 py-2 border border-gray-400">Bid</th>
          <th className="px-2 py-2 border border-gray-400">Who</th>
        </tr>
      </thead>
      <tbody>
        {hands.map((hand, idx) => {
          // Defensive: fallback to null if icon is missing
          const Bidder = PlayerIcons[hand.bidder as Player] || (() => <span>?</span>);
          const Trump = SuitIcons[hand.trump as Suit] || (() => <span>?</span>);
          const Dealer = PlayerIcons[hand.dealer as Player] || (() => <span>?</span>);
          return (
          <React.Fragment key={hand.hand_id}>
            <tr>
              <td rowSpan={3} className="align-middle font-semibold border border-gray-400 w-6 p-0 text-xs">{idx + 1}</td>
              <td className="font-semibold border border-gray-400">{hand.us_meld}</td>
              <td className="font-semibold border border-gray-400">{hand.them_meld}</td>
              <td rowSpan={3} className="align-middle border border-gray-400">
                <span className="inline-flex items-center gap-1">
                  {hand.bid_amount} <span>-</span> <Bidder className="inline" /> <span>-</span> <Trump className="inline" />
                </span>
              </td>
              <td rowSpan={3} className="align-middle border border-gray-400">
                <span className="inline-flex items-center gap-1">
                  <Dealer className="inline" />
                </span>
              </td>
            </tr>
            <tr>
              <td className="border border-gray-400">{hand.us_tricks}</td>
              <td className="border border-gray-400">{hand.them_tricks}</td>
            </tr>
            <tr>
              <td className="border border-gray-400">{hand.us_total}</td>
              <td className="border border-gray-400">{hand.them_total}</td>
            </tr>
          </React.Fragment>
        )})}
      </tbody>
    </table>
  </div>
);

export default HandsTableCard;
