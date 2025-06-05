import axios from "axios";
import type {Game, RunningTotal} from "../types/Game.ts";

const API_HOST = import.meta.env.VITE_API_HOST;
const API_BASE = `${API_HOST}/api/games`;

const apiClient = axios.create({
    baseURL: API_BASE,
    headers: {
        'Content-Type': 'application/json',
        'Accept': 'application/json'
    },
});


export const gameApi = {
    async createGame(): Promise<Game> {
        const response = await apiClient.post('/', {
            dealer: 'South'
        });

        return response.data;
    },

    async getGame(gameId: string): Promise<Game> {
        const response = await apiClient.get(`/${gameId}`);
        return response.data;
    },

    async getRunningTotal(gameId: string) : Promise<RunningTotal> {
        const response = await apiClient.get(`${gameId}/running_total`);

        return response.data;
    },

    async startHand (gameId: string): Promise<Game> {
        const response = await apiClient.post(`/${gameId}/start_hand`);

        return response.data;
    },

    async recordBid(gameId: string, bidder: string, amount: number) : Promise<Game> {
        const response = await apiClient.post(`/${gameId}/record_bid`, {
            bidder,
            amount
        });

        return response.data;
    }
}