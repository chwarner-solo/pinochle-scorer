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
        <pre style={{ background: '#eee', padding: 8, borderRadius: 4, overflowX: 'auto' }}>{game?.game_state || 'No game'}</pre>
      </div>
      <div>
        <strong>Hand State:</strong>
        <pre style={{ background: '#eee', padding: 8, borderRadius: 4, overflowX: 'auto' }}>{hand?.state || 'No hand'}</pre>
      </div>
    </div>
  );
};

export default GameHandAdminPanel;
