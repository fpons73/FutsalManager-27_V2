import { useEffect, useState } from "react";
import { api, type RecordRow } from "../../api";
import { useStore } from "../../store";

export default function RecordsView() {
  const { userClubId, competitions } = useStore();
  const [rows, setRows] = useState<RecordRow[]>([]);
  const [mode, setMode] = useState<"club"|"competition">("club");
  const [competitionId, setCompetitionId] = useState(competitions[0]?.id ?? 1);
  useEffect(() => { if (!userClubId) return; (mode === "club" ? api.getClubRecords(userClubId) : api.getCompetitionRecords(competitionId)).then(setRows).catch(() => setRows([])); }, [mode, competitionId, userClubId]);
  return <div className="mx-auto max-w-5xl space-y-4 p-4 lg:p-6"><div className="flex flex-wrap items-center justify-between gap-3"><div><div className="text-xs uppercase tracking-widest text-fm-accent">Palmarés e historial</div><h2 className="text-2xl font-black">Récords históricos</h2></div><div className="flex gap-2"><select value={mode} onChange={e=>setMode(e.target.value as "club"|"competition")} className="rounded-lg border border-fm-border bg-fm-panel px-3 py-2 text-sm"><option value="club">Mi club</option><option value="competition">Competición</option></select>{mode === "competition" && <select value={competitionId} onChange={e=>setCompetitionId(Number(e.target.value))} className="rounded-lg border border-fm-border bg-fm-panel px-3 py-2 text-sm">{competitions.map(c=><option key={c.id} value={c.id}>{c.name}</option>)}</select>}</div></div><div className="grid gap-3 sm:grid-cols-2 lg:grid-cols-3">{rows.map((r,i)=><div key={`${r.record_type}-${r.season}-${i}`} className="rounded-xl border border-fm-border bg-fm-panel p-4"><div className="text-xs uppercase tracking-widest text-fm-dim">{r.season}</div><div className="mt-1 font-bold">{r.record_type.replaceAll("_"," ")}</div><div className="mt-2 text-2xl font-black text-fm-accent">{r.value}</div>{r.player_name&&<div className="text-xs text-fm-dim">{r.player_name}</div>}{r.club_name&&<div className="text-xs text-fm-dim">{r.club_name}</div>}</div>)}</div>{rows.length===0&&<div className="rounded-xl border border-fm-border bg-fm-panel p-8 text-center text-fm-dim">Todavía no hay récords registrados.</div>}</div>;
}
