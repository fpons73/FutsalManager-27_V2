import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import ImagePicker from "./ImagePicker";

type Attr = { ca:number; pa:number; position:string } & Record<string, number>;

const GROUPS: Record<string, string[]> = {
  "Técnica": ["firstTouch","dribbling","ballControl","technique","passing","vision","crossing","longShots","finishing","heading","penaltyTaking","tackling","marking","interception","blocking"],
  "Mental": ["anticipation","decisions","positioning","offTheBall","workRate","composure","concentration","determination","bravery","aggression","leadership","teamwork","flair"],
  "Físico": ["acceleration","pace","agility","balance","stamina","strength","jumping"],
  "Portero": ["reflexes","handling","oneOnOnes","positioningGk","rushingOut","throwing","kicking"],
  "Oculto": ["professionalism","consistency","importantMatches","injuryProneness"],
};

const LABELS: Record<string, string> = {
  firstTouch:"Primer toque", dribbling:"Regate", ballControl:"Control", technique:"Técnica", passing:"Pase", vision:"Visión", crossing:"Centro", longShots:"Tiro lejano", finishing:"Definición", heading:"Cabeceo", penaltyTaking:"Penaltis", tackling:"Entrada", marking:"Marcaje", interception:"Intercepción", blocking:"Bloqueo",
  anticipation:"Anticipación", decisions:"Decisiones", positioning:"Colocación", offTheBall:"Desmarque", workRate:"Trabajo", composure:"Calma", concentration:"Concentración", determination:"Determinación", bravery:"Valentía", aggression:"Agresividad", leadership:"Liderazgo", teamwork:"Trabajo equipo", flair:"Imaginación",
  acceleration:"Aceleración", pace:"Velocidad", agility:"Agilidad", balance:"Equilibrio", stamina:"Resistencia", strength:"Fuerza", jumping:"Salto",
  reflexes:"Reflejos", handling:"Manejo", oneOnOnes:"Uno contra uno", positioningGk:"Colocación portero", rushingOut:"Salidas", throwing:"Saques mano", kicking:"Saques pie",
  professionalism:"Profesionalidad", consistency:"Regularidad", importantMatches:"Partid. import.", injuryProneness:"Propensión lesión",
};

// Atributos de habilidad para calcular la Calidad Actual (CA)
// Se usa la media de TODOS los atributos de habilidad (técnica+mental+físico y portería si es POR),
// de modo que subir cualquier atributo suba (o no baje) la CA. Se excluyen los ocultos (personalidad).
const TECHNICAL = ["firstTouch","dribbling","ballControl","technique","passing","vision","crossing","longShots","finishing","heading","penaltyTaking","tackling","marking","interception","blocking"];
const MENTAL = ["anticipation","decisions","positioning","offTheBall","workRate","composure","concentration","determination","bravery","aggression","leadership","teamwork","flair"];
const PHYSICAL = ["acceleration","pace","agility","balance","stamina","strength","jumping"];
const GK = ["reflexes","handling","oneOnOnes","positioningGk","rushingOut","throwing","kicking"];

function calcCA(a: Attr): number {
  let keys = [...TECHNICAL, ...MENTAL, ...PHYSICAL];
  if (a.position === "POR") keys = keys.concat(GK);
  const vals = keys.map((k) => a[k] ?? 50);
  const avg = vals.reduce((s, v) => s + v, 0) / vals.length;
  return Math.max(1, Math.min(200, Math.round(avg * 2)));
}

function Num({ label, v, onChange, auto, max = 100 }: { label:string; v:number; onChange:(n:number)=>void; auto?: boolean; max?: number }) {
  const clamp = (n:number) => Math.max(0, Math.min(max, n));
  return (
    <label className="flex flex-col">
      <span className="text-[10px] uppercase tracking-wider text-fm-dim">{label}</span>
      <div className="flex items-center gap-1">
        <input type="number" min={0} max={max} value={v} onChange={(e)=>onChange(clamp(Number(e.target.value)))} className="w-16 rounded border border-fm-border bg-fm-bg px-1.5 py-1 font-mono text-sm" />
      </div>
      {auto && <span className="text-[9px] text-fm-accent">auto</span>}
    </label>
  );
}

