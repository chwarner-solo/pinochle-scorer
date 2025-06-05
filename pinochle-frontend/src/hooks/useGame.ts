import type {Game, RunningTotal} from "../types/Game.ts";
import {gameApi} from "../services/api.ts";
import {useState} from "react";

export const useGame = () => {
    const [game, setGame] = useState<Game | null>(null);
    const [runningTotal, setRunningTotal] = useState<RunningTotal>({us_total: 0, them_total: 0});
    const [loading, setLoading] = useState(false);
    const [error, setError] = useState<string>('');

    const createGame = async () => {
        setLoading(true);
        setError('');
        try {
            const newGame = await gameApi.createGame();
            setGame(newGame);
        } catch (e) {
            setError(e.message);
        } finally {
            setLoading(false);
        }
    }

    const startHand = async () => {
        if (!game) return;
        setLoading(true);
        setError('');
        try {
            const updatedGame = await gameApi.startHand(game.id());
            setGame(updatedGame);
        } catch (e) {
            setError(e instanceof Error ? e.message : 'Failed to start hand');
        } finally {
            setLoading(false);
        }
    };

    return {
        game,
        runningTotal,
        loading,
        error,
        createGame,
        startHand
    };
};