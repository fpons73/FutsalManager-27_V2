import { useEffect, useState } from "react";
import { api, type ContractRow, type PlayerRow } from "../../api";
import { useStore } from "../../store";
import { EmptyState, MetricCard, Panel, StatusBadge } from "../ui";

export default function SquadView() {
  const { userClubId, clubs } = useStore();
  const [players, setPlayers] = useState<PlayerRow[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [contracts, setContracts] = useState<ContractRow[]>([]);
  const [selected, setSelected] = useState<ContractRow | null>(null);
  const [message, setMessage] = useState("");
  const [years, setYears] = useState(3);
  const [wage, setWage] = useState(0);
  const [role, setRole] = useState("rotation");
  const [clause, setClause] = useState(0);
  const [signingBonus, setSigningBonus] = useState(0);
  const [appearanceBonus, setAppearanceBonus] = useState(0);
  const [cleanSheetBonus, setCleanSheetBonus] = useState(0);

  useEffect(() => {
    if (!userClubId) { setLoading(false); setError("No hay club seleccionado"); return; }
    setLoading(true);
    setError(null);
    api.getContracts(userClubId).then(setContracts).catch(()=>{});
    api.getSquad(userClubId).then((p)=>{
      setPlayers(p);
      if (p.length===0) setError(`Club ${userClubId} (${clubs.find(c=>c.id===userClubId)?.name ?? "?"}) sin jugadores — verifica DB`);
    }).catch((e)=> setError(String(e))).finally(()=>setLoading(false));
  }, [userClubId]);

  if (loading) return <div className="mx-auto max-w-6xl p-6"><Panel className="p-8 text-center text-fm-dim">Cargando plantilla…</Panel></div>;
  if (error) return <div className="mx-auto max-w-6xl p-6"><Panel className="p-8 text-center"><div className="text-amber-400">{error}</div><button onClick={()=> userClubId && api.getSquad(userClubId).then(setPlayers)} className="mt-2 rounded bg-fm-accent px-3 py-1 text-sm font-bold text-black">Reintentar</button></Panel></div>;
  if (!players.length) return <div className="mx-auto max-w-6xl p-6"><EmptyState title="Plantilla vacía" description={`Club ${userClubId ?? "—"} sin jugadores disponibles.`} /></div>;

  const selectContract = (contract: ContractRow) => {
    setSelected(contract); setYears(3); setWage(Math.round(contract.wage * 1.12)); setRole(contract.role); setClause(contract.release_clause ?? 0); setSigningBonus(contract.signing_bonus); setAppearanceBonus(contract.appearance_bonus); setCleanSheetBonus(contract.clean_sheet_bonus);
  };
  const renew = async () => {
    if (!userClubId || !selected) return;
    try {
      const result = await api.renewContract({ clubId:userClubId, playerId:selected.player_id, years, wage, releaseClause:clause > 0 ? clause : null, role, signingBonus, appearanceBonus, cleanSheetBonus });
      setMessage(result);
      setContracts(await api.getContracts(userClubId));
    } catch (e) { setMessage(String(e)); }
  };

  return (
    <div className="mx-auto max-w-6xl space-y-4 p-4 lg:p-6">
      <div className="flex items-end justify-between"><div><div className="text-xs font-bold uppercase tracking-[0.2em] text-fm-accent">Gestión del club</div><h2 className="mb-4 text-xl font-black">Plantilla <span className="font-normal text-fm-dim">({players.length} jugadores)</span></h2></div><StatusBadge tone="success">{players.length} jugadores activos</StatusBadge></div>
      <div className="grid gap-3 sm:grid-cols-3"><MetricCard label="Jugadores" value={players.length} detail="Plantilla registrada" /><MetricCard label="CA medio" value={Math.round(players.reduce((s,p)=>s+p.ca,0)/players.length)} detail="Nivel actual" tone="text-fm-sky-300" /><MetricCard label="Alertas contrato" value={contracts.filter(c => c.end_date <= new Date(Date.now()+1000*60*60*24*365).toISOString().slice(0,10)).length} detail="Vencen en 12 meses" tone="text-fm-amber-300" /></div>
      <Panel className="p-4"><section className="mb-5 rounded-xl border border-fm-border bg-fm-panel p-4">
        <div className="mb-3 flex items-center justify-between"><h3 className="font-bold">Contratos</h3><span className="text-xs text-fm-dim">{contracts.filter(c => c.end_date <= new Date(Date.now()+1000*60*60*24*365).toISOString().slice(0,10)).length} vencen en 12 meses</span></div>
        {message && <div className="mb-3 rounded bg-fm-accent/10 px-3 py-2 text-sm text-fm-accent">{message}</div>}
        <div className="space-y-2">
          {contracts.map(c => <button key={c.player_id} onClick={() => selectContract(c)} className={`flex w-full items-center justify-between rounded-lg border px-3 py-2 text-left ${selected?.player_id===c.player_id ? "border-fm-accent bg-fm-accent/10" : "border-fm-border hover:bg-fm-panel2"}`}><span><b>{c.player_name}</b> <span className="text-xs text-fm-dim">{c.position} · {c.role}</span></span><span className="text-right text-xs"><b>€{Math.round(c.wage).toLocaleString()}</b>/sem<br/><span className="text-fm-dim">hasta {c.end_date}</span></span></button>)}
        </div>
        {selected && <div className="mt-3 rounded-lg bg-fm-bg p-3 text-sm"><div className="mb-3 font-semibold">Propuesta para {selected.player_name}</div><div className="grid gap-3 sm:grid-cols-2 lg:grid-cols-4"><label className="text-xs text-fm-dim">Años<select value={years} onChange={e=>setYears(Number(e.target.value))} className="mt-1 w-full rounded border border-fm-border bg-fm-panel px-2 py-1 text-white"><option value={1}>1</option><option value={2}>2</option><option value={3}>3</option><option value={4}>4</option><option value={5}>5</option></select></label><label className="text-xs text-fm-dim">Salario semanal<input type="number" min={1} value={wage} onChange={e=>setWage(Number(e.target.value))} className="mt-1 w-full rounded border border-fm-border bg-fm-panel px-2 py-1 text-white" /></label><label className="text-xs text-fm-dim">Rol<select value={role} onChange={e=>setRole(e.target.value)} className="mt-1 w-full rounded border border-fm-border bg-fm-panel px-2 py-1 text-white"><option value="star">Estrella</option><option value="first_team">Titular</option><option value="rotation">Rotación</option><option value="backup">Suplente</option></select></label><label className="text-xs text-fm-dim">Cláusula<input type="number" min={0} value={clause} onChange={e=>setClause(Number(e.target.value))} className="mt-1 w-full rounded border border-fm-border bg-fm-panel px-2 py-1 text-white" /></label><label className="text-xs text-fm-dim">Prima fichaje<input type="number" min={0} value={signingBonus} onChange={e=>setSigningBonus(Number(e.target.value))} className="mt-1 w-full rounded border border-fm-border bg-fm-panel px-2 py-1 text-white" /></label><label className="text-xs text-fm-dim">Bonus aparición<input type="number" min={0} value={appearanceBonus} onChange={e=>setAppearanceBonus(Number(e.target.value))} className="mt-1 w-full rounded border border-fm-border bg-fm-panel px-2 py-1 text-white" /></label><label className="text-xs text-fm-dim">Bonus portería<input type="number" min={0} value={cleanSheetBonus} onChange={e=>setCleanSheetBonus(Number(e.target.value))} className="mt-1 w-full rounded border border-fm-border bg-fm-panel px-2 py-1 text-white" /></label></div><div className="mt-3 flex flex-wrap items-center justify-between gap-2"><span className="text-xs text-fm-dim">Coste semanal: €{Math.round(wage).toLocaleString()} · Variación: {Math.round(wage-selected.wage)>=0?'+':''}€{Math.round(wage-selected.wage).toLocaleString()}</span><button onClick={renew} disabled={wage<=0} className="rounded bg-fm-accent px-3 py-1.5 font-bold text-black disabled:cursor-not-allowed disabled:opacity-50">Enviar renovación</button></div></div>}
      </section></Panel>
      <Panel className="overflow-hidden">
        <div className="overflow-x-auto">
          <table className="w-full text-sm">
            <thead className="bg-fm-bg text-xs uppercase tracking-widest text-fm-dim">
              <tr>
                <th className="px-3 py-2 text-left">Jugador</th>
                <th className="px-2 py-2">Pos</th>
                <th className="px-2 py-2">Edad</th>
                <th className="px-2 py-2">Nac</th>
                <th className="px-2 py-2">CA</th>
                <th className="px-2 py-2">PA</th>
                <th className="px-2 py-2">Cond</th>
                <th className="px-2 py-2">Salario</th>
                <th className="px-2 py-2">PAS</th>
                <th className="px-2 py-2">FIN</th>
                <th className="px-2 py-2">REG</th>
                <th className="px-2 py-2">ENT</th>
                <th className="px-2 py-2">RIT</th>
              </tr>
            </thead>
            <tbody>
              {players.map((p) => (
                <tr key={p.id} className="border-t border-fm-border hover:bg-fm-panel2">
                  <td className="px-3 py-2 font-semibold"><span className="mr-2 inline-flex items-center gap-1"><img src={p.flag_path ?? undefined} alt={p.nation} title={p.nation} className="h-3 w-5 rounded object-cover" onError={(e)=>{e.currentTarget.style.display="none"}} />{p.second_flag_path && <img src={p.second_flag_path} alt="Segunda nacionalidad" title="Segunda nacionalidad" className="h-3 w-5 rounded object-cover" />}</span>{p.common_name} <span className="font-normal text-fm-dim">({p.first_name} {p.last_name})</span></td>
                  <td className="px-2 py-2 text-center"><span className={`rounded px-1.5 py-0.5 text-xs font-bold ${p.position==="POR" ? "bg-amber-500/20 text-amber-400" : p.position==="PIV" ? "bg-red-500/20 text-red-400" : p.position==="CIE" ? "bg-sky-500/20 text-sky-400" : "bg-emerald-500/20 text-emerald-400"}`}>{p.position}</span></td>
                  <td className="px-2 py-2 text-center">{p.age}</td>
                  <td className="px-2 py-2 text-center text-xs"><span className="inline-flex items-center gap-1"><img src={p.flag_path ?? undefined} alt={p.nation} title={p.nation} className="h-3 w-5 rounded object-cover" onError={(e)=>{e.currentTarget.style.display="none"}} />{p.nation}{p.second_flag_path && <img src={p.second_flag_path} alt="Segunda nacionalidad" title="Segunda nacionalidad" className="h-3 w-5 rounded object-cover" />}</span></td>
                  <td className="px-2 py-2 text-center font-mono font-bold">{p.ca}</td>
                  <td className="px-2 py-2 text-center font-mono text-fm-dim">{p.pa}</td>
                  <td className="px-2 py-2 text-center"><span className={`rounded px-1.5 py-0.5 text-xs ${p.condition>80 ? "bg-emerald-500/20 text-emerald-400" : p.condition>60 ? "bg-amber-500/20 text-amber-400" : "bg-red-500/20 text-red-400"}`}>{p.condition}%</span></td>
                  <td className="px-2 py-2 text-right font-mono text-xs">€{Math.round(p.wage).toLocaleString()}</td>
                  <td className="px-2 py-2 text-center font-mono">{p.attrs.passing}</td>
                  <td className="px-2 py-2 text-center font-mono">{p.attrs.finishing}</td>
                  <td className="px-2 py-2 text-center font-mono">{p.attrs.dribbling}</td>
                  <td className="px-2 py-2 text-center font-mono">{p.attrs.tackling}</td>
                  <td className="px-2 py-2 text-center font-mono">{p.attrs.reflexes}</td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      </Panel>
    </div>
  );
}
