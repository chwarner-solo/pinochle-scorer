import axios from "axios";
import type {Game, GameState, HandState, RunningTotal} from "../types/Game.ts";
import type {BidFormData, FormData, MeldFormData, TricksFormData, TrumpFormData} from "../types/form_types.ts";

export interface GameApi {
    createGame(): Promise<Game>;
};

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
    async createGame(): Promise<Game | null> {
        const response = await apiClient.post('/', {
            dealer: 'South'
        });

        return response.data;
    },

    async getGame(gameId: string | undefined): Promise<Game | null> {
        const response = await apiClient.get(`/${gameId}`);
        return response.data;
    },

    async getRunningTotal(gameId: string) : Promise<RunningTotal> {
        const response = await apiClient.get(`${gameId}/running_total`);

        return response.data;
    },

    async startHand (gameId: string | undefined): Promise<Game | null> {
        const response = await apiClient.post(`/start_hand`, {
            game_id: gameId
        });

        return response.data;
    },

    async recordBid(gameId: string, {player, bid}:BidFormData) : Promise<Game> {
        const response = await apiClient.post(`/${gameId}/record_bid`, {
            player,
            bid
        });

        return response.data;
    },

    async declareTrump(gameId: string, {trump}: TrumpFormData) : Promise<Game> {
        console.log('declaring trump - API Call');
        const response = await apiClient.post(`/${gameId}/declare_trump`, {
            trump
        });

        console.log(response.data);
        return response.data;
    },

    async recordMeld(gameId: string, {us_meld, them_meld}: MeldFormData) : Promise<Game> {
        const response = await apiClient.post(`/${gameId}/record_meld`, {
            us_meld,
            them_meld
        });

        return response.data;
    },

    async recordTricks(gameId: string, {us_tricks, them_tricks}: TricksFormData) : Promise<Game> {
        const response = await apiClient.post(`/${gameId}/record_tricks`, {
            us_tricks,
            them_tricks
        });

        return response.data;
    }

}
export type HandApi = {
    [K in HandState]: (gameId: string, formData: FormData[K]) => Promise<Game | null>;
};

export const handApi: HandApi = {
    async NoHand(gameId: string, _formData: {}) {
        return gameApi.startHand(gameId);
    },
    WaitingForBid: gameApi.recordBid,
    WaitingForTrump: gameApi.declareTrump,
    WaitingForMeld: gameApi.recordMeld,
    WaitingForTricks: gameApi.recordTricks,
    NoMarriage: gameApi.recordMeld,
    Completed: async(_gameId: string, _formData:{}) => { return null; }
}

export const realGameApi: ApiCallMap = {
    NoGame: gameApi.createGame,
    WaitingToStart: gameApi.startHand,
    Completed: gameApi.getGame,
    InProgress: handApi
};

export type ApiCallMap = {
    [K in Exclude<GameState, "InProgress">]: (gameId?: string | undefined) => Promise<Game | null>

} & {
    InProgress: HandApi
}