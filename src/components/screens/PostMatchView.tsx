import { useEffect, useState } from "react";
import { api, type MatchSummary } from "../../api";

export default function PostMatchView({ clubId }: { clubId: number }) {
  const [summary, setSummary] = useState<MatchSummary | null>(null);
  const [loading, setLoading] = useState(true);
  useEffect(() => { api.getLastMatchSummary(clubId).then(setSummary).catch(()=>setSummary(null)).finally(()=>setLoading(false)); }, [clubId]);
  if (loading) return <div className="rounded-xl border border-fm-border bg-fm-panel p-4 text-sm text-fm-dim">Cargando último partido…</div>;
  if (!summary) return <div className="rounded-xl border border-fm-border bg-fm-panel p-4 text-sm text-fm-dim">Todavía no hay partidos finalizados.</div>;
  const result = summary.home.score === summary.away.score ? "Empate" : (summary.home.id === clubId ? summary.home.score > summary.away.score : summary.away.score > summary.home.score) ? "Victoria" : "Derrota";
  const resultClass = result === "Victoria" ? "text-emerald-400" : result === "Derrota" ? "text-red-400" : "text-amber-400";
  return <div className="rounded-xl border border-fm-border bg-fm-panel p-4">
    <div className="mb-3 flex items-center justify-between"><div><div className="text-xs uppercase tracking-widest text-fm-dim">Último partido · J {summary.round}</div><div className="text-xs text-fm-dim">{summary.competition} · {summary.date}</div>{summary.went_to_penalties ? <div className="mt-1 text-xs font-bold text-fm-accent">Desempate por penaltis: {summary.penalty_home_score}-{summary.penalty_away_score}</div> : summary.went_to_extra_time ? <div className="mt-1 text-xs font-bold text-amber-400">Desempate en prórroga</div> : null}</div><span className={`font-black ${resultClass}`}>{result}</span></div>
    <div className="grid grid-cols-[1fr_auto_1fr] items-center gap-3 rounded-xl bg-fm-bg p-4 text-center"><div className="text-left font-bold">{summary.home.name}<div className="text-xs text-fm-dim">{summary.home.short_name}</div></div><div className="font-mono text-3xl font-black">{summary.home.score} <span className="text-fm-dim">—</span> {summary.away.score}</div><div className="text-right font-bold">{summary.away.name}<div className="text-xs text-fm-dim">{summary.away.short_name}</div></div></div>
    <div className="mt-3 grid grid-cols-4 gap-2 text-center text-xs"><div><div className="font-mono font-bold">{summary.home.possession}% — {summary.away.possession}%</div><div className="text-fm-dim">Posesión</div></div><div><div className="font-mono font-bold">{summary.home.shots} — {summary.away.shots}</div><div className="text-fm-dim">Tiros</div></div><div><div className="font-mono font-bold">{summary.home.shots_on_target} — {summary.away.shots_on_target}</div><div className="text-fm-dim">A puerta</div></div><div><div className="font-mono font-bold">{summary.home.fouls} — {summary.away.fouls}</div><div className="text-fm-dim">Faltas</div></div></div>
    <div className="mt-4"><div className="mb-2 text-xs font-bold uppercase tracking-widest text-fm-dim">Momentos clave</div><div className="max-h-32 space-y-1 overflow-auto">{summary.events.slice().reverse().slice(0,8).map((event,i)=><div key={i} className="rounded bg-fm-bg px-2 py-1 text-xs"><span className="mr-2 font-mono text-fm-accent">{event.minute}'</span>{event.description}</div>)}{summary.events.length===0 && <div className="text-xs text-fm-dim">Sin eventos registrados.</div>}</div></div>
  </div>;
}
