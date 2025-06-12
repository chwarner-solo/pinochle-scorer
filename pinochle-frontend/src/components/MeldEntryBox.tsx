import React from 'react';

type MeldEntryBoxProps = {
  usMeld: number;
  themMeld: number;
  setUsMeld: (val: number) => void;
  setThemMeld: (val: number) => void;
  onSubmit: () => void;
  submitting: boolean;
};

export const MeldEntryBox: React.FC<MeldEntryBoxProps> = ({ usMeld, themMeld, setUsMeld, setThemMeld, onSubmit, submitting }) => (
  <form className="flex flex-row items-center gap-4 w-full max-w-xl justify-center" onSubmit={e => { e.preventDefault(); onSubmit(); }}>
    <div className="flex flex-row gap-4 w-full max-w-xs justify-between">
      <label className="flex flex-col items-center w-1/2">
        <span className="font-semibold text-gray-700 mb-1">Us</span>
        <input
          type="number"
          min={0}
          value={usMeld}
          onChange={e => setUsMeld(Number(e.target.value))}
          className="border rounded px-2 py-1 w-20 text-center text-lg"
          required
        />
      </label>
      <label className="flex flex-col items-center w-1/2">
        <span className="font-semibold text-gray-700 mb-1">Them</span>
        <input
          type="number"
          min={0}
          value={themMeld}
          onChange={e => setThemMeld(Number(e.target.value))}
          className="border rounded px-2 py-1 w-20 text-center text-lg"
          required
        />
      </label>
    </div>
    <button
      type="submit"
      className="bg-blue-600 hover:bg-blue-700 text-white font-bold py-2 px-6 rounded-lg shadow disabled:opacity-60 disabled:cursor-not-allowed text-lg"
      disabled={submitting}
    >
      {submitting ? 'Submitting...' : 'Submit Meld'}
    </button>
  </form>
);
