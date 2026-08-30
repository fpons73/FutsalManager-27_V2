import { useEffect, useState } from "react";
import { api, type FinanceRow, type SponsorshipOffer } from "../../api";
import { EmptyState, Panel } from "../ui";

export default function CommercialView() {
  const [finance, setFinance] = useState<FinanceRow | null>(null);
  const [offers, setOffers] = useState<SponsorshipOffer[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [message, setMessage] = useState<string | null>(null);
  const load = () => { setLoading(true); setError(null); Promise.all([api.getFinance(), api.getSponsorshipOffers()]).then(([f, o]) => { setFinance(f); setOffers(o); }).catch((e) => setError(String(e))).finally(() => setLoading(false)); };
  useEffect(load, []);
  const money = (value: number) => `€${Math.round(value).toLocaleString()}`;
  const accept = async (offer: SponsorshipOffer) => { if (!window.confirm(`¿Firmar con ${offer.sponsor_name} por ${money(offer.weekly_amount)} semanales?`)) return; try { setMessage(await api.acceptSponsorshipOffer(offer.id)); load(); } catch (e) { setError(String(e)); } };
  if (loading) return <div className="mx-auto max-w-6xl p-6"><Panel className="p-8 text-center text-fm-dim">Cargando área comercial…</Panel></div>;
  if (error) return <div className="mx-auto max-w-6xl p-6"><Panel className="p-8 text-center text-rose-300">{error}<button onClick={load} className="mt-3 block mx-auto rounded-lg bg-fm-accent px-3 py-1.5 text-sm font-bold text-black">Reintentar</button></Panel></div>;
  if (!finance) return <EmptyState title="Sin datos comerciales" description="No hay una partida activa." />;
  return <div className="mx-auto max-w-6xl space-y-5 p-4 lg:p-6">
    <header><div className="text-xs font-bold uppercase tracking-[0.2em] text-fm-accent">Club · ingresos comerciales</div><h1 className="text-2xl font-black">Área Comercial</h1><p className="mt-1 text-sm text-fm-dim">Gestiona los acuerdos que financian la temporada de {finance.club_name}.</p>{message && <div className="mt-3 rounded-lg border border-emerald-400/30 bg-emerald-500/10 p-3 text-sm text-emerald-200">{message}</div>}</header>
    <div className="grid gap-4 lg:grid-cols-2">
      <Panel className="overflow-hidden"><div className="border-b border-fm-border bg-fm-bg px-4 py-3"><div className="text-xs uppercase tracking-widest text-fm-dim">Derechos televisivos</div><h2 className="mt-1 text-lg font-black">{finance.tv_broadcaster ?? "Sin contrato"}</h2></div><div className="grid gap-3 p-4 text-sm sm:grid-cols-2"><div><span className="text-fm-dim">Ingreso semanal</span><strong className="mt-1 block text-xl text-fm-accent">{money(finance.tv_weekly_income)}</strong></div><div><span className="text-fm-dim">Vencimiento</span><strong className="mt-1 block">{finance.tv_contract_end ?? "—"}</strong></div></div></Panel>
      <Panel className="overflow-hidden"><div className="border-b border-fm-border bg-fm-bg px-4 py-3"><div className="text-xs uppercase tracking-widest text-fm-dim">Patrocinio principal</div><h2 className="mt-1 text-lg font-black">{finance.sponsor_name ?? "Sin patrocinador"}</h2></div><div className="grid gap-3 p-4 text-sm sm:grid-cols-3"><div><span className="text-fm-dim">Ingreso semanal</span><strong className="mt-1 block text-xl text-fm-accent">{money(finance.sponsor_weekly_income)}</strong></div><div><span className="text-fm-dim">Objetivo</span><strong className="mt-1 block">Top {finance.sponsor_target ?? "—"}</strong></div><div><span className="text-fm-dim">Vencimiento</span><strong className="mt-1 block">{finance.sponsor_contract_end ?? "—"}</strong></div></div></Panel>
    </div>
    <Panel className="overflow-hidden"><div className="border-b border-fm-border bg-fm-bg px-4 py-3"><div className="text-xs uppercase tracking-widest text-fm-dim">Ofertas disponibles</div><h2 className="mt-1 text-lg font-black">Elige el próximo acuerdo</h2></div>{offers.length ? <div className="divide-y divide-fm-border">{offers.map((offer) => <div key={offer.id} className="flex flex-wrap items-center justify-between gap-4 p-4"><div><div className="font-bold">{offer.sponsor_name}</div><div className="mt-1 text-sm text-fm-dim">{money(offer.weekly_amount)} / semana · bonus {money(offer.signing_bonus)} · Top {offer.target_value} · {offer.duration_weeks} semanas</div><div className="text-xs text-fm-dim">Oferta válida hasta {offer.expires_date}</div></div><button onClick={() => accept(offer)} className="rounded-lg bg-fm-accent px-4 py-2 text-sm font-black text-black hover:brightness-110">Firmar acuerdo</button></div>)}</div> : <EmptyState title="No hay ofertas nuevas" description="El departamento comercial generará nuevas propuestas próximamente." />}</Panel>
  </div>;
}
