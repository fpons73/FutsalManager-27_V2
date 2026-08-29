import { useEffect, useState } from "react";
import { api, type HonourRow } from "../../api";
import { EmptyState, Panel, StatusBadge } from "../ui";
import { useStore } from "../../store";

export default function HonoursView() {
  const { userClubId } = useStore();
  const [rows, setRows] = useState<HonourRow[]>([]);
  const [loading, setLoading] = useState(true);
  useEffect(() => { api.getHonours(userClubId ?? undefined).then(setRows).catch(() => setRows([])).finally(() => setLoading(false)); }, [userClubId]);
  return <div className="mx-auto max-w-5xl space-y-4 p-4 sm:p-6"><header><p className="text-xs font-bold uppercase tracking-[0.2em] text-fm-accent">Historia del club</p><h2 className="text-2xl font-black">Palmarés</h2><p className="mt-1 text-sm text-fm-dim">Títulos conquistados por tu club.</p></header><Panel className="overflow-hidden"><div className="border-b border-fm-border px-4 py-3 text-xs font-bold uppercase tracking-widest text-fm-dim">Títulos</div>{loading ? <div className="p-8 text-center text-fm-dim">Cargando palmarés…</div> : rows.length === 0 ? <div className="p-4"><EmptyState title="Aún no hay títulos registrados" description="Completa una temporada para estrenar el palmarés." /></div> : <div className="grid gap-3 p-4 sm:grid-cols-2">{rows.map((r, i) => <div key={`${r.season}-${r.competition_id}-${i}`} className="rounded-xl border border-fm-border bg-fm-bg p-4"><div className="flex items-center justify-between gap-2"><StatusBadge tone="success">{r.season}</StatusBadge><span className="text-lg">🏆</span></div><div className="mt-3 font-bold">{r.competition}</div><div className="mt-1 text-xs text-fm-dim">{r.honour_type === "league" ? "Campeón de liga" : r.honour_type === "supercup" ? "Supercampeón" : "Campeón de copa"}</div></div>)}</div>}</Panel></div>;
}
