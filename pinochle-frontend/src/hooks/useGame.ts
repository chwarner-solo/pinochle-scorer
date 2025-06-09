import type {Game, GameState, Hand, HandState, Player} from "../types/Game.ts";
import {useState} from "react";
import type {ApiCallMap} from "../services/api.ts";
import type {AnyFormData, BidFormData, FormData} from "../types/form_types.ts";
import {validationMap} from "../validation/handValidation";

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
    onGameSubmit: (data?: any) => Promise<void>;
    onHandSubmit: (data?: AnyFormData) => Promise<void>;
    onResetGame: () => void;
    onResetHand: () => void;
}

type GameActionMap = {
    [K in GameState]: (data?: any) => Promise<void>;
}

function createNewGame (
    state: GameState,
   api: ApiCallMap,
   setLoading: (value: boolean) => void,
   setError: (value: string | null) => void,
   setGame: (value: Game | null) => void,
   setState: (value: GameState) => void,
    setRequiredTricks: (value: number) => void
): (data?: any) => Promise<void> {
    return async (_data?: any) => {
        console.log("Creating new game");
        console.log("Current state:", state);
        if (state !== 'NoGame') { return;}
        setLoading(true);
        setError(null);
        try {
            const newGame = await api[state](undefined)??null;
            setGame(newGame);
            console.log("New game:", newGame);
            console.log("New game state:", newGame?.game_state || 'Not Set')
            setState(newGame?.game_state || 'NoGame');
            setRequiredTricks(newGame?.required_tricks || 0);
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
    setHandState: (value: HandState | null) => void,
    setRequiredTricks: (value: number) => void
): (data?: any) => Promise<void> {
    return async (_data?: any) => {
        // Allow new hand if game is WaitingToStart, or if game is InProgress and last hand is Completed
        const handIsCompleted = game?.hand?.state === 'Completed';
        console.log("Hand State: ", game?.hand?.state);
        console.log("Game State: ", game?.game_state);
        if (!(state === 'WaitingToStart' || (state === 'InProgress' && handIsCompleted))) {
            console.log("Cannot create hand in state:", state, "hand completed:", handIsCompleted);
            return;
        }
        setLoading(true);
        setError(null);
        try {
            console.log("Creating new hand");
            let apiCall;
            if (state === 'InProgress') {
                console.log("In Pgogress state");
                // Use startHand if hand is completed, otherwise use the hand phase function
                if (handIsCompleted) {
                    apiCall = api['WaitingToStart'];
                } else {
                    apiCall = api[state][game?.hand?.state];
                }
            } else {
                console.log("Waiting to start state");
                apiCall = api[state];
            }
            console.log("apiCall defined?", typeof apiCall === 'function' ? 'Yes' : 'No');
            const newGame = await apiCall(game?.game_id) ?? null;
            console.log("New Game:", newGame);
            console.log("New Game State: ", newGame?.game_state);
            console.log("New Hand State: ", newGame?.hand?.state);
            setGame(newGame || null);
            setState(newGame?.game_state || 'NoGame');
            setHand(newGame?.hand || null);
            setHandState(newGame?.hand?.state || null);
            setRequiredTricks(newGame?.required_tricks || 0);
        } catch (e: any) {
            setError(e.message || "Failed to create hand");
        } finally {
            setLoading(false);
        }
    }
}

// Generic hand phase submit handler
function submitHandPhase(
    api: ApiCallMap,
    game: Game | null,
    handState: HandState | null,
    setLoading: (value: boolean) => void,
    setError: (value: string | null) => void,
    setGame: (value: Game | null) => void,
    setHand: (value: Hand | null) => void,
    setHandState: (value: HandState | null) => void,
    setTrump: (value?: string) => void,
    setRequiredTricks: (value: number) => void
): (formData: AnyFormData) => Promise<void> {
    return async (formData: AnyFormData) => {

        if (!game || !game.game_id || !handState) return;

        const errors = validationMap[handState](formData as any);

        if (Object.keys(errors).length > 0) {
            console.log("Validation errors:", errors);
            setError(JSON.stringify(errors)); // You may want to set this as an object in real usage
            return;
        }

        setError(null);
        setLoading(true);

        try {
            const newGame = await api[game.game_state][handState](game.game_id, formData);

            setGame(newGame || null);
            setHand(newGame?.hand || null);
            setHandState(newGame?.hand?.state || null);
            setTrump(newGame?.trump || '--');
            setRequiredTricks(newGame?.hand?.required_tricks || 0);
        } catch (e: any) {
            setError(e.message || "Failed to submit form");
        } finally {
            setLoading(false);
        }
    }
}

export const useGame = (api: ApiCallMap) : UseGameReturn => {
    const [state, setState] = useState<GameState>('NoGame');
    const [handState, setHandState] = useState<HandState>('NoHand');
    const [game, setGame] = useState<Game | null>(null);
    const [hand, setHand] = useState<Hand | null>(null);
    const [loading, setLoading] = useState(false);
    const [error, setError] = useState<string | null>(null);
    const [usScore, setUsScore] = useState(0);
    const [themScore, setThemScore] = useState(0);
    const [trump, setTrump] = useState('--');
    const [requiredTricks, setRequiredTricks] = useState(0);

    const gameActionMap: GameActionMap = {
        NoGame: createNewGame(state, api, setLoading, setError, setGame, setState, setRequiredTricks),
        WaitingToStart: createNewHand(state, api, game, setLoading, setError, setGame, setState, setHand, setHandState, setRequiredTricks),
        InProgress: async() => {}, // Not used for game-level
        Completed: createNewGame(state, api, setLoading, setError, setGame, setState, setRequiredTricks),
    };

    // Compose onGameSubmit for game phases
    let onGameSubmit: (data?: any) => Promise<void> = gameActionMap[state];
    // Compose onHandSubmit for hand phases
    let onHandSubmit: (data?: AnyFormData) => Promise<void> = async () => {};
    const effectiveHandState = handState ?? 'NoHand';
    if (state === 'InProgress' && effectiveHandState !== 'NoHand' && effectiveHandState !== 'Completed') {
        onHandSubmit = submitHandPhase(api, game, effectiveHandState, setLoading, setError, setGame, setHand, setHandState, setTrump, setRequiredTricks);
    } else if (effectiveHandState === 'NoHand' || effectiveHandState === 'Completed') {
        onHandSubmit = createNewHand(state, api, game, setLoading, setError, setGame, setState, setHand, setHandState, setRequiredTricks);
    }

    const formData: AnyFormData | undefined = undefined; // TODO: Populate with actual form data as needed

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
        onGameSubmit,
        onHandSubmit,
        onResetGame: () => { setGame(null); setState('NoGame'); },
        onResetHand: () => { setHand(null); setHandState('NoHand'); },
        loading: loading,
        error: error
    };
}