export default function PlayerEditor({ player, nations, onClose }: { player:any; nations:any[]; onClose:()=>void }) {
  const [first, setFirst] = useState(player.first_name ?? "");
  const [last, setLast] = useState(player.last_name ?? "");
  const [nationId, setNationId] = useState(player.nation_id ?? 1);
  const [secondNationId, setSecondNationId] = useState<number | null>(player.second_nation_id ?? null);
  const [clubId, setClubId] = useState<string>(player.club_id ? String(player.club_id) : "");
  const [secondaryPosition, setSecondaryPosition] = useState(player.secondary_position ?? "");
  const [attrs, setAttrs] = useState<Attr | null>(null);
  const [autoCalc, setAutoCalc] = useState(true);
  const [msg, setMsg] = useState<string | null>(null);

  useEffect(() => {
    invoke<Attr>("editor_get_player_attributes", { playerId: player.id }).then((a)=>{
      const next = { ...a };
      const ca = calcCA(next as Attr);
      next.ca = ca;
      if (next.pa < ca) next.pa = ca;
      setAttrs(next);
    }).catch((e)=>setMsg(String(e)));
  }, [player.id]);

  // Al cambiar un atributo, si autoCalc está activo, recalcular CA y ajustar PA
  const applyAttr = (k:string, n:number) => {
    setAttrs((a)=>{
      if (!a) return a;
      const next = { ...(a as any), [k]: n };
      if (autoCalc) {
        const ca = calcCA(next as Attr);
        next.ca = ca;
        if (next.pa < ca) next.pa = ca;
      }
      return next as Attr;
    });
  };

  const saveIdentity = async () => {
    try {
      await invoke("editor_update_player", { id: player.id, first, last, nationId, secondNationId, secondaryPosition: secondaryPosition || null, clubId: clubId ? Number(clubId) : null, ca: attrs?.ca ?? 0, pa: attrs?.pa ?? 0, pos: attrs?.position ?? player.position });
      setMsg("Identidad guardada");
    } catch (e) { setMsg(String(e)); }
  };
  const saveAttrs = async () => {
    if (!attrs) return;
    try {
      await invoke("editor_update_player_attributes", { playerId: player.id, attributes: attrs });
      setMsg("Atributos guardados");
    } catch (e) { setMsg(String(e)); }
  };

  if (!attrs) return <div className="p-6 text-center text-fm-dim">Cargando atributos…</div>;

  return (
    <div className="rounded-xl border border-sky-500/30 bg-fm-panel p-4">
      <div className="mb-3 flex items-center justify-between">
        <h3 className="text-lg font-black"><span className="mr-2 inline-flex items-center gap-1"><img src={player.flag_path ?? undefined} alt={player.nation} title={player.nation} className="h-4 w-6 rounded object-cover" onError={(e)=>{e.currentTarget.style.display="none"}} />{player.second_flag_path && <img src={player.second_flag_path} alt="Segunda nacionalidad" title="Segunda nacionalidad" className="h-4 w-6 rounded object-cover" />}</span>Jugador · {player.first_name} {player.last_name} <span className="text-fm-dim">(ID {player.id})</span></h3>
        <button onClick={onClose} className="rounded-lg border border-fm-border px-3 py-1 text-sm text-fm-dim hover:text-white">Cerrar</button>
      </div>
      {msg && <div className="mb-3 rounded-lg border border-amber-500/30 bg-amber-500/10 px-3 py-2 text-sm">{msg}</div>}

      <div className="grid gap-4 lg:grid-cols-3">
        <div className="space-y-3 rounded-lg border border-fm-border bg-fm-bg p-3">
          <ImagePicker command="editor_set_player_photo" entityId={player.id} label="Foto" value={player.photo_path ?? null} prefix="F" />
          <div className="space-y-2 text-sm">
            <div><label className="block text-[10px] uppercase tracking-wider text-fm-dim">Nombre</label><input value={first} onChange={(e)=>setFirst(e.target.value)} className="w-full rounded border border-fm-border bg-fm-bg px-2 py-1" /></div>
            <div><label className="block text-[10px] uppercase tracking-wider text-fm-dim">Apellidos</label><input value={last} onChange={(e)=>setLast(e.target.value)} className="w-full rounded border border-fm-border bg-fm-bg px-2 py-1" /></div>
            <div><label className="block text-[10px] uppercase tracking-wider text-fm-dim">Nacionalidad principal</label>
              <select value={nationId} onChange={(e)=>setNationId(Number(e.target.value))} className="w-full rounded border border-fm-border bg-fm-bg px-2 py-1">
                {nations.map((n:any)=><option key={n.id} value={n.id}>{n.name}</option>)}
              </select>
            </div>
            <div><label className="block text-[10px] uppercase tracking-wider text-fm-dim">Segunda nacionalidad</label><select value={secondNationId ?? ""} onChange={(e)=>setSecondNationId(Number(e.target.value)||null)} className="w-full rounded border border-fm-border bg-fm-bg px-2 py-1"><option value="">Ninguna</option>{nations.map((n:any)=><option key={n.id} value={n.id}>{n.name}</option>)}</select></div>
            <div><label className="block text-[10px] uppercase tracking-wider text-fm-dim">Club ID</label><input value={clubId} onChange={(e)=>setClubId(e.target.value)} className="w-full rounded border border-fm-border bg-fm-bg px-2 py-1" /></div>
            <button onClick={saveIdentity} className="mt-1 w-full rounded bg-fm-accent px-2 py-1.5 text-sm font-bold text-black">Guardar identidad</button>
          </div>
        </div>

        <div className="lg:col-span-2 space-y-4 rounded-lg border border-fm-border bg-fm-bg p-3">
          <div className="flex flex-wrap items-end gap-3">
            <div>
              <label className="block text-[10px] uppercase tracking-wider text-fm-dim">Posición principal</label>
              <select value={attrs.position} onChange={(e)=>setAttrs((a)=> a ? ({ ...(a as any), position: e.target.value }) : a)} className="rounded border border-fm-border bg-fm-panel px-2 py-1 text-sm">
                {["POR","CIE","ALA","PIV","UNI"].map((p)=> <option key={p} value={p}>{p}</option>)}
              </select>
            </div>
            <div><label className="block text-[10px] uppercase tracking-wider text-fm-dim">Posición secundaria</label><select value={secondaryPosition} onChange={(e)=>setSecondaryPosition(e.target.value)} className="rounded border border-fm-border bg-fm-panel px-2 py-1 text-sm"><option value="">Ninguna</option>{["POR","CIE","ALA","PIV","UNI"].map((p)=><option key={p}>{p}</option>)}</select></div>
            <Num label="CA (Calidad Actual)" v={attrs.ca} max={200} onChange={(n)=>setAttrs((a)=> a ? ({ ...(a as any), ca: n }) : a)} auto={autoCalc} />
            <Num label="CP (Potencial)" v={attrs.pa} max={200} onChange={(n)=>setAttrs((a)=> a ? ({ ...(a as any), pa: n }) : a)} />
          </div>
          <label className="flex items-center gap-2 text-xs text-fm-dim">
            <input type="checkbox" checked={autoCalc} onChange={(e)=>setAutoCalc(e.target.checked)} className="accent-fm-accent" />
            Calcular CA automáticamente según los atributos (si lo desactivas, puedes fijarla a mano). CP ≥ CA siempre.
          </label>

          {Object.entries(GROUPS).map(([group, keys])=>(
            <div key={group}>
              <div className="mb-1 text-xs font-bold uppercase tracking-widest text-fm-dim">{group}</div>
              <div className="grid grid-cols-2 gap-2 sm:grid-cols-4 lg:grid-cols-6">
                {keys.map((k)=> <Num key={k} label={LABELS[k] ?? k} v={attrs[k] ?? 50} onChange={(n)=>applyAttr(k, n)} />)}
              </div>
            </div>
          ))}

          <div className="flex items-center gap-3">
            <button onClick={saveAttrs} className="rounded bg-fm-accent px-4 py-1.5 text-sm font-bold text-black">Guardar atributos</button>
            <button onClick={( )=>{ setAttrs((a)=> a ? ({ ...(a as any), ca: calcCA(a), pa: Math.max(a.pa, calcCA(a)) }) : a); setMsg("CA recalculada"); }} className="rounded border border-fm-border px-3 py-1.5 text-sm text-fm-dim hover:text-white">Recalcular CA</button>
          </div>
          <div className="text-xs text-fm-dim">La posición se aplica al guardar atributos (actualiza player_positions) y define qué atributos ponderan el cálculo de CA.</div>
        </div>
      </div>
    </div>
  );
}
