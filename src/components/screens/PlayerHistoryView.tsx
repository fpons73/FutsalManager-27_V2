import { useEffect, useState } from "react";
import { api, type PlayerHistoryRow } from "../../api";

export default function PlayerHistoryView({ playerId, playerName }: { playerId: number; playerName: string }) {
  const [rows, setRows] = useState<PlayerHistoryRow[]>([]);
  useEffect(() => { api.getPlayerHistory(playerId).then(setRows).catch(() => setRows([])); }, [playerId]);
  const [metric, setMetric] = useState<"ability"|"morale">("ability");
  const [season, setSeason] = useState("all");
  const seasons = [...new Set(rows.map(r => r.season))];
  const visible = season === "all" ? rows : rows.filter(r => r.season === season);
  const max = Math.max(100, ...rows.map(r => metric === "morale" ? r.morale : r.potential_ability));
  return <section className="rounded-xl border border-fm-border bg-fm-panel p-4"><div className="mb-3 flex items-center justify-between"><div><div className="text-xs uppercase tracking-widest text-fm-accent">Evolución</div><h3 className="font-bold">{playerName}</h3></div><div className="flex gap-2"><select value={metric} onChange={e=>setMetric(e.target.value as "ability"|"morale")} className="rounded border border-fm-border bg-fm-bg px-2 py-1 text-xs"><option value="ability">CA / PA</option><option value="morale">Moral</option></select><select value={season} onChange={e=>setSeason(e.target.value)} className="rounded border border-fm-border bg-fm-bg px-2 py-1 text-xs"><option value="all">Todas</option>{seasons.map(s=><option key={s} value={s}>{s}</option>)}</select></div></div>{rows.length===0 ? <p className="text-sm text-fm-dim">Aún no hay cierres de temporada registrados.</p> : <div className="space-y-3">{visible.map(r=><div key={r.season} className="grid grid-cols-[70px_1fr_70px] items-center gap-3 text-xs"><span className="text-fm-dim">{r.season}</span><div className="space-y-1"><div className="h-2 overflow-hidden rounded bg-fm-bg"><div className="h-full rounded bg-fm-accent" style={{width:`${Math.min(100,(metric === "morale" ? r.morale : r.current_ability)/max*100)}%`}} /></div>{metric === "ability" && <div className="h-2 overflow-hidden rounded bg-fm-bg"><div className="h-full rounded bg-fm-sky-400" style={{width:`${Math.min(100,r.potential_ability/max*100)}%`}} /></div>}</div><span className="text-right font-mono">{metric === "morale" ? `Moral ${r.morale}` : <>CA {r.current_ability}<br/>PA {r.potential_ability}</>}</span></div>)}</div>}</section>;
}
