import { useEffect, useState } from "react";
import { useStore } from "./store";
import { api } from "./api";
import NewGame from "./components/screens/NewGame";
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
import EditorView from "./components/screens/EditorView";
import YouthView from "./components/screens/YouthView";
import SeasonMovements from "./components/screens/SeasonMovements";
import HonoursView from "./components/screens/HonoursView";
import SavesView from "./components/screens/SavesView";
import StatisticsView from "./components/screens/StatisticsView";
import RecordsView from "./components/screens/RecordsView";
import NationalTeamsView from "./components/screens/NationalTeamsView";
import AppShell from "./components/AppShell";

function Shell({ children }: { children: React.ReactNode }) {
  const { screen, setScreen, gameState, userClubId } = useStore();
  const [unread, setUnread] = useState(0);

  useEffect(() => {
    if (!gameState || !userClubId) return;
    const t = setInterval(async () => {
      try { const inbox = await api.getInbox(); setUnread(inbox.filter((m)=>m.is_read===0).length); } catch {}
    }, 4000);
    api.getInbox().then((inbox)=> setUnread(inbox.filter((m)=>m.is_read===0).length)).catch(()=>{});
    return () => clearInterval(t);
  }, [gameState, userClubId]);

  if (!gameState || !userClubId) return <>{children}</>;

  const items: { id: typeof screen; label: string; badge?: number }[] = [
    { id: "dashboard", label: "Dashboard" },
    { id: "squad", label: "Plantilla" },
    { id: "standings", label: "Clasificación" },
    { id: "fixtures", label: "Calendario" },
    { id: "tactics", label: "Partido" },
    { id: "market", label: "Mercado" },
    { id: "scouting", label: "Ojeo" },
    { id: "training", label: "Entreno" },
    { id: "youth", label: "Cantera" },
    { id: "finance", label: "Finanzas" },
    { id: "inbox", label: "Buzón", badge: unread },
    ...(!gameState ? [{ id: "editor" as typeof screen, label: "Editor" }] : []),
  ];

  return (
    <AppShell unread={unread}>
      <header className="sticky top-0 z-10 border-b border-fm-border bg-fm-panel/95 backdrop-blur">
        <div className="mx-auto flex max-w-6xl items-center justify-between px-4 py-3 lg:px-6">
          <div className="flex items-center gap-3">
            <span className="text-sm font-black tracking-tight"><span className="text-fm-accent">FM</span>27</span>
            <span className="hidden text-xs text-fm-dim md:inline">{gameState.game_date} · {gameState.season} · {gameState.user_club_name}</span>
          </div>
          <nav className="flex flex-wrap gap-1">
            {items.map((it) => (
              <button key={it.id} onClick={() => setScreen(it.id)} className={`relative rounded-lg px-2.5 py-1.5 text-xs font-semibold lg:text-sm ${screen===it.id ? "bg-fm-accent text-black" : "text-fm-dim hover:bg-fm-bg hover:text-white"}`}>
                {it.label}
                {it.badge ? <span className="absolute -right-1 -top-1 rounded-full bg-red-500 px-1 py-0 text-[10px] font-bold text-white">{it.badge}</span> : null}
              </button>
            ))}
            <button onClick={() => setScreen("newgame")} className="ml-1 rounded-lg border border-fm-border px-2.5 py-1.5 text-xs text-fm-dim hover:text-white">Salir</button>
          </nav>
        </div>
      </header>
      <main>{children}</main>
    </AppShell>
  );
}

function MatchFlow() {
  const { userClubId } = useStore();
  const [fixture, setFixture] = useState<any | null>(null);
  const [loaded, setLoaded] = useState(false);
  const [snap, setSnap] = useState<any | null>(null);

  useEffect(() => {
    if (!userClubId) { setLoaded(true); return; }
    api.getNextFixture(userClubId).then((n)=> setFixture(n)).finally(()=>setLoaded(true));
  }, [userClubId]);

  if (snap) return <LiveMatch initial={snap} onBackToSetup={()=> setSnap(null)} />;
  if (!loaded) return <div className="p-8 text-center text-fm-dim">Cargando…</div>;
  if (!fixture) return <LiveMatch />;
  return <TacticsSetup matchId={fixture.id} onStart={(s)=> setSnap(s)} onBack={()=> setSnap(null)} />;
}

export default function App() {
  const { screen, gameState } = useStore();
  // El Editor solo está disponible cuando no hay una partida en curso.
  if ((screen as string) === "editor" && !gameState) {
    return (
      <div className="min-h-screen bg-fm-bg">
        <div className="sticky top-0 z-10 border-b border-fm-border bg-fm-panel/95 backdrop-blur">
          <div className="mx-auto flex max-w-6xl items-center justify-between px-4 py-3">
            <span className="text-sm font-black"><span className="text-fm-accent">FM</span>27 · Editor</span>
            <button onClick={()=>useStore.getState().setScreen("newgame")} className="rounded-lg border border-fm-border px-3 py-1.5 text-sm">← Volver</button>
          </div>
        </div>
        <EditorView />
      </div>
    );
  }
  return (
    <Shell>
      {screen === "newgame" && <NewGame />}
      {screen === "dashboard" && <Dashboard />}
      {screen === "squad" && <SquadView />}
      {screen === "standings" && <StandingsView />}
      {(screen as string) === "statistics" && <StatisticsView />}
      {(screen as string) === "records" && <RecordsView />}
      {(screen as string) === "national-teams" && <NationalTeamsView />}
      {screen === "fixtures" && <FixturesView />}
      {screen === "tactics" && <MatchFlow />}
      {screen === "market" && <MarketView />}
      {screen === "scouting" && <ScoutingView />}
      {screen === "inbox" && <InboxView />}
      {screen === "training" && <TrainingView />}
      {screen === "youth" && <YouthView />}
      {screen === "finance" && <FinanceView />}
      {(screen as string) === "movements" && <SeasonMovements />}
      {(screen as string) === "honours" && <HonoursView />}
      {screen === "saves" && <SavesView />}
      {(screen as string) === "editor" && <EditorView />}
    </Shell>
  );
}
