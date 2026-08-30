import { create } from "zustand";
import type { ClubRow, CompRow, GameStateRow } from "./api";

type Screen = "newgame" | "dashboard" | "squad" | "standings" | "fixtures" | "tactics" | "market" | "scouting" | "inbox" | "training" | "youth" | "finance" | "saves" | "editor";

interface AppStore {
  screen: Screen;
  gameState: GameStateRow | null;
  clubs: ClubRow[];
  competitions: CompRow[];
  selectedComp: number | null;
  userClubId: number | null;
  setScreen: (s: Screen) => void;
  setGameState: (g: GameStateRow | null) => void;
  setClubs: (c: ClubRow[]) => void;
  setCompetitions: (c: CompRow[]) => void;
  setSelectedComp: (id: number | null) => void;
  setUserClub: (id: number | null) => void;
  resetSession: () => void;
}

export const useStore = create<AppStore>((set) => ({
  screen: "newgame",
  gameState: null,
  clubs: [],
  competitions: [],
  selectedComp: null,
  userClubId: null,
  setScreen: (screen) => set({ screen }),
  setGameState: (gameState) => set({ gameState }),
  setClubs: (clubs) => set({ clubs }),
  setCompetitions: (competitions) => set({ competitions }),
  setSelectedComp: (selectedComp) => set({ selectedComp }),
  setUserClub: (userClubId) => set({ userClubId }),
  resetSession: () => set({ gameState: null, userClubId: null, clubs: [], competitions: [], selectedComp: null, screen: "newgame" }),
}));
