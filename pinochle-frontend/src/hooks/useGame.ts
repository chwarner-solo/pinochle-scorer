import type {Game, GameState} from "../types/Game.ts";
import {useState, useEffect, useCallback} from "react";
import type {ApiCallMap} from "../services/api.ts";
import type {AnyFormData} from "../types/form_types.ts";
import {validationMap} from "../validation/handValidation";

export interface UseGameReturn {
    game: Game | null;
    state: GameState;
    handState: string | undefined;
    formData: AnyFormData | undefined;
    usScore: number;
    themScore: number;
    trump: string;
    requiredTricks: number;
    isNewGameAvailable: boolean;
    isNewHandAvailable: boolean;
    loading: boolean;
    error: string | null;
    onGameSubmit: (data?: unknown) => Promise<void>;
    onHandSubmit: (data?: AnyFormData) => Promise<void>;
    onResetGame: () => void;
    completedHands: Game[];
    selectedSeat: string;
    setSelectedSeat: (seat: string) => void;
    bid: number;
    setBid: (amt: number) => void;
    submittingBid: boolean;
    handleSubmitBid: () => void;
    selectedTrump: string;
    setSelectedTrump: (trump: string) => void;
    submittingTrump: boolean;
    onTrumpClick: (suit: string) => void;
    usMeld: number;
    themMeld: number;
    setUsMeld: (value: number) => void;
    setThemMeld: (value: number) => void;
    submittingMeld: boolean;
    handleSubmitMeld: () => void;
    usTricks: number;
    themTricks: number;
    setUsTricks: (value: number) => void;
    setThemTricks: (value: number) => void;
    submittingTricks: boolean;
    handleSubmitTricks: () => void;
    onStartHand: () => void;
}

type GameActionMap = {
    [K in GameState]: (data?: unknown) => Promise<void>;
}

function createNewGame (
    state: GameState,
   api: ApiCallMap,
   setLoading: (value: boolean) => void,
   setError: (value: string | null) => void,
   setGame: (value: Game | null) => void,
   setState: (value: GameState) => void,
    setRequiredTricks: (value: number) => void,
    setUsScore: (value: number) => void,
    setThemScore: (value: number) => void
): (data?: unknown) => Promise<void> {
    return async () => {
        console.log("Creating new game");
        console.log("Current state:", state);
        if (state !== 'NoGame') { return;}
        console.log("Creating new game");
        setLoading(true);
        setError(null);
        try {
            console.log("Calling api[state]");
            console.log(api[state])
            const newGame = await api[state](undefined)??null;
            setGame(newGame);
            console.log("New game state: ", newGame?.game_state || 'Not Set')
            console.log("New hand state: ", newGame?.hand_state || 'Not Set')
            setState(newGame?.game_state || 'NoGame');
            setRequiredTricks(newGame?.required_tricks || 0);
            setUsScore(newGame?.us_score ?? 0);
            setThemScore(newGame?.them_score ?? 0);
        } catch (e: unknown) {
            console.log("Error creating game:", e);
            const errorMessage = e instanceof Error ? e.message : "Failed to create game";
            setError(errorMessage);
        } finally {
            setLoading(false);
        }
    }
}

function createNewHand(
    game_state: GameState,
    api: ApiCallMap,
    game: Game | null,
    setLoading: (value: boolean) => void,
    setError: (value: string | null) => void,
    setGame: (value: Game | null) => void,
    setState: (value: GameState) => void,
    setTrump: (value?: string) => void
): (data?: unknown) => Promise<void> {
    return async () => {
        // Allow new hand if game is WaitingToStart, or if game is InProgress and hand_state is Completed or NoHand
        const canStartHand =
            game_state === 'WaitingToStart' ||
            (game_state === 'InProgress' && (game?.hand_state === 'Completed' || game?.hand_state === 'NoHand'));

        if (!canStartHand) {
            console.log("Cannot create hand in state:", game_state, "hand_state:", game?.hand_state);
            return;
        }
        setLoading(true);
        setError(null);
        try {
            const newGame = await api['WaitingToStart'](game?.game_id) ?? null;
            setGame(newGame || null);
            setState(newGame?.game_state || 'NoGame');
            setTrump(newGame?.trump || '--');
        } catch (e: unknown) {
            const errorMessage = e instanceof Error ? e.message : "Failed to create hand";
            setError(errorMessage);
        } finally {
            setLoading(false);
        }
    }
}

