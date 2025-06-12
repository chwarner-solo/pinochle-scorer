import React from 'react';

export type TableSeatButtonProps = {
  seat: string;
  pos: any;
  isDealer: boolean;
  isBidder: boolean;
  isSelected: boolean;
  onClick?: (seat: string) => void;
};

export const DealerBadge = () => (
  <span className="absolute -top-2 -right-2 bg-orange-400 rounded-full p-0.5 flex items-center justify-center shadow" title="Dealer">
    <svg width="14" height="14" viewBox="0 0 20 20" fill="white">
      <polygon points="10,2 12.59,7.36 18.51,7.63 13.97,11.27 15.45,17.02 10,13.89 4.55,17.02 6.03,11.27 1.49,7.63 7.41,7.36" />
    </svg>
  </span>
);

export const BidderBadge = () => (
  <span className="absolute -top-2 -left-2 bg-blue-500 rounded-full p-0.5 flex items-center justify-center shadow" title="Bidder">
    <svg width="14" height="14" viewBox="0 0 20 20" fill="white">
      <circle cx="10" cy="10" r="6" />
    </svg>
  </span>
);

export const TableSeatButton: React.FC<TableSeatButtonProps> = ({ seat, pos, isDealer, isBidder, isSelected, onClick }) => {
  const positionStyle = {
    ...(pos.top !== undefined ? { top: pos.top } : {}),
    ...(pos.bottom !== undefined ? { bottom: pos.bottom } : {}),
    ...(pos.left !== undefined ? { left: pos.left } : {}),
    ...(pos.right !== undefined ? { right: pos.right } : {}),
    transform: `translate(${pos.translate})`,
  };

  const dealerClass = isDealer ? 'border-4 border-orange-400' : 'border-gray-400';
  const selectedClass = isSelected ? 'bg-blue-600 border-blue-700 shadow' : 'bg-white text-gray-700 border-gray-300 hover:bg-blue-100';

  const buttonClass = `absolute bg-white border rounded-full w-12 h-12 flex items-center justify-center shadow-md text-lg font-bold select-none active:bg-blue-100 transition-all duration-150 ${dealerClass} ${selectedClass}`;

  return (
    <button
      key={seat}
      onClick={() => onClick && onClick(seat)}
      className={buttonClass}
      style={positionStyle}
      tabIndex={-1}
    >
      <Chair seat={seat} isDealer={isDealer} isBidder={isBidder} />
    </button>
  );
};

export const Chair = (props: { seat: string; isDealer: boolean; isBidder: boolean }) => {
  const { seat, isDealer, isBidder } = props;
  // Debug log
  console.log('Chair render:', { seat, isDealer, isBidder });
  return (
    <span className="relative flex items-center justify-center w-full h-full">
      <span className="font-bold text-base bg-white/70 rounded px-1" style={{textShadow: '0 1px 2px #fff, 0 -1px 2px #fff'}}>
        {seat}
      </span>
      {isDealer && <DealerBadge />}
      {isBidder && <BidderBadge />}
    </span>
  );
};
