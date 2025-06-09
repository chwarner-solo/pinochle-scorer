import React from 'react';
import type { Player, Suit } from '../types/Game';

type PlayerIconMap = {
    [K in Player]: React.FC<{ className?: string }>
};

type SuitIconMap = {
    [K in Suit]: React.FC<{ className?: string }>
}

// Unicode arrow icons for player directions (as React components)
export const PlayerIcons: PlayerIconMap = {
  North: ({ className }) => (
    <span className={className}>↓</span> // North points South
  ),
  South: ({ className }) => (
    <span className={className}>↑</span> // South points North
  ),
  East: ({ className }) => (
    <span className={className}>←</span> // East points West
  ),
  West: ({ className }) => (
    <span className={className}>→</span> // West points East
  ),
};

// Suit icons as React.FC for color flexibility
export const SuitIcons: SuitIconMap = {
  Spades: ({ className }) => <span className={className}>♠</span>,
  Hearts: ({ className }) => <span className={className ? className + ' ' : '' + 'text-red-500'}>♥</span>,
  Diamonds: ({ className }) => <span className={className ? className + ' ' : '' + 'text-red-500'}>♦</span>,
  Clubs: ({ className }) => <span className={className}>♣</span>,
  NoMarriage: ({ className }) => <span className={className ? className + ' ' : '' + 'text-gray-400'}>×</span>,
};
