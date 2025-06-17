import type {Game, GameState} from "../types/Game.ts";
import {useState, useEffect} from "react";
import type {ApiCallMap} from "../services/api.ts";
import type {AnyFormData} from "../types/form_types.ts";
import {validationMap} from "../validation/handValidation";
import type { UserInteractionZoneProps } from '../types/component/user_interaction_zone';

// --- Types for Seat Mappings ---
type Seat = 'N' | 'E' | 'S' | 'W';
type PlayerToSeatMap = { [K in Game["players"][number]]: Seat };
type SeatToPlayerMap = { [K in Seat]: Game["players"][number] };

// Helper to map full player name to seat code
const playerToSeatMap: PlayerToSeatMap = {
  North: 'N',
  East: 'E',
  South: 'S',
  West: 'W',
};
const playerToSeat = (player?: string) => player && playerToSeatMap[player as keyof typeof playerToSeatMap] ? playerToSeatMap[player as keyof typeof playerToSeatMap] : '';

// Helper to map seat code to full player name
const seatToPlayerMap: SeatToPlayerMap = {
  N: 'North',
  E: 'East',
  S: 'South',
  W: 'West',
};
const seatToPlayer = (seat: string) => {
  if (seatToPlayerMap[seat as Seat]) return seatToPlayerMap[seat as Seat];
  throw new Error('Invalid seat');
};

