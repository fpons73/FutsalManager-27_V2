import { useEffect, useState } from "react";
import { api, type FixtureRow, type StandingRow } from "../../api";
import { useStore } from "../../store";
import PostMatchView from "./PostMatchView";
import { MetricCard, Panel, StatusBadge } from "../ui";

export default function Dashboard() {
  const { gameState, userClubId, clubs, competitions, selectedComp, setScreen } = useStore();
  const [next, setNext] = useState<FixtureRow | null>(null);
  const [standings, setStandings] = useState<StandingRow[]>([]);
  const [advancing, setAdvancing] = useState(false);
  const [events, setEvents] = useState<string[]>([]);
  const [seasonDone, setSeasonDone] = useState(false);
  const [seasonMsg, setSeasonMsg] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);

  const myClub = clubs.find((c) => c.id === userClubId);
  const myComp = competitions.find((c) => c.id === selectedComp) ?? competitions[0];

  const refresh = async () => {
    if (!userClubId) return;
    const [n, s, done] = await Promise.all([
      api.getNextFixture(userClubId).catch(()=>null),
      myComp ? api.getStandings(myComp.id).catch(()=>[] as StandingRow[]) : Promise.resolve([] as StandingRow[]),
      api.checkSeasonFinished().catch(()=>false)
    ]);
    setNext(n as any);
    setStandings(s as any);
    setError(null);
    setSeasonDone(done as boolean);
  };

  useEffect(() => { refresh(); }, [userClubId, gameState?.game_date, myComp?.id]);

  const advance = async (days: number) => {
    setAdvancing(true);
    try {
      if (days === 1) {
        const r = await api.advanceDay();
        await api.autosaveGame();
        setEvents((e) => [...r.results, ...e].slice(0, 12));
      } else {
        const rs = await api.advanceWeek();
        await api.autosaveGame();
        const all = rs.flatMap((r) => r.results);
        setEvents((e) => [...all, ...e].slice(0, 12));
      }
      const st = await api.getGameState(); useStore.getState().setGameState(st);
      await refresh();
    } catch (e) { setError(String(e)); }
    finally { setAdvancing(false); }
  };

  const rollover = async () => {
    if (!confirm("¿Finalizar temporada y comenzar la siguiente? Se retirarán veteranos y llegarán jóvenes.")) return;
    try {
      const msg = await api.rolloverSeason();
      setSeasonMsg(msg);
      const st = await api.getGameState(); useStore.getState().setGameState(st);
      await refresh();
    } catch(e){ alert(String(e)); }
  };

  if (!gameState || !myClub) return <div className="p-8 text-center text-fm-dim">Cargando…</div>;

  const myStanding = standings.find((s) => s.club_id === userClubId);

  return (
    <div className="mx-auto max-w-6xl space-y-6 p-4 lg:p-6">
      <div className="relative flex flex-wrap items-center justify-between gap-3 overflow-hidden rounded-2xl border border-fm-border bg-fm-panel p-5 shadow-2xl shadow-cyan-950/20"><div className="pointer-events-none absolute -right-16 -top-20 h-48 w-48 rounded-full bg-fm-accent/10 blur-3xl" />
        <div>
          <div className="text-xs uppercase tracking-widest text-fm-dim">{gameState.season} · Jornada {next?.round ?? "—"}</div>
          <div className="text-xs font-bold uppercase tracking-[0.25em] text-fm-accent">Centro de mando</div><div className="text-2xl font-black">{myClub.name} <span className="font-normal text-fm-dim">({myClub.short_name})</span></div>
        </div>
        <div className="flex items-center gap-3">
          <div className="rounded-lg bg-fm-bg px-4 py-2 text-center">
            <div className="text-xs text-fm-dim">Fecha</div>
            <div className="font-mono font-bold">{gameState.game_date}</div>
          </div>
          {myStanding && <StatusBadge tone={myStanding.position <= 3 ? "success" : "default"}>Posición {myStanding.position}</StatusBadge>}
          <button onClick={() => advance(1)} disabled={advancing} className="rounded-lg bg-fm-accent px-4 py-2.5 text-sm font-bold text-black hover:brightness-110 disabled:opacity-50">Avanzar 1 día</button>
          <button onClick={() => advance(7)} disabled={advancing} className="rounded-lg border border-fm-border bg-fm-panel2 px-4 py-2.5 text-sm font-semibold hover:bg-fm-border disabled:opacity-50">+7 días</button>
        </div>
      </div>

      {error && <div role="alert" className="rounded-xl border border-rose-400/30 bg-rose-400/10 p-3 text-sm text-rose-200">{error}</div>}

      {seasonDone && (
        <div className="rounded-xl border border-amber-500/30 bg-amber-500/10 p-4">
          <div className="font-bold text-amber-400">¡Temporada finalizada!</div>
          <div className="mt-1 text-sm text-fm-dim">Todos los partidos se han jugado. Pulsa para generar la siguiente temporada, retirar veteranos y rejuvenecer la plantilla.</div>
          <button onClick={rollover} className="mt-3 rounded-lg bg-amber-500 px-4 py-2 text-sm font-bold text-black">Comenzar temporada siguiente</button>
          {seasonMsg && <div className="mt-2 rounded bg-fm-bg px-3 py-2 font-mono text-xs">{seasonMsg}</div>}
        </div>
      )}

      <div className="grid gap-4 lg:grid-cols-3">
        {userClubId !== null && <PostMatchView clubId={userClubId} />}
        <Panel className="p-4">
          <h3 className="mb-3 text-xs font-bold uppercase tracking-widest text-fm-dim">Próximo partido</h3>
          {next ? (
            <div className="space-y-2">
              <div className="text-xs text-fm-dim">{next.date} · J {next.round}</div>
              <div className="flex items-center justify-between rounded-lg bg-fm-bg p-3">
                <span className={`font-bold ${next.home_id===userClubId ? "text-fm-accent" : ""}`}>{next.home_short}</span>
                <span className="text-fm-dim">vs</span>
                <span className={`font-bold ${next.away_id===userClubId ? "text-fm-accent" : ""}`}>{next.away_short}</span>
              </div>
              <div className="text-xs text-fm-dim">{next.home_name} — {next.away_name}</div>
              <button onClick={()=>setScreen("tactics")} className="mt-2 w-full rounded-lg bg-fm-accent px-3 py-1.5 text-sm font-bold text-black">Ver en vivo →</button>
            </div>
          ) : <div className="text-sm text-fm-dim">Sin partidos pendientes {seasonDone && "— temporada terminada"}</div>}
        </Panel>

        <Panel className="p-4">
          <h3 className="mb-3 text-xs font-bold uppercase tracking-widest text-fm-dim">Clasificación · {myComp?.name ?? ""}</h3>
          <div className="space-y-1 text-sm">
            {standings.slice(0, 6).map((s) => (
              <div key={s.club_id} className={`flex items-center justify-between rounded px-2 py-1 ${s.club_id===userClubId ? "bg-fm-accent/15 font-bold" : "hover:bg-fm-bg"}`}>
                <span className="flex items-center gap-2"><span className="w-5 text-fm-dim">{s.position}.</span> {s.short_name}</span>
                <span className="font-mono">{s.points} pts <span className="text-fm-dim">({s.played} PJ)</span></span>
              </div>
            ))}
            {myStanding && myStanding.position > 6 && (
              <div className="flex items-center justify-between rounded bg-fm-accent/15 px-2 py-1 font-bold">
                <span className="flex items-center gap-2"><span className="w-5 text-fm-dim">{myStanding.position}.</span> {myStanding.short_name} (tú)</span>
                <span className="font-mono">{myStanding.points} pts</span>
              </div>
            )}
          </div>
          <button onClick={()=>setScreen("standings")} className="mt-3 text-xs text-fm-dim hover:text-white">Ver tabla completa →</button>
        </Panel>

        <Panel className="p-4">
          <h3 className="mb-3 text-xs font-bold uppercase tracking-widest text-fm-dim">Últimos resultados</h3>
          {events.length === 0 ? <div className="text-sm text-fm-dim">Avanza días para ver resultados.</div> : (
            <div className="space-y-1.5">
              {events.map((ev, i) => <div key={i} className="rounded bg-fm-bg px-2 py-1.5 font-mono text-xs">{ev}</div>)}
            </div>
          )}
        </Panel>
      </div>

      <div className="grid gap-3 sm:grid-cols-3">
        {myStanding && <MetricCard label="Posición liguera" value={`#${myStanding.position}`} detail={`${myStanding.points} puntos`} />}
        <MetricCard label="Partidos jugados" value={myStanding?.played ?? 0} detail="En la competición activa" tone="text-fm-sky-300" />
        <MetricCard label="Próximo paso" value={next ? `Jornada ${next.round}` : "—"} detail={next ? next.date : "Sin partido pendiente"} tone="text-fm-amber-300" />
      </div>

      <div className="grid gap-4 sm:grid-cols-3">
        <button onClick={()=>setScreen("market")} className="rounded-xl border border-fm-border bg-fm-panel p-4 text-left hover:border-fm-accent/30"><div className="text-xs uppercase tracking-widest text-fm-dim">Mercado</div><div className="font-semibold">Fichajes y ofertas</div><div className="text-xs text-fm-dim">Valoración por CA/edad/potencial</div></button>
        <button onClick={()=>setScreen("training")} className="rounded-xl border border-fm-border bg-fm-panel p-4 text-left hover:border-fm-accent/30"><div className="text-xs uppercase tracking-widest text-fm-dim">Entrenamiento</div><div className="font-semibold">Progreso semanal</div><div className="text-xs text-fm-dim">Jóvenes mejoran más</div></button>
        <button onClick={()=>setScreen("finance")} className="rounded-xl border border-fm-border bg-fm-panel p-4 text-left hover:border-fm-accent/30"><div className="text-xs uppercase tracking-widest text-fm-dim">Finanzas</div><div className="font-semibold">Balance y salarios</div><div className="text-xs text-fm-dim">Taquilla + patrocinio</div></button>
      </div>
    </div>
  );
}
