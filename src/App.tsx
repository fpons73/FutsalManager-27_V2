import { useEffect, useState } from "react";
import { useStore } from "./store";
import { api } from "./api";
import NewGame from "./components/screens/NewGame";
import StartMenu from "./components/screens/StartMenu";
import Dashboard from "./components/screens/Dashboard";
import SquadView from "./components/screens/SquadView";
import StandingsView from "./components/screens/StandingsView";
import FixturesView from "./components/screens/FixturesView";
import LiveMatch from "./components/screens/LiveMatch";
import TacticsSetup from "./components/screens/TacticsSetup";
import MarketView from "./components/screens/MarketView";
import ScoutingView from "./components/screens/ScoutingView";
import InboxView from "./components/screens/InboxView";
import TrainingView from "./components/screens/TrainingView";
import FinanceView from "./components/screens/FinanceView";
import CommercialView from "./components/screens/CommercialView";
import EditorView from "./components/screens/EditorView";
import YouthView from "./components/screens/YouthView";
import SeasonMovements from "./components/screens/SeasonMovements";
import HonoursView from "./components/screens/HonoursView";
import SavesView from "./components/screens/SavesView";
import StatisticsView from "./components/screens/StatisticsView";
import RecordsView from "./components/screens/RecordsView";
import NationalTeamsView from "./components/screens/NationalTeamsView";
import BoardView from "./components/screens/BoardView";
import AppShell from "./components/AppShell";

function Shell({ children }: { children: React.ReactNode }) {
  const { gameState, userClubId } = useStore();
  const [unread, setUnread] = useState(0);
  useEffect(() => {
    if (!gameState || !userClubId) return;
    const refresh = () => api.getInbox().then(i => setUnread(i.filter(m => m.is_read === 0).length)).catch(() => {});
    refresh(); const timer = setInterval(refresh, 4000); return () => clearInterval(timer);
  }, [gameState, userClubId]);
  if (!gameState || !userClubId) return <>{children}</>;
  return <AppShell unread={unread}>{children}</AppShell>;
}

function MatchFlow() {
  const { userClubId } = useStore();
  const [fixture, setFixture] = useState<any | null>(null);
  const [loaded, setLoaded] = useState(false);
  const [snap, setSnap] = useState<any | null>(null);
  useEffect(() => { if (!userClubId) { setLoaded(true); return; } api.getNextFixture(userClubId).then(setFixture).finally(() => setLoaded(true)); }, [userClubId]);
  if (snap) return <LiveMatch initial={snap} onBackToSetup={() => setSnap(null)} />;
  if (!loaded) return <div className="p-8 text-center text-fm-dim">Cargando…</div>;
  if (!fixture) return <LiveMatch />;
  return <TacticsSetup matchId={fixture.id} onStart={setSnap} onBack={() => setSnap(null)} />;
}

export default function App() {
  const { screen, gameState } = useStore();
  if (!gameState && (screen as string) === "home") return <StartMenu />;
  if ((screen as string) === "editor" && !gameState) return <div className="min-h-screen bg-fm-bg"><EditorView /></div>;
  if (!gameState && (screen as string) === "saves") return <SavesView />;
  return <Shell>
    {screen === "newgame" && <NewGame />}{screen === "dashboard" && <Dashboard />}{screen === "squad" && <SquadView />}{screen === "standings" && <StandingsView />}{(screen as string) === "statistics" && <StatisticsView />}{(screen as string) === "records" && <RecordsView />}{(screen as string) === "national-teams" && <NationalTeamsView />}{screen === "fixtures" && <FixturesView />}{screen === "tactics" && <MatchFlow />}{screen === "market" && <MarketView />}{screen === "scouting" && <ScoutingView />}{screen === "inbox" && <InboxView />}{screen === "training" && <TrainingView />}{screen === "youth" && <YouthView />}{screen === "finance" && <FinanceView />}{screen === "commercial" && <CommercialView />}{(screen as string) === "board" && <BoardView />}{(screen as string) === "movements" && <SeasonMovements />}{(screen as string) === "honours" && <HonoursView />}{screen === "saves" && <SavesView />}{(screen as string) === "editor" && <EditorView />}
  </Shell>;
}
