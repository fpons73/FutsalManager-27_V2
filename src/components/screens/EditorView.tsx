import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import ClubEditor from "../editor/ClubEditor";
import PlayerEditor from "../editor/PlayerEditor";
import StaffEditor from "../editor/StaffEditor";
import NationEditor from "../editor/NationEditor";
import { EmptyState, Icon, MetricCard, Panel, StatusBadge } from "../ui";

type Tab = "clubs" | "players" | "staff" | "contracts" | "stadiums" | "nations" | "competitions";

const emptyClub = { id: 0, name: "", short: "", nation: 1, city: "", stadium: "", cap: 2000, rep: 600, c1: "#0f4c3a", c2: "#ffffff" };
const emptyPlayer = { id: 0, first: "", last: "", nation: 1, secondNation: "", secondaryPos: "", club: "", ca: 80, pa: 120, pos: "ALA" };
const emptyStaff = { id: 0, first: "", last: "", nation: 1, role: "assistant", club: "", tactical: 10, manManagement: 12, judging: 12, motivating: 10, workingYoungsters: 10, physioLevel: 10, wage: 600 };
const emptyNation = { id: 0, name: "", conf: 1, rep: 500, level: 50 };
const emptyComp = { id: 0, name: "", nation: "", tier: "", teams: 16, season: "2026/2027" };

