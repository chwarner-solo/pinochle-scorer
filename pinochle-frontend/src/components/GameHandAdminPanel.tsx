import React from "react";
import type { Game, Hand } from "../types/Game";

interface GameHandAdminPanelProps {
  game: Game | null;
  hand: Hand | null;
  onResetGame: () => void;
  onResetHand: () => void;
}

const panelStyle: React.CSSProperties = {
  border: "1px solid #ccc",
  borderRadius: 8,
  padding: 16,
  margin: 16,
  background: "#f9f9f9",
  maxWidth: 600,
};

const buttonStyle: React.CSSProperties = {
  marginRight: 12,
  padding: "6px 20px",
  borderRadius: 4,
  border: "none",
  background: "#1976d2",
  color: "white",
  fontWeight: 600,
  cursor: "pointer",
};

const GameHandAdminPanel: React.FC<GameHandAdminPanelProps> = ({ game, hand, onResetGame, onResetHand }) => {
  return (
    <div style={panelStyle}>
      <h3>Game & Hand Admin Panel</h3>
      <div style={{ marginBottom: 12 }}>
        <button style={buttonStyle} onClick={onResetGame}>
          Reset Game
        </button>
        <button style={buttonStyle} onClick={onResetHand}>
          Reset Hand
        </button>
      </div>
      <div style={{ marginBottom: 12 }}>
        <strong>Game:</strong>
        <pre style={{ background: '#eee', padding: 8, borderRadius: 4, overflowX: 'auto' }}>{JSON.stringify(game, null, 2)}</pre>
      </div>
      <div>
        <strong>Hand:</strong>
        <pre style={{ background: '#eee', padding: 8, borderRadius: 4, overflowX: 'auto' }}>{JSON.stringify(hand, null, 2)}</pre>
      </div>
      <div>
        <strong>Game State:</strong>
        {(() => {
          const validGameStates = [
            'NoGame', 'WaitingToStart', 'InProgress', 'Completed'
          ];
          let gameState = game?.game_state;
          if (!gameState && game === null) gameState = 'NoGame';
          if (!gameState || !validGameStates.includes(gameState)) {
            return <pre style={{ background: '#fee', color: '#c00', padding: 8, borderRadius: 4, fontWeight: 'bold' }}>Not Set</pre>;
          }
          return <pre style={{ background: '#eee', padding: 8, borderRadius: 4 }}>{gameState}</pre>;
        })()}
      </div>
      <div>
        <strong>Hand State:</strong>
        {(() => {
          const validHandStates = [
            'NoHand', 'WaitingForBid', 'WaitingForTrump', 'WaitingForMeld', 'WaitingForTricks', 'Completed', 'NoMarriage'
          ];
          let handState = hand?.state;
          if (!handState && hand === null) handState = 'NoHand';
          if (!handState || !validHandStates.includes(handState)) {
            return <pre style={{ background: '#fee', color: '#c00', padding: 8, borderRadius: 4, fontWeight: 'bold' }}>Not Set</pre>;
          }
          return <pre style={{ background: '#eee', padding: 8, borderRadius: 4 }}>{handState}</pre>;
        })()}
      </div>
    </div>
  );
};

export default GameHandAdminPanel;
