import React from 'react';

import type {HandEntryBidProps} from '../types/component/user_interaction_zone';
import BidEntryBox from "./BidEntryBox.tsx";

export const HandEntryBid: React.FC<HandEntryBidProps> = ({ bidEntryBoxProps }) => {

  return (
      <div className="flex flex-col items-center justify-center w-full px-6 py-3">
        <span className="text-lg font-semibold text-gray-700 mb-2">Enter Bidder Seat & Bid:</span>
        <BidEntryBox{...bidEntryBoxProps} />
      </div>
  );
};
