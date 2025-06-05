import {Game, RunningTotal} from "../src/types/Games";

const API_BASE = '/api';

export const gameApi = {
    async createGame(): Promise<Game> {
        const response = await fetch(`${API_BASE}/games/`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            }
        });

        if (!response.ok) throw new Error('Failed to create game');

        return response.json();
    },

    async getGame(gameId: string): Promise<Game> {
        const response = await fetch(`${API_BASE}/games/${gameId}`);
        if (!response.ok) throw new Error('Failed to get game');
        return response.json();
    },

    async getRunningTotal(gameId: string) : Promise<RunningTotal> {
        const response = await fetch(`${API_BASE}/games/${gameId}/running_total`);
        if (!response.ok) throw new Error('Failed to get running total');
        return response.json();
    },

    async startHand (gameId: string): Promise<Game> {
        const response = await fetch(`${API_BASE}/games/start_hand`, {
            method: 'POST',
        });
        if (!response.ok) throw new Error('Failed to start hand');
        return response.json();
    }
}