// Generic hand phase submit handler
function submitHandPhase(
    api: ApiCallMap,
    game: Game | null,
    setLoading: (value: boolean) => void,
    setError: (value: string | null) => void,
    setGame: (value: Game | null) => void,
    setState: (value: GameState) => void,
    setTrump: (value?: string) => void,
    setRequiredTricks: (value: number) => void,
    setUsScore: (value: number) => void,
    setThemScore: (value: number) => void
): (formData: AnyFormData) => Promise<void> {
    return async (formData: AnyFormData) => {
        if (!game || !game.game_id) return;
        const handState = game.hand_state;
        const errors = validationMap[handState](formData as never);
        if (Object.keys(errors).length > 0) {
            console.log("Validation errors:", errors);
            setError("Validation error");
            return;
        }
        setLoading(true);
        try {
            const newGame = await api[game.game_state][handState](game.game_id, formData);
            setGame(newGame || null);
            setState(newGame?.game_state || 'NoGame');
            setTrump(newGame?.trump || '--');
            setRequiredTricks(newGame?.required_tricks || 0);
            setUsScore(newGame?.us_score ?? 0);
            setThemScore(newGame?.them_score ?? 0);
        } catch (e: unknown) {
            const errorMessage = e instanceof Error ? e.message : "Failed to submit form";
            setError(errorMessage);
        } finally {
            setLoading(false);
        }
    }
}

// Helper to determine if a new hand can be started
function isHandReadyForNew(handState: string | null | undefined) {
    return handState === 'NoHand' || handState === 'Completed' || handState == null;
}

