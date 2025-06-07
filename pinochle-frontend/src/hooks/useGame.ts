import type {Game, GameState, Hand, HandState} from "../types/Game.ts";
import {useState} from "react";
import type {ApiCallMap} from "../services/api.ts";
import type {AnyFormData} from "../types/form_types.ts";

export interface UseGameReturn {
    game: Game | null;
    hand: Hand | null;
    state: GameState;
    handState: HandState | null;
    formData: AnyFormData | undefined;
    usScore: number;
    themScore: number;
    trump: string;
    requiredTricks: number;
    isNewGameAvailable: boolean;
    isNewHandAvailable: boolean;
    loading: boolean;
    error: string | null;
    onSubmit: ()=>Promise<void>;
}

type GameActionMap = {
    [K in GameState]: () => Promise<void>;
}

type HandActionMap = {
    [K in HandState]: (gameId: string, formData: FormData) => Promise<void>;
}

function createNewGame (
    state: GameState,
   api: ApiCallMap,
   setLoading: (value: boolean) => void,
   setError: (value: string | null) => void,
   setGame: (value: Game | null) => void,
   setState: (value: GameState) => void,
): () => Promise<void> {
    return async () => {
        if (state !== 'NoGame') { return;}
        setLoading(true);
        setError(null);
        try {
            const newGame = await api[state](undefined)??null;
            setGame(newGame);
            setState(newGame?.state || 'NoGame');
        } catch (e: any) {
            setError(e.message || "Failed to create game");
        } finally {
            setLoading(false);
        }
    }
}

function createNewHand(
    state: GameState,
    api: ApiCallMap,
    game: Game | null,
    setLoading: (value: boolean) => void,
    setError: (value: string | null) => void,
    setGame: (value: Game | null) => void,
    setState: (value: GameState) => void,
    setHand: (value: Hand | null) => void,
    setHandState: (value: HandState | null) => void
): () => Promise<void> {
    return async () => {
        if (state !== 'WaitingToStart') {
            console.log("Cannot create hand in state:", state);
            return; }
        setLoading(true);
        setError(null);
        try {
            const newGame = await api[state](game?.game_id)??null;
            setGame(newGame || null);

            console.log(newGame);
            setState(newGame?.state || 'NoGame');
            console.log("Game State: ", newGame?.state);
            setHand(newGame?.hand || null);
            console.log("Hand: ", newGame?.hand)
            setHandState(newGame?.hand?.state || null);
            console.log("Hand State: ", newGame?.hand?.state);
        } catch (e: any) {
            setError(e.message || "Failed to create hand");
        } finally {
            setLoading(false);
        }
    }
}
export const useGame = (api: ApiCallMap) : UseGameReturn => {
    const [state, setState] = useState<GameState>('NoGame');
    const [handState, setHandState] = useState<HandState | null>(null);
    const [game, setGame] = useState<Game | null>(null);
    const [hand, setHand] = useState<Hand | null>(null);
    const [loading, setLoading] = useState(false);
    const [error, setError] = useState<string | null>(null);
    const [usScore, setUsScore] = useState(0);
    const [themScore, setThemScore] = useState(0);
    const [trump, setTrump] = useState('--');
    const [requiredTricks, setRequiredTricks] = useState(0);

    const gameActionMap: GameActionMap = {
        NoGame: createNewGame(state, api, setLoading, setError, setGame, setState),
        WaitingToStart: createNewHand(state, api, game, setLoading, setError, setGame, setState, setHand, setHandState),
        InProgress: async(): Promise<void> => {
            if (state !== 'InProgress') return;
        },
        Completed: createNewGame(state, api, setLoading, setError, setGame, setState),
    };

    const formData: AnyFormData | undefined = undefined;

    return {
        game: game,
        hand: hand,
        state: state,
        handState: handState,
        formData: formData,
        usScore: usScore,
        themScore: themScore,
        trump: trump,
        requiredTricks: requiredTricks,
        isNewGameAvailable: false,
        isNewHandAvailable: false,
        onSubmit: gameActionMap[state],
        loading: loading,
        error: error
    };
}
