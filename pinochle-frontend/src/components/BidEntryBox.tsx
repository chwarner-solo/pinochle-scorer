import React from "react";

interface BidEntryBoxProps {
  selected: string;
  onSelect: (seat: string) => void;
  bid: number;
  setBid: (amt: number) => void;
  onSubmit: () => void;
  submitting: boolean;
}

const SEATS = ["N", "E", "S", "W"];

const BidEntryBox: React.FC<BidEntryBoxProps> = ({
  selected,
  onSelect,
  bid,
  setBid,
  onSubmit,
  submitting,
}) => (
  <form
    className="flex flex-row items-center gap-4 justify-center"
    onSubmit={e => {
      e.preventDefault();
      onSubmit();
    }}
  >
    <div className="flex flex-row gap-2">
      {SEATS.map(seat => (
        <button
          key={seat}
          type="button"
          className={`px-4 py-2 rounded-lg font-bold border-2 focus:outline-none transition-all duration-150 ${
            selected === seat
              ? "bg-blue-500 text-white border-blue-700 shadow"
              : "bg-white text-gray-700 border-gray-300 hover:bg-blue-100"
          }`}
          onClick={() => onSelect(seat)}
          disabled={submitting}
        >
          {seat}
        </button>
      ))}
    </div>
    <div className="flex flex-row items-center gap-2">
      <label className="text-lg font-semibold text-gray-700" htmlFor="bid-amount">
        Bid:
      </label>
      <input
        id="bid-amount"
        type="number"
        min={50}
        max={250}
        step={1}
        value={bid}
        onChange={e => setBid(Number(e.target.value))}
        className="border px-2 py-1 rounded w-24 text-center"
        disabled={submitting}
      />
    </div>
    <button
      type="submit"
      className="bg-green-600 hover:bg-green-700 text-white font-bold py-1 px-6 rounded shadow text-base"
      disabled={submitting || !selected}
    >
      Submit Bid
    </button>
  </form>
);

export default BidEntryBox;
