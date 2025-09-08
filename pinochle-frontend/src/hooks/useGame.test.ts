import {useGame} from "./useGame.ts";
import {act, renderHook} from "@testing-library/react";
import type {ApiCallMap} from "../services/api.ts";
import {vi, describe, it, expect, beforeEach} from "vitest";
import type {HandState} from "../types/Game.ts";

let  mockApi: ApiCallMap;

beforeEach(() => {
    mockApi = {
        Completed: async (_gameId: string) => {},
        InProgress: async (_gameId: string, _handState: HandState, _formData: any) => {},
        NoGame: vi.fn().mockResolvedValue({game_id: "1", game_state: "WaitingToStart"}),
        WaitingToStart: vi.fn().mockResolvedValue({game_id: "1", game_state: "WaitingToStart"})
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
