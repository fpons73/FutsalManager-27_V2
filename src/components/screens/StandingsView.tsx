import { useEffect, useState } from "react";
import { api, type StandingRow } from "../../api";
import { useStore } from "../../store";

export default function StandingsView() {
  const { competitions, selectedComp, setSelectedComp, userClubId } = useStore();
  const [rows, setRows] = useState<StandingRow[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [kind, setKind] = useState<"clubs" | "selecciones">("clubs");
  const clubComps = competitions.filter((c)=>c.kind === "club");
  const natComps = competitions.filter((c)=>c.kind === "national_team");
  const opts = kind === "clubs" ? clubComps : natComps;
  const sel = selectedComp && opts.some((c)=>c.id===selectedComp) ? selectedComp : (opts[0]?.id ?? 1);

  useEffect(() => { if (sel) { setLoading(true); setError(null); api.getStandings(sel).then(setRows).catch((e)=>{setRows([]);setError(String(e));}).finally(()=>setLoading(false)); } }, [sel, kind]);

  return (
    <div className="mx-auto max-w-5xl p-6">
      <div className="mb-4 flex flex-wrap items-center justify-between gap-3">
        <h2 className="text-xl font-black">Clasificación</h2>
        <div className="flex items-center gap-2">
          <div className="inline-flex rounded-full border border-fm-border bg-fm-panel p-0.5">
            <button onClick={()=>setKind("clubs")} className={`rounded-full px-3 py-1 text-xs font-bold ${kind==="clubs" ? "bg-fm-accent text-black" : "text-fm-dim"}`}>Clubes</button>
            <button onClick={()=>setKind("selecciones")} className={`rounded-full px-3 py-1 text-xs font-bold ${kind==="selecciones" ? "bg-fm-accent text-black" : "text-fm-dim"}`}>Selecciones</button>
          </div>
          <select value={sel} onChange={(e) => setSelectedComp(Number(e.target.value))} className="rounded-lg border border-fm-border bg-fm-panel px-3 py-1.5 text-sm">
            {opts.map((c) => <option key={c.id} value={c.id}>{c.name} · {c.nation || "Internacional"}</option>)}
          </select>
        </div>
      </div>
      {loading && <div className="mb-3 rounded-xl border border-fm-border bg-fm-panel p-4 text-sm text-fm-dim" aria-live="polite">Cargando clasificación…</div>}
      {error && <div role="alert" className="mb-3 rounded-xl border border-rose-400/30 bg-rose-400/10 p-3 text-sm text-rose-200">{error}</div>}
      <div className="overflow-hidden rounded-xl border border-fm-border bg-fm-panel">
        <table className="w-full text-sm">
          <thead className="bg-fm-bg text-xs uppercase tracking-widest text-fm-dim">
            <tr><th className="px-3 py-2 text-left">#</th><th className="px-2 py-2 text-left">Club</th><th className="px-2 py-2">PJ</th><th className="px-2 py-2">G</th><th className="px-2 py-2">E</th><th className="px-2 py-2">P</th><th className="px-2 py-2">GF</th><th className="px-2 py-2">GC</th><th className="px-2 py-2">DG</th><th className="px-2 py-2">Pts</th></tr>
          </thead>
          <tbody>
            {rows.map((r) => (
              <tr key={r.club_id} className={`border-t border-fm-border ${r.club_id===userClubId ? "bg-fm-accent/10 font-bold" : "hover:bg-fm-panel2"}`}>
                <td className="px-3 py-2 font-mono">{r.position}</td>
                <td className="px-2 py-2">{r.club_name} <span className="text-fm-dim">({r.short_name})</span></td>
                <td className="px-2 py-2 text-center">{r.played}</td>
                <td className="px-2 py-2 text-center text-emerald-400">{r.won}</td>
                <td className="px-2 py-2 text-center text-amber-400">{r.drawn}</td>
                <td className="px-2 py-2 text-center text-red-400">{r.lost}</td>
                <td className="px-2 py-2 text-center">{r.goals_for}</td>
                <td className="px-2 py-2 text-center">{r.goals_against}</td>
                <td className="px-2 py-2 text-center font-mono">{r.goal_difference > 0 ? `+${r.goal_difference}` : r.goal_difference}</td>
                <td className="px-2 py-2 text-center font-black">{r.points}</td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </div>
  );
}
