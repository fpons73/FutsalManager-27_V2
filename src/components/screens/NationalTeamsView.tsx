import { useEffect, useState } from "react";
import { api, type NationalPlayerRow } from "../../api";
import { useStore } from "../../store";
import Flag from "../Flag";

export default function NationalTeamsView() {
  const { competitions } = useStore();
  const nations = competitions.filter(c=>c.kind === "national_team");
  const [nationId, setNationId] = useState<number>(1);
  const [players, setPlayers] = useState<NationalPlayerRow[]>([]);
  const load = () => api.getNationalPlayers(nationId).then(setPlayers).catch(()=>setPlayers([]));
  useEffect(()=>{ if(nations.length && !nations.some(n=>n.id===nationId)) setNationId(nations[0].id); },[nations,nationId]);
  useEffect(()=>{load();},[nationId]);
  return <div className="mx-auto max-w-6xl space-y-4 p-4 lg:p-6"><div><div className="text-xs uppercase tracking-widest text-fm-accent">Fútbol internacional</div><h2 className="text-2xl font-black">Selecciones nacionales</h2><p className="text-sm text-fm-dim">Consulta jugadores elegibles por nacionalidad y prepara convocatorias.</p></div><div className="flex flex-wrap gap-2"><select value={nationId} onChange={e=>setNationId(Number(e.target.value))} className="rounded-lg border border-fm-border bg-fm-panel px-3 py-2 text-sm">{nations.length ? nations.map(n=><option key={n.id} value={n.id}>{n.name}</option>) : <option value={nationId}>Selección</option>}</select><span className="rounded-lg bg-fm-panel2 px-3 py-2 text-sm text-fm-dim">{players.filter(p=>p.selected).length}/14 convocados</span></div><div className="overflow-hidden rounded-xl border border-fm-border bg-fm-panel"><table className="w-full text-sm"><thead className="bg-fm-bg text-left text-xs uppercase tracking-widest text-fm-dim"><tr><th className="px-4 py-3">Jugador</th><th>Pos</th><th>Club</th><th>Elegibilidad</th><th/></tr></thead><tbody>{players.map(p=><tr key={p.player_id} className="border-t border-fm-border"><td className="px-4 py-3 font-semibold"><span className="mr-2 inline-flex"><Flag src={p.flag_path} alt={p.nation_name} /></span>{p.player_name}</td><td className="text-center">{p.position}</td><td className="text-fm-dim">{p.club_name ?? "Libre"}</td><td className="text-center text-xs text-emerald-300">{p.callup_status}</td><td className="px-4 py-2 text-right"><button onClick={async()=>{await api.setNationalCallup(nationId,p.player_id,!Boolean(p.selected));load();}} className={`rounded px-3 py-1 text-xs font-bold ${p.selected ? "border border-fm-border text-fm-dim" : "bg-fm-accent text-black"}`}>{p.selected ? "Descartar" : "Convocar"}</button></td></tr>)}</tbody></table>{!players.length&&<div className="p-8 text-center text-fm-dim">No hay jugadores elegibles para esta selección.</div>}</div></div>;
}
