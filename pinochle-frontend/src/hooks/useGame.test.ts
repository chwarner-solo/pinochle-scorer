import {useGame} from "./useGame.ts";
import {act, renderHook} from "@testing-library/react";
import type {ApiCallMap} from "../services/api.ts";
import {vi, describe, it, expect, beforeEach} from "vitest";

let  mockApi: ApiCallMap;

beforeEach(() => {
    mockApi = {
        Completed: vi.fn().mockResolvedValue({game_id: "1", game_state: "Completed"}),
        InProgress: {
            NoMarriage: vi.fn().mockResolvedValue({game_id: "1", game_state: "InProgress"}),
            NoHand: vi.fn().mockResolvedValue({game_id: "1", game_state: "InProgress"}),
            WaitingForBid: vi.fn().mockResolvedValue({game_id: "1", game_state: "InProgress"}),
            WaitingForTrump: vi.fn().mockResolvedValue({game_id: "1", game_state: "InProgress"}),
            WaitingForMeld: vi.fn().mockResolvedValue({game_id: "1", game_state: "InProgress"}),
            WaitingForTricks: vi.fn().mockResolvedValue({game_id: "1", game_state: "InProgress"}),
            Completed: vi.fn().mockResolvedValue({game_id: "1", game_state: "InProgress"})
        },
        NoGame: vi.fn().mockResolvedValue({game_id: "1", game_state: "WaitingToStart"}),
        WaitingToStart: vi.fn().mockResolvedValue({game_id: "1", game_state: "InProgress"}),
        getCompletedHands: vi.fn().mockResolvedValue([])
    }
});
describe("useGame hook", () => {
    it("should initialize with state 'NoGame'", () => {
        const { result } = renderHook(() => useGame(mockApi));
        expect(result.current.state).toBe("NoGame");
        expect(result.current.game).toBe(null);
    });

    it("Should transition to 'WaitingToStart' after onGameSubmit", async () => {
       const { result } = renderHook(() => useGame(mockApi));
       await act(async () => {
           await result.current.onGameSubmit();
       });
        expect(result.current.state).toBe("WaitingToStart");
       expect(result.current.game).not.toBeNull();
       expect(mockApi.NoGame).toHaveBeenCalled();
    });
});