export default function EditorView() {
  const [tab, setTab] = useState<Tab>("clubs");
  const [data, setData] = useState<any[]>([]);
  const [search, setSearch] = useState("");
  const [loading, setLoading] = useState(false);
  const [msg, setMsg] = useState<string | null>(null);

  const [newClub, setNewClub] = useState(emptyClub);
  const [newPlayer, setNewPlayer] = useState(emptyPlayer);
  const [newStaff, setNewStaff] = useState(emptyStaff);
  const [newNation, setNewNation] = useState(emptyNation);
  const [newComp, setNewComp] = useState(emptyComp);

  const [editingClub, setEditingClub] = useState<any | null>(null);
  const [editingPlayer, setEditingPlayer] = useState<any | null>(null);
  const [editingStaff, setEditingStaff] = useState<any | null>(null);
  const [editingNation, setEditingNation] = useState<any | null>(null);

  const [confeds, setConfeds] = useState<any[]>([]);
  const [nations, setNations] = useState<any[]>([]);
  const [clubs, setClubs] = useState<any[]>([]);
  const [, setStadiums] = useState<any[]>([]);

  const load = async (t: Tab) => {
    setLoading(true);
    try {
      if (t === "nations") setData(await invoke<any[]>("editor_list_nations"));
      else if (t === "clubs") setData(await invoke<any[]>("editor_list_clubs"));
      else if (t === "players") setData(await invoke<any[]>("editor_list_players", { limit: 2000 }));
      else if (t === "staff") setData(await invoke<any[]>("editor_list_staff"));
      else if (t === "contracts") setData(await invoke<any[]>("editor_list_contracts"));
      else if (t === "stadiums") setData(await invoke<any[]>("editor_list_stadiums"));
      else if (t === "competitions") setData(await invoke<any[]>("editor_list_competitions"));
      setMsg(null);
    } catch (e) { setMsg(String(e)); }
    finally { setLoading(false); }
  };

  useEffect(() => { load(tab); }, [tab]);
  useEffect(() => {
    invoke<number>("editor_init")
      .then(async () => {
        const [c, n, cl, st] = await Promise.all([
          invoke<any[]>("editor_list_confederations_full").catch(()=>[]),
          invoke<any[]>("editor_list_nations").catch(()=>[]),
          invoke<any[]>("editor_list_clubs").catch(()=>[]),
          invoke<any[]>("editor_list_stadiums").catch(()=>[]),
        ]);
        setConfeds(c); setNations(n); setClubs(cl); setStadiums(st);
        load(tab);
      })
      .catch(()=>{});
  }, [tab]);

  const filtered = data.filter((r:any) => {
    if (!search) return true;
    const s = search.toLowerCase();
    return JSON.stringify(Object.values(r)).toLowerCase().includes(s);
  });
  const setSearchAndClear = (q:string) => setSearch(q);

  const inEdit = (state:any) => state.id !== 0;

  // ---- Clubs ----
  const saveClub = async () => {
    try {
      if (newClub.id) await invoke("editor_update_club", { id: newClub.id, name: newClub.name, shortName: newClub.short, nationId: newClub.nation, city: newClub.city, stadium: newClub.stadium, capacity: newClub.cap, reputation: newClub.rep, c1: newClub.c1, c2: newClub.c2 });
      else await invoke("editor_create_club", { name: newClub.name, shortName: newClub.short, nationId: newClub.nation, city: newClub.city, stadium: newClub.stadium, capacity: newClub.cap, reputation: newClub.rep, c1: newClub.c1, c2: newClub.c2 });
      setMsg(newClub.id ? "Club actualizado" : "Club creado");
      setNewClub(emptyClub); load("clubs");
    }catch(e){ setMsg(String(e)); }
  };
  const delClub = async (id:number) => { if(!confirm("¿Borrar club?")) return; try{ await invoke("editor_delete_club",{id}); load("clubs"); }catch(e){ setMsg(String(e)); } };

  // ---- Players ----
  const savePlayer = async () => {
    try {
      if (newPlayer.id) await invoke("editor_update_player", { id: newPlayer.id, first: newPlayer.first, last: newPlayer.last, nationId: newPlayer.nation, secondNationId: newPlayer.secondNation ? Number(newPlayer.secondNation) : null, secondaryPosition: newPlayer.secondaryPos || null, clubId: newPlayer.club ? Number(newPlayer.club) : null, ca: newPlayer.ca, pa: newPlayer.pa, pos: newPlayer.pos });
      else await invoke("editor_create_player", { first: newPlayer.first, last: newPlayer.last, nationId: newPlayer.nation, clubId: newPlayer.club ? Number(newPlayer.club) : null, ca: newPlayer.ca, pa: newPlayer.pa, pos: newPlayer.pos });
      setMsg(newPlayer.id ? "Jugador actualizado" : "Jugador creado");
      setNewPlayer(emptyPlayer); load("players");
    }catch(e){ setMsg(String(e)); }
  };
  const delPlayer = async (id:number) => { if(!confirm("¿Borrar jugador?")) return; try{ await invoke("editor_delete_player",{id}); load("players"); }catch(e){ setMsg(String(e)); } };

  // ---- Staff ----
  const saveStaff = async () => {
    try {
      if (newStaff.id) await invoke("editor_update_staff", { id: newStaff.id, first: newStaff.first, last: newStaff.last, nationId: newStaff.nation, role: newStaff.role, clubId: newStaff.club ? Number(newStaff.club) : null, tactical: newStaff.tactical, manManagement: newStaff.manManagement, judging: newStaff.judging, motivating: newStaff.motivating, workingYoungsters: newStaff.workingYoungsters, physioLevel: newStaff.physioLevel, wageWeekly: newStaff.wage });
      else await invoke("editor_create_staff", { first: newStaff.first, last: newStaff.last, nationId: newStaff.nation, role: newStaff.role, clubId: newStaff.club ? Number(newStaff.club) : null, tactical: newStaff.tactical, manManagement: newStaff.manManagement, judging: newStaff.judging, motivating: newStaff.motivating, workingYoungsters: newStaff.workingYoungsters, physioLevel: newStaff.physioLevel, wageWeekly: newStaff.wage });
      setMsg(newStaff.id ? "Staff actualizado" : "Staff creado");
      setNewStaff(emptyStaff); load("staff");
    }catch(e){ setMsg(String(e)); }
  };
  const delStaff = async (id:number) => { if(!confirm("¿Borrar staff?")) return; try{ await invoke("editor_delete_staff",{id}); load("staff"); }catch(e){ setMsg(String(e)); } };

  // ---- Nations ----
  const saveNation = async () => {
    try {
      if (newNation.id) await invoke("editor_update_nation", { id: newNation.id, name: newNation.name, confederationId: newNation.conf, reputation: newNation.rep, futsalLevel: newNation.level });
      else await invoke("editor_create_nation", { name: newNation.name, confederationId: newNation.conf, reputation: newNation.rep, futsalLevel: newNation.level });
      setMsg(newNation.id ? "Nación actualizada" : "Nación creada");
      setNewNation(emptyNation); load("nations");
    }catch(e){ setMsg(String(e)); }
  };
  const delNation = async (id:number) => { if(!confirm("¿Borrar nación?")) return; try{ await invoke("editor_delete_nation",{id}); load("nations"); }catch(e){ setMsg(String(e)); } };

  // ---- Competitions ----
  const saveComp = async () => {
    try {
      if (newComp.id) await invoke("editor_update_competition", { id: newComp.id, name: newComp.name, nationId: newComp.nation ? Number(newComp.nation) : null, tier: newComp.tier ? Number(newComp.tier) : null, totalTeams: newComp.teams, season: newComp.season });
      else await invoke("editor_create_competition", { name: newComp.name, nationId: newComp.nation ? Number(newComp.nation) : null, tier: newComp.tier ? Number(newComp.tier) : null, totalTeams: newComp.teams, season: newComp.season });
      setMsg(newComp.id ? "Competición actualizada" : "Competición creada");
      setNewComp(emptyComp); load("competitions");
    }catch(e){ setMsg(String(e)); }
  };
  const delComp = async (id:number) => { if(!confirm("¿Borrar competición?")) return; try{ await invoke("editor_delete_competition",{id}); load("competitions"); }catch(e){ setMsg(String(e)); } };

  const tabMeta: Record<Tab, { label: string; icon: "home" | "users" | "calendar" | "trophy" | "market" | "search" | "finance" | "menu" }> = {
    clubs: { label: "Clubes", icon: "home" }, players: { label: "Jugadores", icon: "users" }, staff: { label: "Staff", icon: "users" }, contracts: { label: "Contratos", icon: "calendar" }, stadiums: { label: "Pabellones", icon: "home" }, nations: { label: "Países", icon: "home" }, competitions: { label: "Competiciones", icon: "trophy" },
  };

  return (
    <div className="mx-auto max-w-7xl space-y-5 p-4 sm:p-6">
      <header className="flex flex-wrap items-end justify-between gap-3">
        <div><p className="text-xs font-bold uppercase tracking-[0.22em] text-fm-accent">Centro de configuración</p><h2 className="mt-1 text-2xl font-black tracking-tight">Editor de base de datos</h2><p className="mt-1 text-sm text-fm-dim">Prepara el universo antes de iniciar una partida.</p></div>
        <StatusBadge tone="success">Modo prepartida</StatusBadge>
      </header>
      <div className="grid gap-3 sm:grid-cols-3"><MetricCard label="Registros visibles" value={data.length} detail={tabMeta[tab].label} /><MetricCard label="Coincidencias" value={filtered.length} detail={search ? "Filtro activo" : "Sin filtro"} tone="text-sky-300" /><MetricCard label="Módulo activo" value={<span className="flex items-center gap-2"><Icon name={tabMeta[tab].icon} />{tabMeta[tab].label}</span>} tone="text-fm-accent" /></div>

      <Panel className="overflow-hidden"><div className="flex gap-1 overflow-x-auto p-2" role="tablist" aria-label="Módulos del editor">
        {(["clubs","players","staff","contracts","stadiums","nations","competitions"] as Tab[]).map((t)=>(
          <button key={t} role="tab" aria-selected={tab===t} onClick={()=>{ setTab(t); setSearch(""); }} className={`flex shrink-0 items-center gap-2 rounded-xl px-3 py-2 text-sm font-semibold transition-colors ${tab===t ? "bg-fm-accent text-black" : "text-fm-dim hover:bg-fm-panel2 hover:text-white"}`}><Icon name={tabMeta[t].icon} />{tabMeta[t].label}</button>
        ))}
      </div></Panel>

      {/* Buscador global de la pestaña */}
      <Panel className="flex items-center gap-2 px-3 py-2">
        <Icon name="search" label="Buscar" />
        <input aria-label={`Buscar en ${tabMeta[tab].label}`} value={search} onChange={(e)=>setSearchAndClear(e.target.value)} placeholder={`Buscar en ${tabMeta[tab].label}…`} className="w-full bg-transparent text-sm outline-none" />
        {filtered.length !== data.length && <span className="shrink-0 text-xs text-fm-dim">{filtered.length}/{data.length}</span>}
      </Panel>

      {msg && <div role="status" className="rounded-lg border border-amber-500/30 bg-amber-500/10 px-3 py-2 text-sm">{msg}</div>}

      {loading ? <Panel className="p-8 text-center text-fm-dim" aria-live="polite">Cargando {tabMeta[tab].label.toLowerCase()}…</Panel> : (
        <>
          {tab==="clubs" && (
            <div className="space-y-4">
              {editingClub && <ClubEditor club={editingClub} nations={nations} onClose={()=>{ setEditingClub(null); setNewClub(emptyClub); load("clubs"); }} />}
              <div className="rounded-xl border border-fm-border bg-fm-panel p-3">
                <div className="mb-2 text-xs font-bold uppercase tracking-widest text-fm-dim">{inEdit(newClub) ? "Datos del club · ID " + newClub.id : "Nuevo club"}</div>
                <div className="grid gap-2 sm:grid-cols-2 lg:grid-cols-4">
                  <input placeholder="Nombre" value={newClub.name} onChange={(e)=>setNewClub({...newClub,name:e.target.value})} className="rounded border border-fm-border bg-fm-bg px-2 py-1.5 text-sm" />
                  <input placeholder="Corto (3-4)" value={newClub.short} onChange={(e)=>setNewClub({...newClub,short:e.target.value})} className="rounded border border-fm-border bg-fm-bg px-2 py-1.5 text-sm" />
                  <select value={newClub.nation} onChange={(e)=>setNewClub({...newClub,nation:Number(e.target.value)})} className="rounded border border-fm-border bg-fm-bg px-2 py-1.5 text-sm">
                    {nations.map((n:any)=><option key={n.id} value={n.id}>{n.name}</option>)}
                  </select>
                  <input placeholder="Ciudad" value={newClub.city} onChange={(e)=>setNewClub({...newClub,city:e.target.value})} className="rounded border border-fm-border bg-fm-bg px-2 py-1.5 text-sm" />
                  <input placeholder="Pabellón" value={newClub.stadium} onChange={(e)=>setNewClub({...newClub,stadium:e.target.value})} className="rounded border border-fm-border bg-fm-bg px-2 py-1.5 text-sm" />
                  <input type="number" placeholder="Capacidad" value={newClub.cap} onChange={(e)=>setNewClub({...newClub,cap:Number(e.target.value)})} className="rounded border border-fm-border bg-fm-bg px-2 py-1.5 text-sm" />
                  <input type="number" placeholder="Rep" value={newClub.rep} onChange={(e)=>setNewClub({...newClub,rep:Number(e.target.value)})} className="rounded border border-fm-border bg-fm-bg px-2 py-1.5 text-sm" />
                  <button onClick={saveClub} className="rounded bg-fm-accent px-3 py-1.5 text-sm font-bold text-black">{inEdit(newClub) ? "Guardar" : "Crear club (+12 jugadores)"}</button>
                </div>
              </div>
              <div className="overflow-hidden rounded-xl border border-fm-border bg-fm-panel">
                <div className="max-h-[26rem] overflow-auto">
                  <table className="w-full text-sm">
                    <thead className="sticky top-0 bg-fm-bg text-xs uppercase tracking-widest text-fm-dim"><tr><th className="px-2 py-2 text-left">Club</th><th className="px-2 py-2">Nación</th><th className="px-2 py-2">Pabellón</th><th className="px-2 py-2">Rep</th><th className="px-2 py-2">Plant.</th><th></th></tr></thead>
                    <tbody>{filtered.map((c:any)=><tr key={c.id} className="border-t border-fm-border hover:bg-fm-panel2">
                      <td className="px-2 py-1.5"><span className="inline-flex items-center gap-2"><span className="flex h-7 w-7 items-center justify-center rounded bg-fm-bg text-xs font-bold text-white" style={{background:c.primary_color}}>{c.short_name}</span><span className="font-semibold">{c.name}</span></span></td>
                      <td className="px-2 py-1.5 text-xs">{c.nation}</td><td className="px-2 py-1.5 text-xs">{c.stadium}</td><td className="px-2 py-1.5 text-center font-mono">{c.reputation}</td><td className="px-2 py-1.5 text-center font-mono">{c.squad_count}</td>
                      <td className="px-2 py-1.5 text-right space-x-1"><button onClick={()=>{ setNewClub({ id: c.id, name: c.name, short: c.short_name, nation: c.nation_id, city: c.city ?? "", stadium: c.stadium ?? "", cap: c.capacity ?? 2000, rep: c.reputation, c1: c.primary_color ?? "#0f4c3a", c2: c.secondary_color ?? "#ffffff" }); setEditingClub(c); }} className="rounded bg-sky-600 px-2 py-0.5 text-xs text-white">Editar</button><button onClick={()=>delClub(c.id)} className="rounded bg-red-600 px-2 py-0.5 text-xs text-white">Borrar</button></td>
                    </tr>)}</tbody>
                  </table>
                </div>
              </div>
            </div>
          )}

          {tab==="players" && (
            <div className="space-y-4">
              {editingPlayer && <PlayerEditor player={editingPlayer} nations={nations} onClose={()=>{ setEditingPlayer(null); load("players"); }} />}
              <div className="rounded-xl border border-fm-border bg-fm-panel p-3">
                <div className="mb-2 text-xs font-bold uppercase tracking-widest text-fm-dim">{inEdit(newPlayer) ? "Editar jugador · ID " + newPlayer.id : "Nuevo jugador"}</div>
                <div className="grid gap-2 sm:grid-cols-2 lg:grid-cols-5">
                  <input placeholder="Nombre" value={newPlayer.first} onChange={(e)=>setNewPlayer({...newPlayer,first:e.target.value})} className="rounded border border-fm-border bg-fm-bg px-2 py-1.5 text-sm" />
                  <input placeholder="Apellidos" value={newPlayer.last} onChange={(e)=>setNewPlayer({...newPlayer,last:e.target.value})} className="rounded border border-fm-border bg-fm-bg px-2 py-1.5 text-sm" />
                  <select value={newPlayer.nation} onChange={(e)=>setNewPlayer({...newPlayer,nation:Number(e.target.value)})} className="rounded border border-fm-border bg-fm-bg px-2 py-1.5 text-sm">
                    {nations.map((n:any)=><option key={n.id} value={n.id}>{n.name}</option>)}
                  </select>
                  <select value={newPlayer.pos} onChange={(e)=>setNewPlayer({...newPlayer,pos:e.target.value})} className="rounded border border-fm-border bg-fm-bg px-2 py-1.5 text-sm"><option>POR</option><option>CIE</option><option>ALA</option><option>PIV</option><option>UNI</option></select>
                  <input placeholder="Club ID" value={newPlayer.club} onChange={(e)=>setNewPlayer({...newPlayer,club:e.target.value})} className="rounded border border-fm-border bg-fm-bg px-2 py-1.5 text-sm" />
                  <input type="number" placeholder="CA" value={newPlayer.ca} onChange={(e)=>setNewPlayer({...newPlayer,ca:Number(e.target.value)})} className="rounded border border-fm-border bg-fm-bg px-2 py-1.5 text-sm" />
                  <input type="number" placeholder="PA" value={newPlayer.pa} onChange={(e)=>setNewPlayer({...newPlayer,pa:Number(e.target.value)})} className="rounded border border-fm-border bg-fm-bg px-2 py-1.5 text-sm" />
                  <button onClick={savePlayer} className="rounded bg-fm-accent px-3 py-1.5 text-sm font-bold text-black">{inEdit(newPlayer) ? "Guardar" : "Crear"}</button>
                </div>
              </div>
              <div className="overflow-hidden rounded-xl border border-fm-border bg-fm-panel">
                <div className="max-h-[26rem] overflow-auto">
                  <table className="w-full text-sm">
                    <thead className="sticky top-0 bg-fm-bg text-xs uppercase tracking-widest text-fm-dim"><tr><th className="px-2 py-2 text-left">Jugador</th><th className="px-2 py-2">Nación</th><th className="px-2 py-2">Club</th><th className="px-2 py-2">Pos</th><th className="px-2 py-2">CA/PA</th><th></th></tr></thead>
                    <tbody>{filtered.length === 0 ? <tr><td colSpan={6} className="p-6"><EmptyState title="No hay jugadores que mostrar" description="Ajusta la búsqueda o crea un jugador libre." /></td></tr> : filtered.map((p:any)=><tr key={p.id} className="border-t border-fm-border hover:bg-fm-panel2">
                      <td className="px-2 py-1.5 font-semibold"><span className="mr-2 inline-flex items-center gap-1"><img src={p.flag_path ?? undefined} alt={p.nation} title={p.nation} className="h-3 w-5 rounded object-cover" onError={(e)=>{e.currentTarget.style.display="none"}} />{p.second_flag_path && <img src={p.second_flag_path} alt="Segunda nacionalidad" title="Segunda nacionalidad" className="h-3 w-5 rounded object-cover" />}</span>{p.common_name}</td><td className="px-2 py-1.5 text-xs">{p.nation}</td><td className="px-2 py-1.5 text-xs">{p.club || "-"}</td><td className="px-2 py-1.5 text-center text-xs">{p.position}</td><td className="px-2 py-1.5 text-center font-mono">{p.ca}/{p.pa}</td>
                      <td className="px-2 py-1.5 text-right space-x-1"><button onClick={()=>setEditingPlayer(p)} className="rounded bg-sky-600 px-2 py-0.5 text-xs text-white">Editar</button><button onClick={()=>delPlayer(p.id)} className="rounded bg-red-600 px-2 py-0.5 text-xs text-white">Borrar</button></td>
                    </tr>)}</tbody>
                  </table>
                </div>
              </div>
            </div>
          )}

          {tab==="staff" && (
            <div className="space-y-4">
              {editingStaff && <StaffEditor staff={editingStaff} nations={nations} clubs={clubs} onClose={()=>{ setEditingStaff(null); load("staff"); }} />}
              <div className="rounded-xl border border-fm-border bg-fm-panel p-3">
                <div className="mb-2 text-xs font-bold uppercase tracking-widest text-fm-dim">{inEdit(newStaff) ? "Editar staff · ID " + newStaff.id : "Nuevo staff"}</div>
                <div className="grid gap-2 sm:grid-cols-2 lg:grid-cols-6">
                  <input placeholder="Nombre" value={newStaff.first} onChange={(e)=>setNewStaff({...newStaff,first:e.target.value})} className="rounded border border-fm-border bg-fm-bg px-2 py-1.5 text-sm" />
                  <input placeholder="Apellidos" value={newStaff.last} onChange={(e)=>setNewStaff({...newStaff,last:e.target.value})} className="rounded border border-fm-border bg-fm-bg px-2 py-1.5 text-sm" />
                  <select value={newStaff.nation} onChange={(e)=>setNewStaff({...newStaff,nation:Number(e.target.value)})} className="rounded border border-fm-border bg-fm-bg px-2 py-1.5 text-sm">{nations.map((n:any)=><option key={n.id} value={n.id}>{n.name}</option>)}</select>
                  <select value={newStaff.role} onChange={(e)=>setNewStaff({...newStaff,role:e.target.value})} className="rounded border border-fm-border bg-fm-bg px-2 py-1.5 text-sm">{["coach","assistant","scout","physio","fitness_coach","goalkeeper_coach","technical_coach","analyst"].map(r=><option key={r}>{r}</option>)}</select>
                  <select value={newStaff.club} onChange={(e)=>setNewStaff({...newStaff,club:e.target.value})} className="rounded border border-fm-border bg-fm-bg px-2 py-1.5 text-sm"><option value="">Libre</option>{clubs.map((c:any)=><option key={c.id} value={c.id}>{c.name}</option>)}</select>
                  <input type="number" placeholder="Salario" value={newStaff.wage} onChange={(e)=>setNewStaff({...newStaff,wage:Number(e.target.value)})} className="rounded border border-fm-border bg-fm-bg px-2 py-1.5 text-sm" />
                  <button onClick={saveStaff} className="rounded bg-fm-accent px-3 py-1.5 text-sm font-bold text-black">{inEdit(newStaff) ? "Guardar" : "Crear"}</button>
                </div>
              </div>
              <div className="overflow-hidden rounded-xl border border-fm-border bg-fm-panel">
                <div className="max-h-[26rem] overflow-auto">
                  <table className="w-full text-sm">
                    <thead className="sticky top-0 bg-fm-bg text-xs uppercase tracking-widest text-fm-dim"><tr><th className="px-2 py-2 text-left">Staff</th><th className="px-2 py-2">Rol</th><th className="px-2 py-2">Club</th><th className="px-2 py-2">Salario</th><th></th></tr></thead>
                    <tbody>{filtered.length === 0 ? <tr><td colSpan={5} className="p-6"><EmptyState title="No hay staff que mostrar" description="Crea preparadores libres para probar el escenario." /></td></tr> : filtered.map((s:any)=><tr key={s.id} className="border-t border-fm-border hover:bg-fm-panel2">
                      <td className="px-2 py-1.5 font-semibold"><span className="mr-2"><img src={s.flag_path ?? undefined} alt={s.nation} title={s.nation} className="inline h-3 w-5 rounded object-cover" onError={(e)=>{e.currentTarget.style.display="none"}} /></span>{s.first_name} {s.last_name}</td><td className="px-2 py-1.5 text-center"><span className="rounded bg-fm-bg px-1.5 py-0.5 text-xs font-bold">{s.role}</span></td><td className="px-2 py-1.5 text-xs">{s.club_name || "libre"}</td><td className="px-2 py-1.5 text-right font-mono text-xs">€{Math.round(s.wage_weekly).toLocaleString()}</td>
                      <td className="px-2 py-1.5 text-right space-x-1"><button onClick={()=>setEditingStaff(s)} className="rounded bg-sky-600 px-2 py-0.5 text-xs text-white">Editar</button><button onClick={()=>delStaff(s.id)} className="rounded bg-red-600 px-2 py-0.5 text-xs text-white">Borrar</button></td>
                    </tr>)}</tbody>
                  </table>
                </div>
              </div>
            </div>
          )}

          {tab==="contracts" && (<div className="overflow-hidden rounded-xl border border-fm-border bg-fm-panel"><div className="border-b border-fm-border bg-fm-bg px-3 py-2 text-xs font-bold uppercase tracking-widest text-fm-dim">Contratos y cesiones</div><div className="overflow-x-auto"><table className="w-full text-sm"><thead className="text-xs uppercase tracking-widest text-fm-dim"><tr><th className="px-2 py-2 text-left">Jugador</th><th>Club</th><th>Salario</th><th>Inicio</th><th>Fin</th><th>Rol</th><th>Cesión</th><th>Activo</th><th></th></tr></thead><tbody>{filtered.map((c:any)=><tr key={c.id} className="border-t border-fm-border"><td className="px-2 py-2 font-semibold">{c.player_name}</td><td className="px-2 py-2 text-xs">{c.club_name}</td><td className="px-2 py-2"><input type="number" defaultValue={c.wage_weekly} className="w-24 rounded border border-fm-border bg-fm-bg px-1 py-1" id={`w-${c.id}`}/></td><td><input type="date" defaultValue={c.start_date} className="rounded border border-fm-border bg-fm-bg px-1 py-1 text-xs" id={`s-${c.id}`}/></td><td><input type="date" defaultValue={c.end_date} className="rounded border border-fm-border bg-fm-bg px-1 py-1 text-xs" id={`e-${c.id}`}/></td><td className="px-2"><input defaultValue={c.role} className="w-24 rounded border border-fm-border bg-fm-bg px-1 py-1 text-xs" id={`r-${c.id}`}/></td><td className="px-2 text-xs">{c.loan_until || "—"}</td><td className="text-center">{c.is_active ? "Sí" : "No"}</td><td><button onClick={async()=>{try{await invoke("editor_update_contract",{id:c.id,clubId:c.club_id,wage:Number((document.getElementById(`w-${c.id}`) as HTMLInputElement).value),startDate:(document.getElementById(`s-${c.id}`) as HTMLInputElement).value,endDate:(document.getElementById(`e-${c.id}`) as HTMLInputElement).value,releaseClause:c.release_clause,role:(document.getElementById(`r-${c.id}`) as HTMLInputElement).value,signingBonus:c.signing_bonus,appearanceBonus:c.appearance_bonus,cleanSheetBonus:c.clean_sheet_bonus,isActive:c.is_active});setMsg("Contrato actualizado");}catch(e){setMsg(String(e));}}} className="rounded bg-fm-accent px-2 py-1 text-xs font-bold text-black">Guardar</button></td></tr>)}</tbody></table></div></div>)}

          {tab==="stadiums" && (<div className="overflow-hidden rounded-xl border border-fm-border bg-fm-panel"><div className="border-b border-fm-border bg-fm-bg px-3 py-2 text-xs font-bold uppercase tracking-widest text-fm-dim">Pabellones · {filtered.length}</div><div className="overflow-x-auto"><table className="w-full text-sm"><thead className="text-xs uppercase tracking-widest text-fm-dim"><tr><th className="px-2 py-2 text-left">Pabellón</th><th>Ciudad</th><th>Capacidad</th><th>Club</th><th>Foto</th><th></th></tr></thead><tbody>{filtered.map((s:any)=><tr key={s.id} className="border-t border-fm-border"><td className="px-2 py-2 font-semibold">{s.name}</td><td className="px-2 py-2 text-xs">{s.city}</td><td className="px-2 py-2 text-center">{s.capacity}</td><td className="px-2 py-2 text-xs">{s.club_name||"Libre"}</td><td className="px-2 py-2">{s.photo_path?<img src={s.photo_path} alt={s.name} className="h-8 w-12 rounded object-cover"/>:"—"}</td><td className="px-2 py-2"><button onClick={async()=>{const name=prompt("Nombre",s.name);if(!name)return;try{await invoke("editor_update_stadium",{id:s.id,name,cityId:s.city_id,capacity:s.capacity,pitchType:s.pitch_type});load("stadiums");}catch(e){setMsg(String(e));}}} className="mr-1 rounded bg-sky-600 px-2 py-1 text-xs">Editar</button><button onClick={async()=>{if(!confirm("¿Borrar pabellón?"))return;try{await invoke("editor_delete_stadium",{id:s.id});load("stadiums");}catch(e){setMsg(String(e));}}} className="rounded bg-red-600 px-2 py-1 text-xs">Borrar</button></td></tr>)}</tbody></table></div></div>)}

          {tab==="nations" && (
            <div className="space-y-4">
              {editingNation && <NationEditor nation={editingNation} confeds={confeds} onClose={()=>{ setEditingNation(null); load("nations"); }} />}
              <div className="rounded-xl border border-fm-border bg-fm-panel p-3">
                <div className="mb-2 text-xs font-bold uppercase tracking-widest text-fm-dim">{inEdit(newNation) ? "Editar nación · ID " + newNation.id : "Nueva nación"}</div>
                <div className="flex flex-wrap gap-2">
                  <input placeholder="Nombre" value={newNation.name} onChange={(e)=>setNewNation({...newNation,name:e.target.value})} className="rounded border border-fm-border bg-fm-bg px-2 py-1.5 text-sm" />
                  <select value={newNation.conf} onChange={(e)=>setNewNation({...newNation,conf:Number(e.target.value)})} className="rounded border border-fm-border bg-fm-bg px-2 py-1.5 text-sm">{confeds.map((c:any)=><option key={c.id} value={c.id}>{c.name}</option>)}</select>
                  <input type="number" placeholder="Rep" value={newNation.rep} onChange={(e)=>setNewNation({...newNation,rep:Number(e.target.value)})} className="w-24 rounded border border-fm-border bg-fm-bg px-2 py-1.5 text-sm" />
                  <input type="number" placeholder="Nivel" value={newNation.level} onChange={(e)=>setNewNation({...newNation,level:Number(e.target.value)})} className="w-24 rounded border border-fm-border bg-fm-bg px-2 py-1.5 text-sm" />
                  <button onClick={saveNation} className="rounded bg-fm-accent px-3 py-1.5 text-sm font-bold text-black">{inEdit(newNation) ? "Guardar" : "Crear"}</button>
                </div>
              </div>
              <div className="overflow-hidden rounded-xl border border-fm-border bg-fm-panel">
                <div className="max-h-[26rem] overflow-auto">
                  <table className="w-full text-sm">
                    <thead className="sticky top-0 bg-fm-bg text-xs uppercase tracking-widest text-fm-dim"><tr><th className="px-2 py-2 text-left">País</th><th className="px-2 py-2">Federación</th><th className="px-2 py-2">Rep</th><th className="px-2 py-2">Nivel</th><th></th></tr></thead>
                    <tbody>{filtered.length === 0 ? <tr><td colSpan={5} className="p-6"><EmptyState title="No hay países que mostrar" /></td></tr> : filtered.map((n:any)=><tr key={n.id} className="border-t border-fm-border hover:bg-fm-panel2">
                      <td className="px-2 py-1.5 font-semibold"><span className="mr-2"><img src={n.flag_path ?? undefined} alt={n.name} title={n.name} className="inline h-3 w-5 rounded object-cover" onError={(e)=>{e.currentTarget.style.display="none"}} /></span>{n.name}</td><td className="px-2 py-1.5 text-xs">{n.confederation}</td><td className="px-2 py-1.5 text-center font-mono">{n.reputation}</td><td className="px-2 py-1.5 text-center font-mono">{n.futsal_level}</td>
                      <td className="px-2 py-1.5 text-right space-x-1"><button onClick={()=>setEditingNation(n)} className="rounded bg-sky-600 px-2 py-0.5 text-xs text-white">Editar</button><button onClick={()=>delNation(n.id)} className="rounded bg-red-600 px-2 py-0.5 text-xs text-white">Borrar</button></td>
                    </tr>)}</tbody>
                  </table>
                </div>
              </div>
            </div>
          )}

          {tab==="competitions" && (
            <div className="space-y-4">
              <div className="rounded-xl border border-fm-border bg-fm-panel p-3">
                <div className="mb-2 text-xs font-bold uppercase tracking-widest text-fm-dim">{inEdit(newComp) ? "Editar competición · ID " + newComp.id : "Nueva competición"}</div>
                <div className="flex flex-wrap gap-2">
                  <input placeholder="Nombre" value={newComp.name} onChange={(e)=>setNewComp({...newComp,name:e.target.value})} className="rounded border border-fm-border bg-fm-bg px-2 py-1.5 text-sm" />
                  <select value={newComp.nation} onChange={(e)=>setNewComp({...newComp,nation:e.target.value})} className="rounded border border-fm-border bg-fm-bg px-2 py-1.5 text-sm"><option value="">Internacional</option>{nations.map((n:any)=><option key={n.id} value={n.id}>{n.name}</option>)}</select>
                  <input placeholder="Tier" value={newComp.tier} onChange={(e)=>setNewComp({...newComp,tier:e.target.value})} className="w-16 rounded border border-fm-border bg-fm-bg px-2 py-1.5 text-sm" />
                  <input type="number" placeholder="Equipos" value={newComp.teams} onChange={(e)=>setNewComp({...newComp,teams:Number(e.target.value)})} className="w-24 rounded border border-fm-border bg-fm-bg px-2 py-1.5 text-sm" />
                  <input placeholder="Temporada" value={newComp.season} onChange={(e)=>setNewComp({...newComp,season:e.target.value})} className="w-32 rounded border border-fm-border bg-fm-bg px-2 py-1.5 text-sm" />
                  <button onClick={saveComp} className="rounded bg-fm-accent px-3 py-1.5 text-sm font-bold text-black">{inEdit(newComp) ? "Guardar" : "Crear"}</button>
                </div>
              </div>
              <div className="overflow-hidden rounded-xl border border-fm-border bg-fm-panel">
                <div className="max-h-[26rem] overflow-auto">
                  <table className="w-full text-sm">
                    <thead className="sticky top-0 bg-fm-bg text-xs uppercase tracking-widest text-fm-dim"><tr><th className="px-2 py-2 text-left">Competición</th><th className="px-2 py-2">Tipo</th><th className="px-2 py-2">Nación</th><th className="px-2 py-2">Tier</th><th className="px-2 py-2">Equipos</th><th className="px-2 py-2">Temp.</th><th></th></tr></thead>
                    <tbody>{filtered.map((c:any)=><tr key={c.id} className="border-t border-fm-border hover:bg-fm-panel2">
                      <td className="px-2 py-1.5 font-semibold">{c.name}</td><td className="px-2 py-1.5 text-center"><span className={`rounded px-1.5 py-0.5 text-[10px] font-bold ${c.kind==="national_team" ? "bg-violet-500/20 text-violet-300" : "bg-sky-500/20 text-sky-300"}`}>{c.kind==="national_team" ? "Sel." : "Club"}</span></td><td className="px-2 py-1.5 text-xs">{c.nation || "—"}</td><td className="px-2 py-1.5 text-center">{c.tier ?? "—"}</td><td className="px-2 py-1.5 text-center">{c.total_teams ?? "—"}</td><td className="px-2 py-1.5 text-xs">{c.season}</td>
                      <td className="px-2 py-1.5 text-right space-x-1"><button onClick={()=>setNewComp({ id: c.id, name: c.name, nation: c.nation_id ? String(c.nation_id) : "", tier: c.tier ? String(c.tier) : "", teams: c.total_teams ?? 16, season: c.season })} className="rounded bg-sky-600 px-2 py-0.5 text-xs text-white">Editar</button><button onClick={()=>delComp(c.id)} className="rounded bg-red-600 px-2 py-0.5 text-xs text-white">Borrar</button></td>
                    </tr>)}</tbody>
                  </table>
                </div>
              </div>
            </div>
          )}
        </>
      )}
    </div>
  );
}