export const useGame = (api: ApiCallMap) : UseGameReturn => {
    const [state, setState] = useState<GameState>('NoGame');
    const [game, setGame] = useState<Game | null>(null);
    const [loading, setLoading] = useState(false);
    const [error, setError] = useState<string | null>(null);
    const [usScore, setUsScore] = useState(0);
    const [themScore, setThemScore] = useState(0);
    const [trump, setTrump] = useState('--');
    const [requiredTricks, setRequiredTricks] = useState(0);
    const [completedHands, setCompletedHands] = useState<Game[]>([]);
    const [selectedSeat, setSelectedSeat] = useState("");
    const [bid, setBid] = useState(50);
    const [submittingBid, setSubmittingBid] = useState(false);
    const [selectedTrump, setSelectedTrump] = useState("");
    const [submittingTrump, setSubmittingTrump] = useState(false);
    const [usMeld, setUsMeld] = useState(0);
    const [themMeld, setThemMeld] = useState(0);
    const [submittingMeld, setSubmittingMeld] = useState(false);
    const [usTricks, setUsTricks] = useState(0);
    const [themTricks, setThemTricks] = useState(0);
    const [submittingTricks, setSubmittingTricks] = useState(false);

    const seatToPlayerMap = { N: "North", E: "East", S: "South", W: "West" };

    // Helper to fetch completed hands
    const fetchCompletedHands = useCallback(async (gameId: string) => {
        try {
            console.log("Fetching completed hands for game ID:", gameId);
            const resp = await api.getCompletedHands(gameId);
            setCompletedHands(resp);
        } catch (e) {
            console.log("Error fetching completed hands:", e)
            setCompletedHands([]);
        }
    }, [api]);

    // Update completed hands only when game is InProgress and hand_state is WaitingForBid
    useEffect(() => {
        if (game?.game_id && game.game_state === 'InProgress' && game.hand_state === 'WaitingForBid') {
            fetchCompletedHands(game.game_id);
        }
    }, [game?.game_id, game?.game_state, game?.hand_state, fetchCompletedHands]);

    const gameActionMap: GameActionMap = {
        NoGame: createNewGame(state, api, setLoading, setError, setGame, setState, setRequiredTricks, setUsScore, setThemScore),
        WaitingToStart: createNewHand(state, api, game, setLoading, setError, setGame, setState, setTrump),
        InProgress: async() => {}, // Not used for game-level
        Completed: createNewGame(state, api, setLoading, setError, setGame, setState, setRequiredTricks, setUsScore, setThemScore),
    };

    // Compose onGameSubmit for game phases
    const onGameSubmit: (data?: unknown) => Promise<void> = gameActionMap[state];
    // Compose onHandSubmit for hand phases
    let onHandSubmit: (data?: AnyFormData) => Promise<void> = async () => {};
    if (state === 'InProgress' && game?.hand_state !== 'NoHand' && game?.hand_state !== 'Completed' && game?.hand_state != null) {
        onHandSubmit = submitHandPhase(api, game, setLoading, setError, setGame, setState, setTrump, setRequiredTricks, setUsScore, setThemScore);
    } else if (isHandReadyForNew(game?.hand_state)) {
        onHandSubmit = createNewHand(state, api, game, setLoading, setError, setGame, setState, setTrump);
    }

    const handleSubmitBid = async () => {
        const player = seatToPlayerMap[selectedSeat] || selectedSeat;
        console.log('[handleSubmitBid] called with:', { selectedSeat, bid, player });
        setSubmittingBid(true);
        try {
            const result = await onHandSubmit({ player, bid });
            console.log('[handleSubmitBid] onHandSubmit result:', result);
        } catch (e) {
            console.error('[handleSubmitBid] error:', e);
        } finally {
            setSubmittingBid(false);
        }
    };

    const handleTrumpClick = async (suit: string) => {
        setSelectedTrump(suit);
        setSubmittingTrump(true);
        try {
            const result = await onHandSubmit({ trump: suit });
            console.log('[handleTrumpClick] onHandSubmit result:', result);
        } catch (e) {
            console.error('[handleTrumpClick] error:', e);
        } finally {
            setSubmittingTrump(false);
        }
    };

    const handleSubmitMeld = async () => {
        setSubmittingMeld(true);
        try {
            const result = await onHandSubmit({ us_meld: usMeld, them_meld: themMeld });
            console.log('[handleSubmitMeld] onHandSubmit result:', result);
        } catch (e) {
            console.error('[handleSubmitMeld] error:', e);
        } finally {
            setSubmittingMeld(false);
        }
    };

    const handleSubmitTricks = async () => {
        setSubmittingTricks(true);
        try {
            const result = await onHandSubmit({
                us_meld: usMeld,
                them_meld: themMeld,
                us_tricks: usTricks,
                them_tricks: themTricks
            });
            console.log('[handleSubmitTricks] onHandSubmit result:', result);
        } catch (e) {
            console.error('[handleSubmitTricks] error:', e);
        } finally {
            setSubmittingTricks(false);
        }
    };

    const handleStartHand = async () => {
        setUsMeld(0);
        setThemMeld(0);
        setUsTricks(0);
        setThemTricks(0);
        setSelectedTrump(""); // Ensure trump is reset
        setSubmittingTrump(false);
        setSelectedSeat("");
        setBid(50); // Ensure bid is reset
        setSubmittingBid(false);
        setSubmittingMeld(false);
        setSubmittingTricks(false);
        await onHandSubmit({}); // Backend call to start new hand
    };

    const formData: AnyFormData | undefined = undefined; // TODO: Populate with actual form data as needed

    return {
        game: game,
        state: state || 'NoGame',
        handState: game?.hand_state || 'NoHand',
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
        loading: loading,
        error: error,
        completedHands: completedHands,
        selectedSeat,
        setSelectedSeat,
        bid,
        setBid,
        submittingBid,
        handleSubmitBid,
        selectedTrump,
        setSelectedTrump,
        submittingTrump,
        onTrumpClick: handleTrumpClick,
        usMeld,
        themMeld,
        setUsMeld,
        setThemMeld,
        submittingMeld,
        handleSubmitMeld,
        usTricks,
        themTricks,
        setUsTricks,
        setThemTricks,
        submittingTricks,
        handleSubmitTricks,
        onStartHand: handleStartHand,
    };
}
