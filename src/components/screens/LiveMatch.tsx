import { useEffect, useState, useRef } from "react";
import { api, type MatchSnapshot } from "../../api";
import { useStore } from "../../store";
import FutsalPitch from "../FutsalPitch";
import { Panel, StatusBadge } from "../ui";

export default function LiveMatch({ initial, onBackToSetup }: { initial?: MatchSnapshot | null; onBackToSetup?: () => void }) {
  const { userClubId } = useStore();
  const [snap, setSnap] = useState<MatchSnapshot | null>(initial ?? null);
  const [running, setRunning] = useState(Boolean(initial));
  const [speed, setSpeed] = useState<1 | 2 | 5>(1);
  const intervalRef = useRef<number | null>(null);
  const [actionMsg, setActionMsg] = useState<string | null>(null);
  const [selectedOut, setSelectedOut] = useState<number | null>(null);
  const [tacticalOpen, setTacticalOpen] = useState(false);
  const [formation, setFormation] = useState(0);
  const [tempo, setTempo] = useState(50);
  const [pressing, setPressing] = useState(50);
  const [defensiveLine, setDefensiveLine] = useState(50);
  const [width, setWidth] = useState(50);

  const start = async () => {
    if (!userClubId) return;
    const nxt = await api.getNextFixture(userClubId);
    if (!nxt) { alert("No hay partido pendiente"); return; }
    const s = await api.startLive(nxt.id);
    setSnap(s);
    setRunning(true);
  };

  useEffect(() => {
    if (!running) { if (intervalRef.current) window.clearInterval(intervalRef.current); return; }
    const ms = speed === 1 ? 500 : speed === 2 ? 250 : 120;
    intervalRef.current = window.setInterval(async () => {
      try {
        const s = await api.tickLive(speed * 2);
        setSnap(s);
        if (s.state === "Finished") setRunning(false);
      } catch { setRunning(false); }
    }, ms);
    return () => { if (intervalRef.current) window.clearInterval(intervalRef.current); };
  }, [running, speed]);

  const fmt = (sec: number) => `${String(Math.floor(sec/60)).padStart(2,"0")}:${String(sec%60).padStart(2,"0")}`;
  const myPlayers = snap?.players.filter((p)=>p.team_id===0);
  const onPitch = myPlayers?.filter((p)=>p.on_pitch) ?? [];
  const bench = myPlayers?.filter((p)=>!p.on_pitch) ?? [];
  const alerts = snap ? [
    ...(snap.fouls[0] >= 5 ? ["⚠️ Tu equipo está a una falta del doble penalti"] : []),
    ...(snap.fouls[1] >= 5 ? ["⚠️ El rival está a una falta del doble penalti"] : []),
    ...(snap.powerplay[0] ? ["🔋 Powerplay activo para tu equipo"] : []),
    ...(onPitch.some((p)=>p.stamina < 35) ? ["🔴 Hay jugadores con fatiga crítica"] : []),
  ] : [];
  const makeSub = async (inId:number) => {
    if (selectedOut === null) return;
    try { setSnap(await api.liveSubstitute(0, selectedOut, inId)); setActionMsg("Cambio realizado"); setSelectedOut(null); } catch(e) { setActionMsg(String(e)); }
  };
  const timeout = async () => {
    try { setSnap(await api.liveTimeout(0)); setActionMsg("Tiempo muerto solicitado (60 segundos)"); } catch(e) { setActionMsg(String(e)); }
  };
  const applyTactics = async () => {
    try { setSnap(await api.liveTactics(formation, tempo, pressing, defensiveLine, width)); setActionMsg("Táctica actualizada"); setTacticalOpen(false); } catch(e) { setActionMsg(String(e)); }
  };

  if (!snap) {
    return (
      <div className="mx-auto max-w-6xl p-4 lg:p-6 text-center"><Panel className="p-8">
        <h2 className="mb-4 text-xl font-black">Partido en vivo</h2>
        <p className="mb-4 text-sm text-fm-dim">Simulación 2D con motor Rust: faltas, doble penalti, powerplay y cambios volantes.</p>
        {onBackToSetup ? (
          <button onClick={onBackToSetup} className="rounded-lg bg-fm-accent px-6 py-3 font-bold text-black">Configurar tácticas</button>
        ) : (
          <button onClick={start} className="rounded-lg bg-fm-accent px-6 py-3 font-bold text-black">Iniciar próximo partido</button>
        )}
      </Panel></div>
    );
  }

  return (
    <div className="mx-auto max-w-6xl space-y-4 p-4 lg:p-6">
      <Panel className="flex flex-wrap items-center justify-between gap-3 p-3">
        <div className="flex items-center gap-4">
          <span className="font-mono text-lg font-black">{snap.score[0]} - {snap.score[1]}</span>
          <StatusBadge tone={snap.state === "Finished" ? "success" : "default"}>{fmt(snap.time_seconds)} · {snap.state}</StatusBadge>
          <span className="text-xs text-fm-dim">Faltas {snap.fouls[0]}-{snap.fouls[1]} · Tiros {snap.shots[0]}-{snap.shots[1]} · Pos {snap.possession[0]}%/{snap.possession[1]}%</span>
        </div>
        <div className="flex items-center gap-2">
          <button onClick={() => setRunning(!running)} className="rounded-lg bg-fm-accent px-4 py-1.5 text-sm font-bold text-black">{running ? "Pausar" : "Reanudar"}</button>
          <select value={speed} onChange={(e)=>setSpeed(Number(e.target.value) as any)} className="rounded-lg border border-fm-border bg-fm-bg px-2 py-1.5 text-sm">
            <option value={1}>x1</option>
            <option value={2}>x2</option>
            <option value={5}>x5</option>
          </select>
        </div>
      </Panel>

      <Panel className="overflow-hidden p-2"><FutsalPitch snap={snap} /></Panel>
      <div className="grid gap-2 sm:grid-cols-3"><div className="rounded-lg border border-fm-border bg-fm-panel px-3 py-2 text-xs">Tarjetas: 🟨 {snap.yellow_cards[0]}-{snap.yellow_cards[1]} · 🟥 {snap.red_cards[0]}-{snap.red_cards[1]}</div><div className="rounded-lg border border-fm-border bg-fm-panel px-3 py-2 text-xs">Tiempos: {snap.timeouts_used[0]}/1 · {snap.timeouts_used[1]}/1</div><div className="rounded-lg border border-fm-border bg-fm-panel px-3 py-2 text-xs">Powerplay: {snap.powerplay[0] ? "Tú" : snap.powerplay[1] ? "Rival" : "Inactivo"}</div></div>
      {alerts.length > 0 && <div className="space-y-1">{alerts.map((a)=><div key={a} className="rounded-lg border border-amber-500/30 bg-amber-500/10 px-3 py-2 text-sm text-amber-200">{a}</div>)}</div>}
      {actionMsg && <div className="rounded-lg border border-fm-accent/30 bg-fm-accent/10 px-3 py-2 text-sm">{actionMsg}</div>}
      <div className="grid gap-4 lg:grid-cols-2">
        <div className="rounded-xl border border-fm-border bg-fm-panel p-3">
          <div className="mb-2 flex items-center justify-between"><h3 className="text-xs font-bold uppercase tracking-widest text-fm-dim">Intervención</h3><span className="flex gap-1"><button onClick={()=>setTacticalOpen(!tacticalOpen)} disabled={snap.state === "Finished"} className="rounded bg-sky-500 px-2 py-1 text-xs font-bold text-black disabled:opacity-40">Táctica</button><button onClick={timeout} disabled={!running || snap.state === "Finished"} className="rounded bg-amber-500 px-2 py-1 text-xs font-bold text-black disabled:opacity-40">Tiempo muerto</button></span></div>
          {tacticalOpen && <div className="mb-3 grid gap-2 rounded-lg bg-fm-bg p-2 sm:grid-cols-2"><label className="text-xs">Formación<select value={formation} onChange={e=>setFormation(Number(e.target.value))} className="ml-2 rounded bg-fm-panel px-1 py-1"><option value={0}>3-1</option><option value={1}>4-0</option><option value={2}>2-2</option><option value={3}>5-0</option></select></label><label className="text-xs">Tempo <input type="range" value={tempo} onChange={e=>setTempo(Number(e.target.value))} /></label><label className="text-xs">Presión <input type="range" value={pressing} onChange={e=>setPressing(Number(e.target.value))} /></label><label className="text-xs">Línea <input type="range" value={defensiveLine} onChange={e=>setDefensiveLine(Number(e.target.value))} /></label><label className="text-xs">Amplitud <input type="range" value={width} onChange={e=>setWidth(Number(e.target.value))} /></label><button onClick={applyTactics} className="rounded bg-fm-accent px-2 py-1 text-xs font-bold text-black">Aplicar ajustes</button></div>}
          <div className="mb-2 text-xs text-fm-dim">Selecciona un jugador en pista y después uno del banquillo.</div>
          <div className="grid grid-cols-2 gap-1">{onPitch.map((p)=><button key={p.id} onClick={()=>setSelectedOut(p.id)} className={`rounded px-2 py-1 text-left text-xs ${selectedOut===p.id?"bg-fm-accent text-black":"bg-fm-bg"}`}>Sale #{p.shirt} · {p.role} · {Math.round(p.stamina)}%</button>)}</div>
          {selectedOut !== null && <div className="mt-2 grid grid-cols-2 gap-1">{bench.map((p)=><button key={p.id} onClick={()=>makeSub(p.id)} className="rounded bg-emerald-500/15 px-2 py-1 text-left text-xs text-emerald-300">Entra #{p.shirt} · {p.role}</button>)}</div>}
        </div>
        <div className="rounded-xl border border-fm-border bg-fm-panel p-3">
          <h3 className="mb-2 text-xs font-bold uppercase tracking-widest text-fm-dim">Eventos</h3>
          <div className="max-h-48 space-y-1 overflow-auto">
            {snap.events.slice().reverse().slice(0, 12).map((e,i)=>(
              <div key={i} className="rounded bg-fm-bg px-2 py-1 font-mono text-xs">{String(e.minute).padStart(2,"0")}' {e.kind}: {e.description}</div>
            ))}
          </div>
        </div>
        <div className="rounded-xl border border-fm-border bg-fm-panel p-3">
          <h3 className="mb-2 text-xs font-bold uppercase tracking-widest text-fm-dim">Jugadores en pista</h3>
          <div className="grid grid-cols-2 gap-1 text-xs">
            {snap.players.filter((p)=>p.on_pitch).map((p)=>(
              <div key={p.id} className="flex items-center justify-between rounded bg-fm-bg px-2 py-1">
                <span>#{p.shirt} {p.role} {p.stamina < 40 ? "🔴" : p.stamina < 60 ? "🟡" : "🟢"}</span>
                <span className="font-mono">{Math.round(p.stamina)}%</span>
              </div>
            ))}
          </div>
        </div>
      </div>
    </div>
  );
}
