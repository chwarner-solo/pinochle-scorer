import React from 'react';
import { TricksEntryBox } from './TricksEntryBox';

type HandEntryTricksProps = {
  usTricks: number;
  themTricks: number;
  setUsTricks: (val: number) => void;
  setThemTricks: (val: number) => void;
  onSubmit: () => void;
  submitting: boolean;
};

export const HandEntryTricks: React.FC<HandEntryTricksProps> = ({
  usTricks,
  themTricks,
  setUsTricks,
  setThemTricks,
  onSubmit,
  submitting,
}) => (
  <div className="flex flex-col items-center justify-center w-full px-6 py-3">
    <span className="text-lg font-semibold text-gray-700 mb-2">Enter Tricks:</span>
    <TricksEntryBox
      usTricks={usTricks}
      themTricks={themTricks}
      setUsTricks={setUsTricks}
      setThemTricks={setThemTricks}
      onSubmit={onSubmit}
      submitting={submitting}
    />
  </div>
);