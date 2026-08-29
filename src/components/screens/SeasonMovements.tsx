import { useEffect, useState } from "react";
import { api, type MovementRow } from "../../api";
import { EmptyState, Panel, StatusBadge } from "../ui";

export default function SeasonMovements() {
  const [rows, setRows] = useState<MovementRow[]>([]);
  const [loading, setLoading] = useState(true);
  useEffect(() => { api.getSeasonMovements().then(setRows).catch(() => setRows([])).finally(() => setLoading(false)); }, []);
  return <div className="mx-auto max-w-5xl space-y-4 p-4 sm:p-6">
    <header><p className="text-xs font-bold uppercase tracking-[0.2em] text-fm-accent">Carrera larga</p><h2 className="text-2xl font-black">Ascensos y descensos</h2><p className="mt-1 text-sm text-fm-dim">Historial de movimientos entre divisiones.</p></header>
    <Panel className="overflow-hidden"><div className="border-b border-fm-border px-4 py-3 text-xs font-bold uppercase tracking-widest text-fm-dim">Movimientos registrados</div>
      {loading ? <div className="p-8 text-center text-fm-dim">Cargando historial…</div> : rows.length === 0 ? <div className="p-4"><EmptyState title="Todavía no hay movimientos" description="Aparecerán al finalizar la primera temporada." /></div> : <div className="overflow-x-auto"><table className="w-full text-sm"><thead className="bg-fm-bg text-left text-xs uppercase tracking-widest text-fm-dim"><tr><th className="px-4 py-3">Temporada</th><th className="px-4 py-3">Club</th><th className="px-4 py-3">Movimiento</th><th className="px-4 py-3">Desde</th><th className="px-4 py-3">Hacia</th><th className="px-4 py-3">Pos.</th></tr></thead><tbody>{rows.map((r, i) => <tr key={`${r.season}-${r.club_id}-${r.movement_type}-${i}`} className="border-t border-fm-border hover:bg-fm-panel2"><td className="px-4 py-3 font-mono text-xs">{r.season}</td><td className="px-4 py-3 font-semibold">{r.club_name}</td><td className="px-4 py-3"><StatusBadge tone={r.movement_type === "promotion" ? "success" : "danger"}>{r.movement_type === "promotion" ? "Ascenso" : "Descenso"}</StatusBadge></td><td className="px-4 py-3 text-xs text-fm-dim">{r.from_competition}</td><td className="px-4 py-3 text-xs text-fm-dim">{r.to_competition}</td><td className="px-4 py-3 text-center font-mono">{r.source_position}</td></tr>)}</tbody></table></div>}
    </Panel>
  </div>;
}