// --- Helper to get seat code for a player, fallback to '' ---
const getBidderSeat = (bidder: string | undefined): Seat | '' => {
  if (!bidder) return '';
  if (["N", "E", "S", "W"].includes(bidder)) return bidder as Seat;
  return playerToSeatMap[bidder as keyof typeof playerToSeatMap] || '';
};

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
    onGameSubmit: (data?: any) => Promise<void>;
    onHandSubmit: (data?: AnyFormData) => Promise<void>;
    onResetGame: () => void;
    completedHands: Game[];
    selectedSeat: string;
    setSelectedSeat: (value: string) => void;
    bid: number;
    setBid: (value: number) => void;
    submitting: boolean;
    selectedTrump: string;
    setSelectedTrump: (value: string) => void;
    submittingTrump: boolean;
    usMeld: number;
    setUsMeld: (value: number) => void;
    themMeld: number;
    setThemMeld: (value: number) => void;
    submittingMeld: boolean;
    usTricks: number;
    setUsTricks: (value: number) => void;
    themTricks: number;
    setThemTricks: (value: number) => void;
    submittingTricks: boolean;
    handleSubmitBid: () => Promise<void>;
    handleTrumpClick: (suit: string) => Promise<void>;
    handleSubmitMeld: () => Promise<void>;
    handleSubmitTricks: () => Promise<void>;
    playerToSeat: (player?: string) => string;
    seatToPlayer: (seat: string) => string;
    getBidderSeat: (bidder: string | undefined) => Seat | '';
    resetGame: () => void;
    usPrevScore: number;
    themPrevScore: number;
    user_interaction_zone: UserInteractionZoneProps;
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
    setRequiredTricks: (value: number) => void,
    setUsScore: (value: number) => void,
    setThemScore: (value: number) => void
): (data?: any) => Promise<void> {
    return async (_data?: any) => {
        if (state !== 'NoGame') { return;}
        setLoading(true);
        setError(null);
        try {
            const newGame = await api[state](undefined)??null;
            setGame(newGame);
            setState(newGame?.game_state || 'NoGame');
            setRequiredTricks(newGame?.required_tricks || 0);
            setUsScore(newGame?.us_score ?? 0);
            setThemScore(newGame?.them_score ?? 0);
        } catch (e: any) {
            setError(e.message || "Failed to create game");
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
): (data?: any) => Promise<void> {
    return async (_data?: any) => {
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
        const errors = validationMap[handState](formData as any);
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
        } catch (e: any) {
            setError(e.message || "Failed to submit form");
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

    // --- Track previous scores for US and THEM ---
    const [usPrevScore, setUsPrevScore] = useState(0);
    const [themPrevScore, setThemPrevScore] = useState(0);

    // Update previous scores when a new hand is started (when hand transitions from Completed to WaitingForBid)
    useEffect(() => {
        if (game?.hand_state === 'WaitingForBid' && game?.previous_hand_state === 'Completed') {
            setUsPrevScore(game?.us_score ?? 0);
            setThemPrevScore(game?.them_score ?? 0);
        }
    }, [game?.hand_state, game?.us_score, game?.them_score, game?.previous_hand_state]);

    // Optionally, also update when a new game starts
    useEffect(() => {
        if (state === 'NoGame' || state === 'WaitingToStart') {
            setUsPrevScore(0);
            setThemPrevScore(0);
        }
    }, [state]);

    // Extended state for UI
    const [selectedSeat, setSelectedSeat] = useState('');
    const [bid, setBid] = useState(50);
    const [submitting, setSubmitting] = useState(false);
    const [selectedTrump, setSelectedTrump] = useState('');
    const [submittingTrump, setSubmittingTrump] = useState(false);
    const [usMeld, setUsMeld] = useState(0);
    const [themMeld, setThemMeld] = useState(0);
    const [submittingMeld, setSubmittingMeld] = useState(false);
    const [usTricks, setUsTricks] = useState(0);
    const [themTricks, setThemTricks] = useState(0);
    const [submittingTricks, setSubmittingTricks] = useState(false);

    // Helper to fetch completed hands
    const fetchCompletedHands = async (gameId: string) => {
        try {
            console.log("Fetching completed hands for game ID:", gameId);
            const resp = await api.getCompletedHands(gameId);
            setCompletedHands(resp);
        } catch (e) {
            console.log("Error fetching completed hands:", e)
            setCompletedHands([]);
        }
    };

    // Update completed hands only when game is InProgress and hand_state is WaitingForBid
    useEffect(() => {
        if (game?.game_id && game.game_state === 'InProgress' && game.hand_state === 'WaitingForBid') {
            fetchCompletedHands(game.game_id);
        }
    }, [game?.game_id, game?.game_state, game?.hand_state]);

    useEffect(() => {
        if (!game?.bidder) setSelectedSeat('');
        else setSelectedSeat(playerToSeat(game.bidder));
    }, [game?.bidder]);

    useEffect(() => {
        if (game?.hand_state === 'WaitingForTrump') {
            setSelectedTrump('');
        }
    }, [game?.hand_state]);

    useEffect(() => {
        if (game?.hand_state === 'Complete' || state === 'Complete') {
            setSelectedTrump('');
        }
    }, [game?.hand_state, state]);

    useEffect(() => {
        if (game?.hand_state === 'WaitingForMeld') {
            setUsMeld(0);
            setThemMeld(0);
        }
    }, [game?.hand_state]);

    useEffect(() => {
        if (game?.hand_state === 'WaitingForTricks') {
            setUsTricks(0);
            setThemTricks(0);
        }
    }, [game?.hand_state]);

    const gameActionMap: GameActionMap = {
        NoGame: createNewGame(state, api, setLoading, setError, setGame, setState, setRequiredTricks, setUsScore, setThemScore),
        WaitingToStart: createNewHand(state, api, game, setLoading, setError, setGame, setState, setTrump),
        InProgress: async() => {}, // Not used for game-level
        Completed: createNewGame(state, api, setLoading, setError, setGame, setState, setRequiredTricks, setUsScore, setThemScore),
    };

    // Compose onGameSubmit for game phases
    let onGameSubmit: (data?: any) => Promise<void> = gameActionMap[state];
    // Compose onHandSubmit for hand phases
    let onHandSubmit: (data?: AnyFormData) => Promise<void> = async () => {};
    if (state === 'InProgress' && game?.hand_state !== 'NoHand' && game?.hand_state !== 'Completed' && game?.hand_state != null) {
        onHandSubmit = submitHandPhase(api, game, setLoading, setError, setGame, setState, setTrump, setRequiredTricks, setUsScore, setThemScore);
    } else if (isHandReadyForNew(game?.hand_state)) {
        onHandSubmit = createNewHand(state, api, game, setLoading, setError, setGame, setState, setTrump);
    }

    const handleSubmitBid = async () => {
        setSubmitting(true);
        try {
            const bidForm = { bid, player: seatToPlayer(selectedSeat) };
            await onHandSubmit(bidForm);
        } finally {
            setSubmitting(false);
        }
    };

    const handleTrumpClick = async (suit: string) => {
        setSelectedTrump(suit);
        setSubmittingTrump(true);
        try {
            await onHandSubmit({ trump: suit });
        } finally {
            setSubmittingTrump(false);
        }
    };

    const handleSubmitMeld = async () => {
        setSubmittingMeld(true);
        try {
            await onHandSubmit({ us_meld: usMeld, them_meld: themMeld });
        } finally {
            setSubmittingMeld(false);
        }
    };

    const handleSubmitTricks = async () => {
        setSubmittingTricks(true);
        try {
            await onHandSubmit({ us_tricks: usTricks, them_tricks: themTricks });
        } finally {
            setSubmittingTricks(false);
        }
    };

    const formData: AnyFormData | undefined = undefined; // TODO: Populate with actual form data as needed

    // --- User Interaction Zone Views Structure ---
    const views = {
        NoGame: {
            onSubmit: onGameSubmit,
            dealer: selectedSeat || "South",
            loading,
        },
        WaitingToStart: {
            onSubmit: onHandSubmit,
            loading,
        },
        InProgress: {
            WaitingForBid: {
                bidEntryBoxProps: {
                    selected: selectedSeat,
                    onSelect: setSelectedSeat,
                    bid,
                    setBid,
                    submitting,
                },
                onSubmit: handleSubmitBid,
            },
            WaitingForTrump: {
                formData: { trump: selectedTrump },
                onHandSubmit: (data) => handleTrumpClick(data.trump),
                loading: submittingTrump,
                error,
            },
            WaitingForMeld: {
                usMeld,
                themMeld,
                setUsMeld,
                setThemMeld,
                onSubmit: handleSubmitMeld,
                submitting: submittingMeld,
            },
            WaitingForTricks: {
                usTricks,
                themTricks,
                setUsTricks,
                setThemTricks,
                onSubmit: handleSubmitTricks,
                submitting: submittingTricks,
            },
            Completed: {
                trump,
                usHandScore: game?.us_hand_score ?? 0,
                themHandScore: game?.them_hand_score ?? 0,
                usTotal: game?.us_score ?? 0,
                themTotal: game?.them_score ?? 0,
                bidder: game?.bidder ?? "",
                bid: game?.bid_amount ?? 0,
                reqTricks: game?.required_tricks ?? 0,
                onStartHand: async () => {
                    if (!game?.game_id) return;
                    await onHandSubmit({});
                },
                usPrev: usPrevScore,
                themPrev: themPrevScore,
                usMeld: game?.us_meld ?? 0,
                themMeld: game?.them_meld ?? 0,
                usTricks: game?.us_tricks ?? 0,
                themTricks: game?.them_tricks ?? 0,
            },
        },
        Completed: {
            onSubmit: onGameSubmit,
            loading,
        },
    };

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
        selectedSeat, setSelectedSeat,
        bid, setBid,
        submitting,
        selectedTrump, setSelectedTrump,
        submittingTrump,
        usMeld, setUsMeld,
        themMeld, setThemMeld,
        submittingMeld,
        usTricks, setUsTricks,
        themTricks, setThemTricks,
        submittingTricks,
        handleSubmitBid,
        handleTrumpClick,
        handleSubmitMeld,
        handleSubmitTricks,
        playerToSeat,
        seatToPlayer,
        getBidderSeat,
        usPrevScore,
        themPrevScore,
        resetGame: () => { setGame(null); setState('NoGame'); },
        user_interaction_zone: {
            gameState: state,
            handState: game?.hand_state || 'NoHand',
            views,
        },
    };
}
