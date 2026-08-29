import { useEffect, useState } from "react";
import { api, type ClubRow, type CompRow } from "../../api";
import { useStore } from "../../store";

export default function NewGame() {
  const { setScreen, setGameState, setClubs, setCompetitions, setSelectedComp, setUserClub } = useStore();
  const [clubs, setLocalClubs] = useState<ClubRow[]>([]);
  const [comps, setComps] = useState<CompRow[]>([]);
  const [mode, setMode] = useState<"clubs" | "selecciones">("clubs");
  const [filter, setFilter] = useState<string>("all");
  const [loading, setLoading] = useState(true);
  const [creating, setCreating] = useState<number | null>(null);
  const [search, setSearch] = useState("");
  const [selectedClub, setSelectedClub] = useState<ClubRow | null>(null);

  useEffect(() => {
    api.newGame().then((res) => {
      setLocalClubs(res.clubs);
      setComps(res.competitions);
      setLoading(false);
    }).catch(() => setLoading(false));
  }, []);

  const start = async (clubId: number) => {
    setCreating(clubId);
    try {
      const res = await api.newGame(clubId);
      const state = await api.getGameState();
      setClubs(res.clubs);
      setCompetitions(res.competitions);
      setGameState(state);
      setUserClub(clubId);
      const comp = res.competitions.find((c) => {
        const club = res.clubs.find((x) => x.id === clubId);
        return club ? c.nation === club.nation && c.kind === "club" : false;
      });
      setSelectedComp(comp ? comp.id : res.competitions.find((c)=>c.kind==="club")?.id ?? null);
      setScreen("dashboard");
    } catch (e) {
      alert(String(e));
    } finally {
      setCreating(null);
    }
  };

  if (loading) return <div className="p-12 text-center text-fm-dim">Generando mundo…</div>;

  // Competiciones por tipo (clubes / selecciones)
  const natComps = comps.filter((c) => c.kind === "national_team");

  // Naciones dinámicas según los clubes realmente existentes
  const nations = Array.from(new Set(clubs.map((c) => c.nation)));
  const normalizedSearch = search.trim().toLowerCase();
  const filtered = (filter === "all" ? clubs : clubs.filter((c) => c.nation === filter)).filter((c) => !normalizedSearch || `${c.name} ${c.short_name} ${c.nation}`.toLowerCase().includes(normalizedSearch));

  // Agrupar por nación y, dentro de cada nación, por división (tier asc) + sin liga
  const byNation: Record<string, ClubRow[]> = {};
  filtered.forEach((c) => { (byNation[c.nation] ??= []).push(c); });
  const tierName = (tier: number | null) => tier === 1 ? "1ª División" : tier === 2 ? "2ª División" : tier === 3 ? "3ª División (Segunda B)" : "Sin liga";

  return (
    <div className="mx-auto max-w-6xl p-6">
      <div className="mb-8 text-center">
        <h1 className="text-4xl font-black tracking-tight"><span className="text-fm-accent">FUTSAL</span> MANAGER 27</h1>
        <p className="mt-2 text-fm-dim">Elige un proyecto y construye tu dinastía de futsal en la temporada 2026/27</p>
        <div className="mx-auto mt-5 grid max-w-3xl grid-cols-3 gap-2 text-left text-xs">
          <div className="rounded-xl border border-fm-border bg-fm-panel p-3"><div className="font-black text-fm-accent">{clubs.length}</div><div className="text-fm-dim">clubes disponibles</div></div>
          <div className="rounded-xl border border-fm-border bg-fm-panel p-3"><div className="font-black text-fm-accent">{nations.length}</div><div className="text-fm-dim">países jugables</div></div>
          <div className="rounded-xl border border-fm-border bg-fm-panel p-3"><div className="font-black text-fm-accent">2026/27</div><div className="text-fm-dim">nueva carrera</div></div>
        </div>

        {/* Discriminador Clubes / Selecciones */}
        <div className="mt-5 inline-flex items-center gap-1 rounded-full border border-fm-border bg-fm-panel p-1">
          <button onClick={() => setMode("clubs")} className={`rounded-full px-5 py-2 text-sm font-bold ${mode==="clubs" ? "bg-fm-accent text-black" : "text-fm-dim hover:text-white"}`}>🏟  Clubes</button>
          <button onClick={() => setMode("selecciones")} className={`rounded-full px-5 py-2 text-sm font-bold ${mode==="selecciones" ? "bg-fm-accent text-black" : "text-fm-dim hover:text-white"}`}>🌍  Selecciones</button>
        </div>
      </div>

      {mode === "clubs" ? (
        <>
          {/* Filtro por país y búsqueda */}
          <div className="mb-4 flex flex-col items-stretch justify-center gap-3 sm:flex-row">
            <input value={search} onChange={(e)=>setSearch(e.target.value)} placeholder="Buscar club, abreviatura o país…" className="rounded-full border border-fm-border bg-fm-panel px-4 py-2 text-sm outline-none focus:border-fm-accent sm:w-80" />
            <span className="self-center text-xs text-fm-dim">{filtered.length} resultados</span>
          </div>
          <div className="mb-6 flex flex-wrap justify-center gap-2">
            <button onClick={() => setFilter("all")} className={`rounded-full px-4 py-1.5 text-sm font-semibold ${filter==="all" ? "bg-fm-accent text-black" : "bg-fm-panel border border-fm-border text-fm-dim hover:text-white"}`}>Todas</button>
            {nations.map((n) => (
              <button key={n} onClick={() => setFilter(n)} className={`rounded-full px-4 py-1.5 text-sm font-semibold ${filter===n ? "bg-fm-accent text-black" : "bg-fm-panel border border-fm-border text-fm-dim hover:text-white"}`}>{n}</button>
            ))}
            <button onClick={()=>setScreen("editor")} className="rounded-full border border-amber-500/30 bg-amber-500/10 px-4 py-1.5 text-sm font-semibold text-amber-400 hover:bg-amber-500/20">Editor BD</button>
          </div>

          {Object.entries(byNation).map(([nation, list]) => {
            if (list.length === 0) return null;
            // Agrupar por división dentro del país
            const byDivision = new Map<number, ClubRow[]>(); // key tier (null -> -1)
            list.forEach((c) => {
              const key = c.tier ?? -1;
              if (!byDivision.has(key)) byDivision.set(key, []);
              byDivision.get(key)!.push(c);
            });
            const tiers = Array.from(byDivision.keys()).sort((a, b) => {
              if (a === -1) return 1; if (b === -1) return -1; return a - b;
            });
            return (
              <div key={nation} className="mb-8">
                <h2 className="mb-3 flex items-center gap-2 text-sm font-bold uppercase tracking-widest text-fm-dim"><span className="h-px w-6 bg-fm-border"/> {nation} <span className="rounded bg-fm-panel2 px-2 py-0.5 text-xs normal-case">{list.length} clubes</span></h2>
                <div className="space-y-5">
                  {tiers.map((tierKey) => {
                    const divList = byDivision.get(tierKey)!.sort((a, b) => b.reputation - a.reputation);
                    return (
                      <div key={tierKey}>
                        <h3 className="mb-2 flex items-center gap-2 text-xs font-bold text-fm-dim">
                          <span>{tierKey === -1 ? "Sin liga" : tierName(tierKey)}</span>
                          <span className="rounded bg-fm-panel2 px-1.5 py-0.5 text-[10px] normal-case">{divList.length} equipos</span>
                        </h3>
                        <div className="grid grid-cols-1 gap-3 sm:grid-cols-2 lg:grid-cols-4">
                          {divList.map((c) => (
                            <button key={c.id} onClick={() => setSelectedClub(c)} disabled={creating!==null} className={`group flex items-center gap-3 rounded-xl border p-3 text-left transition hover:border-fm-accent/50 hover:bg-fm-panel2 disabled:opacity-60 ${selectedClub?.id===c.id ? "border-fm-accent bg-fm-accent/10" : "border-fm-border bg-fm-panel"}`}>
                              <span className="flex h-10 w-10 items-center justify-center rounded-lg text-xs font-black text-white" style={{ background: c.primary_color }}>{c.short_name}</span>
                              <span className="min-w-0 flex-1">
                                <span className="block truncate text-sm font-semibold">{c.name}</span>
                                <span className="block text-xs text-fm-dim">Rep {c.reputation}</span>
                              </span>
                              <span className="text-fm-dim group-hover:text-fm-accent">{creating===c.id ? "…" : "＋"}</span>
                            </button>
                          ))}
                        </div>
                      </div>
                    );
                  })}
                </div>
              </div>
            );
          })}
          {filtered.length === 0 && <div className="p-12 text-center text-fm-dim">No hay clubes que coincidan con la búsqueda.</div>}
          {selectedClub && (
            <div className="sticky bottom-4 z-10 mt-5 flex flex-wrap items-center justify-between gap-4 rounded-2xl border border-fm-accent/40 bg-fm-panel/95 p-4 shadow-2xl backdrop-blur">
              <div className="flex items-center gap-3"><span className="flex h-12 w-12 items-center justify-center rounded-xl text-xs font-black text-white" style={{background:selectedClub.primary_color}}>{selectedClub.short_name}</span><div><div className="text-xs uppercase tracking-widest text-fm-accent">Club seleccionado</div><div className="font-black">{selectedClub.name}</div><div className="text-xs text-fm-dim">{selectedClub.nation} · {tierName(selectedClub.tier)} · Reputación {selectedClub.reputation}</div></div></div>
              <button onClick={()=>start(selectedClub.id)} disabled={creating!==null} className="rounded-xl bg-fm-accent px-5 py-2.5 text-sm font-black text-black hover:brightness-110 disabled:opacity-50">{creating===selectedClub.id ? "Preparando…" : "Comenzar carrera →"}</button>
            </div>
          )}
        </>
      ) : (
        /* Modo selecciones: catálogo de competiciones de selecciones */
        <div className="space-y-6">
          <div className="rounded-xl border border-fm-border bg-fm-panel p-4">
            <h2 className="mb-1 text-lg font-black">Competiciones de selecciones <span className="font-normal text-fm-dim">({natComps.length})</span></h2>
            <p className="text-sm text-fm-dim">Mundial, continentales y copas nacionales de selecciones. Estas competiciones se añadirán al mundo del juego; en esta versión aún no se gestionan plantillas nacionales, así que se muestran como catálogo.</p>
          </div>
          <div className="grid gap-3 sm:grid-cols-2 lg:grid-cols-3">
            {natComps.map((c) => (
              <div key={c.id} className="rounded-xl border border-fm-border bg-fm-panel p-4">
                <div className="text-xs uppercase tracking-widest text-fm-dim">🌍 Selecciones</div>
                <div className="mt-1 font-semibold">{c.name}</div>
                <div className="mt-1 text-xs text-fm-dim">{c.nation || "Internacional"}</div>
                <button onClick={()=>setMode("clubs")} className="mt-3 rounded-lg border border-fm-border px-3 py-1.5 text-xs font-semibold text-fm-dim hover:text-white">Jugar con clubes →</button>
              </div>
            ))}
          </div>
          {natComps.length === 0 && <div className="p-12 text-center text-fm-dim">No hay competiciones de selecciones. Usa el Editor para añadirlas.</div>}
        </div>
      )}
    </div>
  );
}
