import { useEffect, useState } from "react";
import { api, type FinanceRow } from "../../api";
import { EmptyState, Panel } from "../ui";

export default function CommercialView() {
  const [finance, setFinance] = useState<FinanceRow | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const load = () => { setLoading(true); setError(null); api.getFinance().then(setFinance).catch((e) => setError(String(e))).finally(() => setLoading(false)); };
  useEffect(load, []);
  const money = (value: number) => `€${Math.round(value).toLocaleString()}`;
  if (loading) return <div className="mx-auto max-w-6xl p-6"><Panel className="p-8 text-center text-fm-dim">Cargando área comercial…</Panel></div>;
  if (error) return <div className="mx-auto max-w-6xl p-6"><Panel className="p-8 text-center text-rose-300">{error}<button onClick={load} className="mt-3 block mx-auto rounded-lg bg-fm-accent px-3 py-1.5 text-sm font-bold text-black">Reintentar</button></Panel></div>;
  if (!finance) return <EmptyState title="Sin datos comerciales" description="No hay una partida activa." />;
  return <div className="mx-auto max-w-6xl space-y-5 p-4 lg:p-6">
    <header><div className="text-xs font-bold uppercase tracking-[0.2em] text-fm-accent">Club · ingresos comerciales</div><h1 className="text-2xl font-black">Área Comercial</h1><p className="mt-1 text-sm text-fm-dim">Gestiona los acuerdos que financian la temporada de {finance.club_name}.</p></header>
    <div className="grid gap-4 lg:grid-cols-2">
      <Panel className="overflow-hidden"><div className="border-b border-fm-border bg-fm-bg px-4 py-3"><div className="text-xs uppercase tracking-widest text-fm-dim">Derechos televisivos</div><h2 className="mt-1 text-lg font-black">{finance.tv_broadcaster ?? "Sin contrato"}</h2></div><div className="grid gap-3 p-4 text-sm sm:grid-cols-2"><div><span className="text-fm-dim">Ingreso semanal</span><strong className="mt-1 block text-xl text-fm-accent">{money(finance.tv_weekly_income)}</strong></div><div><span className="text-fm-dim">Vencimiento</span><strong className="mt-1 block">{finance.tv_contract_end ?? "—"}</strong></div></div><div className="border-t border-fm-border px-4 py-3 text-xs text-fm-dim">La negociación de nuevas cadenas estará disponible al finalizar el contrato actual.</div></Panel>
      <Panel className="overflow-hidden"><div className="border-b border-fm-border bg-fm-bg px-4 py-3"><div className="text-xs uppercase tracking-widest text-fm-dim">Patrocinio principal</div><h2 className="mt-1 text-lg font-black">{finance.sponsor_name ?? "Sin patrocinador"}</h2></div><div className="grid gap-3 p-4 text-sm sm:grid-cols-3"><div><span className="text-fm-dim">Ingreso semanal</span><strong className="mt-1 block text-xl text-fm-accent">{money(finance.sponsor_weekly_income)}</strong></div><div><span className="text-fm-dim">Objetivo</span><strong className="mt-1 block">Top {finance.sponsor_target ?? "—"}</strong></div><div><span className="text-fm-dim">Vencimiento</span><strong className="mt-1 block">{finance.sponsor_contract_end ?? "—"}</strong></div></div><div className="border-t border-fm-border px-4 py-3 text-xs text-fm-dim">Cumple el objetivo deportivo para proteger tu relación comercial.</div></Panel>
    </div>
    <Panel className="border-cyan-400/20 bg-cyan-500/5 p-4"><div className="text-sm font-bold text-cyan-200">Próximamente</div><p className="mt-1 text-sm text-fm-dim">Podrás comparar ofertas, negociar duración e importe, y aceptar acuerdos con distintos objetivos y riesgos.</p></Panel>
  </div>;
}
