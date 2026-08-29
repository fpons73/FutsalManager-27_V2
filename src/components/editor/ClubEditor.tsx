import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import ImagePicker from "./ImagePicker";
import PlayerEditor from "./PlayerEditor";

type Staff = { id: number; first_name: string; last_name: string; common_name: string; nation: string; nation_id: number; role: string; club_id: number | null; club_name: string | null; tactical: number; man_management: number; judging: number; motivating: number; working_youngsters: number; physio_level: number; wage_weekly: number };
type Player = { id: number; first_name: string; last_name: string; common_name: string; nation: string; nation_id: number; club: string; club_id: number | null; position: string; ca: number; pa: number; age: number; foot: string };

const ROLES = ["coach","assistant","scout","physio"];

export default function ClubEditor({ club, nations, onClose }: { club: any; nations: any[]; onClose: () => void }) {
  const [msg, setMsg] = useState<string | null>(null);
  const [coaches, setCoaches] = useState<Staff[]>([]);
  const [coachId, setCoachId] = useState<number | null>(club.coach_id ?? null);
  const [staff, setStaff] = useState<Staff[]>([]);
  const [squad, setSquad] = useState<Player[]>([]);
  const [allPlayers, setAllPlayers] = useState<Player[]>([]);
  const [search, setSearch] = useState("");
  const [editingPlayer, setEditingPlayer] = useState<Player | null>(null);
  const [finance, setFinance] = useState<any|null>(null);
  const [financeMsg, setFinanceMsg] = useState<string|null>(null);

  const refresh = async () => {
    try {
      const [c, st, sq, all] = await Promise.all([
        invoke<Staff[]>("editor_list_coaches"),
        invoke<Staff[]>("editor_list_staff", { clubId: club.id }),
        invoke<Player[]>("editor_list_players_by_club", { clubId: club.id }),
        invoke<Player[]>("editor_list_players", { limit: 2000 }),
      ]);
      setCoaches(c); setStaff(st); setSquad(sq); setAllPlayers(all);
    } catch (e) { setMsg(String(e)); }
  };
  useEffect(() => { refresh(); invoke<any>("editor_get_finance", { clubId: club.id }).then(setFinance).catch(()=>{}); }, [club.id]);

  const setCoach = async (id: number | null) => {
    try { await invoke("editor_set_coach", { clubId: club.id, staffId: id }); setCoachId(id); setMsg("Entrenador asignado"); refresh(); } catch (e){ setMsg(String(e)); }
  };

  const addStaff = async (role: string) => {
    const first = prompt("Nombre del " + role); if (!first) return;
    const last = prompt("Apellidos"); if (!last) return;
    const nid = nations[0]?.id ?? 1;
    try {
      await invoke("editor_create_staff", { first, last, nationId: nid, role, clubId: club.id, tactical: 10, manManagement: 12, judging: 12, motivating: 10, workingYoungsters: 10, physioLevel: 10, wageWeekly: 600 });
      refresh(); setMsg("Staff añadido");
    } catch (e){ setMsg(String(e)); }
  };
  const delStaff = async (id: number) => {
    if (!confirm("¿Borrar staff?")) return;
    try { await invoke("editor_delete_staff", { id }); refresh(); } catch (e){ setMsg(String(e)); }
  };

  const addPlayer = async (pid: number) => {
    try { await invoke("editor_assign_player", { playerId: pid, clubId: club.id }); refresh(); setMsg("Jugador añadido a la plantilla"); } catch (e){ setMsg(String(e)); }
  };
  const removePlayer = async (pid: number) => {
    try { await invoke("editor_release_player", { playerId: pid }); refresh(); setMsg("Jugador liberado"); } catch (e){ setMsg(String(e)); }
  };

  const freePlayers = allPlayers.filter((p) => (p.club_id ?? null) !== club.id && (!search || (p.common_name || p.first_name + " " + p.last_name).toLowerCase().includes(search.toLowerCase())));

  if (editingPlayer) {
    return (
      <PlayerEditor
        player={editingPlayer}
        nations={nations}
        onClose={() => { setEditingPlayer(null); refresh(); setMsg("Plantilla actualizada"); }}
      />
    );
  }

  return (
    <div className="rounded-xl border border-sky-500/30 bg-fm-panel p-4">
      <div className="mb-3 flex items-center justify-between">
        <h3 className="text-lg font-black">Editando {club.name} <span className="text-fm-dim">(ID {club.id})</span></h3>
        <button onClick={onClose} className="rounded-lg border border-fm-border px-3 py-1 text-sm text-fm-dim hover:text-white">Cerrar</button>
      </div>
      {msg && <div className="mb-3 rounded-lg border border-amber-500/30 bg-amber-500/10 px-3 py-2 text-sm">{msg}</div>}

      <div className="grid gap-4 lg:grid-cols-2">
        {/* Escudo */}
        <section className="rounded-lg border border-fm-border bg-fm-bg p-3">
          <div className="mb-2 text-xs font-bold uppercase tracking-widest text-fm-dim">Escudo</div>
          <ImagePicker command="editor_set_crest" entityId={club.id} label="Escudo" value={club.crest_path ?? null} prefix={club.short_name?.[0] ?? "🛡"} />
          {club.short_name && <div className="mt-2 text-xs text-fm-dim">Corto: {club.short_name}</div>}
        </section>

        {finance && <section className="mt-4 rounded-lg border border-fm-border bg-fm-bg p-3"><div className="mb-2 text-xs font-bold uppercase tracking-widest text-fm-dim">Economía</div>{financeMsg&&<div className="mb-2 text-xs text-fm-accent">{financeMsg}</div>}<div className="grid gap-2 sm:grid-cols-3"><label className="text-xs text-fm-dim">Balance<input type="number" value={finance.balance} onChange={e=>setFinance({...finance,balance:Number(e.target.value)})} className="mt-1 w-full rounded border border-fm-border bg-fm-panel px-2 py-1"/></label><label className="text-xs text-fm-dim">Presupuesto fichajes<input type="number" value={finance.transfer_budget} onChange={e=>setFinance({...finance,transfer_budget:Number(e.target.value)})} className="mt-1 w-full rounded border border-fm-border bg-fm-panel px-2 py-1"/></label><label className="text-xs text-fm-dim">Presupuesto salarial<input type="number" value={finance.wage_budget} onChange={e=>setFinance({...finance,wage_budget:Number(e.target.value)})} className="mt-1 w-full rounded border border-fm-border bg-fm-panel px-2 py-1"/></label><label className="text-xs text-fm-dim">Patrocinio acumulado<input type="number" value={finance.sponsorship} onChange={e=>setFinance({...finance,sponsorship:Number(e.target.value)})} className="mt-1 w-full rounded border border-fm-border bg-fm-panel px-2 py-1"/></label><label className="text-xs text-fm-dim">Taquilla acumulada<input type="number" value={finance.ticket_income} onChange={e=>setFinance({...finance,ticket_income:Number(e.target.value)})} className="mt-1 w-full rounded border border-fm-border bg-fm-panel px-2 py-1"/></label><label className="text-xs text-fm-dim">Premios acumulados<input type="number" value={finance.prize_money} onChange={e=>setFinance({...finance,prize_money:Number(e.target.value)})} className="mt-1 w-full rounded border border-fm-panel bg-fm-panel px-2 py-1"/></label></div><button onClick={async()=>{try{await invoke("editor_update_finance",{clubId:club.id,balance:finance.balance,transferBudget:finance.transfer_budget,wageBudget:finance.wage_budget,sponsorship:finance.sponsorship,ticketIncome:finance.ticket_income,prizeMoney:finance.prize_money});setFinanceMsg("Economía guardada");}catch(e){setFinanceMsg(String(e));}}} className="mt-3 rounded bg-fm-accent px-3 py-1.5 text-sm font-bold text-black">Guardar economía</button></section>}

      {/* Entrenador */}
        <section className="rounded-lg border border-fm-border bg-fm-bg p-3">
          <div className="mb-2 text-xs font-bold uppercase tracking-widest text-fm-dim">Entrenador</div>
          <div className="flex flex-wrap items-center gap-2">
            <select value={coachId ?? ""} onChange={(e)=>setCoach(e.target.value ? Number(e.target.value) : null)} className="flex-1 rounded border border-fm-border bg-fm-panel px-2 py-1.5 text-sm">
              <option value="">— Sin entrenador —</option>
              {coaches.map((c)=> <option key={c.id} value={c.id}>{c.common_name} {c.club_name ? `(${c.club_name})` : "(libre)"}</option>)}
            </select>
            <button onClick={() => addStaff("coach")} className="rounded bg-fm-accent px-3 py-1.5 text-sm font-bold text-black">Nuevo</button>
          </div>
          {coaches.find((c)=>c.id===coachId) && (
            <div className="mt-2 text-xs text-fm-dim">
              Táctica {coaches.find((c)=>c.id===coachId)!.tactical} · Gestión {coaches.find((c)=>c.id===coachId)!.man_management} · Motivación {coaches.find((c)=>c.id===coachId)!.motivating}
            </div>
          )}
        </section>
      </div>

      {/* Staff */}
      <section className="mt-4 rounded-lg border border-fm-border bg-fm-bg p-3">
        <div className="mb-2 flex items-center justify-between">
          <div className="text-xs font-bold uppercase tracking-widest text-fm-dim">Cuerpo técnico ({staff.length}) · preparadores incluidos</div>
          <div className="flex gap-1">
            {ROLES.map((r)=> (
              <button key={r} onClick={()=>addStaff(r)} className="rounded bg-fm-panel2 px-2 py-1 text-xs font-semibold text-fm-dim hover:text-white">+ {r}</button>
            ))}
          </div>
        </div>
        {staff.length===0 ? <div className="text-sm text-fm-dim">Sin staff. Usa los botones para añadir.</div> : (
          <div className="grid gap-1 sm:grid-cols-2">
            {staff.map((s)=>(
              <div key={s.id} className="flex items-center justify-between rounded bg-fm-panel px-2 py-1.5 text-sm">
                <span><b>{s.common_name}</b> <span className="rounded bg-fm-bg px-1.5 py-0.5 text-xs font-bold uppercase">{s.role}</span></span>
                <button onClick={()=>delStaff(s.id)} className="rounded bg-red-600 px-2 py-0.5 text-xs text-white">Borrar</button>
              </div>
            ))}
          </div>
        )}
      </section>

      {/* Plantilla */}
      <section className="mt-4 rounded-lg border border-fm-border bg-fm-bg p-3">
        <div className="mb-2 text-xs font-bold uppercase tracking-widest text-fm-dim">Plantilla ({squad.length} jugadores)</div>
        <div className="mb-2 flex gap-2">
          <input placeholder="Buscar jugador libre para fichar…" value={search} onChange={(e)=>setSearch(e.target.value)} className="flex-1 rounded border border-fm-border bg-fm-panel px-2 py-1.5 text-sm" />
        </div>
        {search && freePlayers.length>0 && (
          <div className="mb-2 max-h-40 overflow-auto rounded border border-fm-border bg-fm-panel">
            {freePlayers.slice(0, 20).map((p)=>(
              <div key={p.id} className="flex items-center justify-between border-b border-fm-border px-2 py-1 text-sm hover:bg-fm-panel2">
                <span>{p.common_name} <span className="text-fm-dim">({p.position}, CA {p.ca}, {p.club || "libre"})</span></span>
                <button onClick={()=>addPlayer(p.id)} className="rounded bg-fm-accent px-2 py-0.5 text-xs font-bold text-black">Fichar</button>
              </div>
            ))}
          </div>
        )}
        {squad.length===0 ? <div className="text-sm text-fm-dim">Sin jugadores. Busca arriba para fichar.</div> : (
          <div className="grid gap-1 sm:grid-cols-2">
            {squad.map((p)=>(
              <div key={p.id} className="flex items-center justify-between rounded bg-fm-panel px-2 py-1.5 text-sm">
                <button onClick={()=>setEditingPlayer(p)} className="text-left hover:text-fm-accent">
                  {p.common_name} <span className="rounded bg-fm-bg px-1.5 py-0.5 text-xs font-bold">{p.position}</span> <span className="font-mono text-xs text-fm-dim">CA {p.ca}</span>
                </button>
                <button onClick={()=>removePlayer(p.id)} className="rounded bg-red-600 px-2 py-0.5 text-xs text-white">Quitar</button>
              </div>
            ))}
          </div>
        )}
      </section>
    </div>
  );
}
