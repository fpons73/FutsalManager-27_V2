import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import ImagePicker from "./ImagePicker";
import Flag from "../Flag";

export default function NationEditor({ nation, confeds, onClose }: { nation:any; confeds:any[]; onClose:()=>void }) {
  const [name, setName] = useState(nation.name ?? "");
  const [confId, setConfId] = useState(nation.confederation_id ?? 1);
  const [rep, setRep] = useState(nation.reputation ?? 500);
  const [level, setLevel] = useState(nation.futsal_level ?? 50);
  const [confName, setConfName] = useState(nation.confederation ?? "");
  const [confShort, setConfShort] = useState("");
  const [confRep, setConfRep] = useState(1000);
  const [msg, setMsg] = useState<string | null>(null);

  const conf = confeds.find((c:any)=>c.id === confId);

  const saveNation = async () => {
    try {
      await invoke("editor_update_nation", { id: nation.id, name, confederationId: confId, reputation: rep, futsalLevel: level });
      setMsg("Nación guardada");
    } catch (e) { setMsg(String(e)); }
  };
  const saveConfed = async () => {
    try {
      await invoke("editor_update_confederation", { id: confId, name: confName || (conf?.name ?? ""), shortName: confShort || (conf?.short_name ?? ""), reputation: confRep });
      setMsg("Federación guardada");
    } catch (e) { setMsg(String(e)); }
  };

  return (
    <div className="rounded-xl border border-sky-500/30 bg-fm-panel p-4">
      <div className="mb-3 flex items-center justify-between">
        <h3 className="text-lg font-black">País · {nation.name} <span className="text-fm-dim">(ID {nation.id})</span></h3>
        <button onClick={onClose} className="rounded-lg border border-fm-border px-3 py-1 text-sm text-fm-dim hover:text-white">Cerrar</button>
      </div>
      {msg && <div className="mb-3 rounded-lg border border-amber-500/30 bg-amber-500/10 px-3 py-2 text-sm">{msg}</div>}

      <div className="grid gap-4 lg:grid-cols-3">
        {/* Bandera del país */}
        <div className="rounded-lg border border-fm-border bg-fm-bg p-3">
          <div className="mb-2 text-xs font-bold uppercase tracking-widest text-fm-dim">Bandera del país</div>
          <div className="flex items-center gap-2"><Flag src={nation.flag_path} alt={nation.name} className="h-4 w-6" /><ImagePicker command="editor_set_nation_flag" entityId={nation.id} label="Bandera" value={nation.flag_path ?? null} prefix="🏳" /></div>
        </div>

        {/* Datos del país */}
        <div className="space-y-2 rounded-lg border border-fm-border bg-fm-bg p-3 text-sm">
          <div className="mb-1 text-xs font-bold uppercase tracking-widest text-fm-dim">Datos</div>
          <div><label className="block text-[10px] uppercase tracking-wider text-fm-dim">Nombre</label><input value={name} onChange={(e)=>setName(e.target.value)} className="w-full rounded border border-fm-border bg-fm-bg px-2 py-1" /></div>
          <div><label className="block text-[10px] uppercase tracking-wider text-fm-dim">Federación</label>
            <select value={confId} onChange={(e)=>setConfId(Number(e.target.value))} className="w-full rounded border border-fm-border bg-fm-bg px-2 py-1">
              {confeds.map((c:any)=><option key={c.id} value={c.id}>{c.name}</option>)}
            </select>
          </div>
          <div><label className="block text-[10px] uppercase tracking-wider text-fm-dim">Reputación</label><input type="number" value={rep} onChange={(e)=>setRep(Number(e.target.value))} className="w-full rounded border border-fm-border bg-fm-bg px-2 py-1" /></div>
          <div><label className="block text-[10px] uppercase tracking-wider text-fm-dim">Nivel futsal</label><input type="number" value={level} onChange={(e)=>setLevel(Number(e.target.value))} className="w-full rounded border border-fm-border bg-fm-bg px-2 py-1" /></div>
          <button onClick={saveNation} className="w-full rounded bg-fm-accent px-2 py-1.5 text-sm font-bold text-black">Guardar país</button>
        </div>

        {/* Escudo federación */}
        <div className="space-y-2 rounded-lg border border-fm-border bg-fm-bg p-3 text-sm">
          <div className="mb-2 text-xs font-bold uppercase tracking-widest text-fm-dim">Escudo de la federación</div>
          <ImagePicker command="editor_set_confed_crest" entityId={confId} label="Escudo federación" value={conf?.crest_path ?? null} prefix="⛨" />
          <div><label className="block text-[10px] uppercase tracking-wider text-fm-dim">Nombre</label><input value={confName} onChange={(e)=>setConfName(e.target.value)} placeholder={conf?.name ?? ""} className="w-full rounded border border-fm-border bg-fm-bg px-2 py-1" /></div>
          <div><label className="block text-[10px] uppercase tracking-wider text-fm-dim">Corto</label><input value={confShort} onChange={(e)=>setConfShort(e.target.value)} placeholder={conf?.short_name ?? ""} className="w-full rounded border border-fm-border bg-fm-bg px-2 py-1" /></div>
          <div><label className="block text-[10px] uppercase tracking-wider text-fm-dim">Reputación</label><input type="number" value={confRep} onChange={(e)=>setConfRep(Number(e.target.value))} placeholder={String(conf?.reputation ?? 1000)} className="w-full rounded border border-fm-border bg-fm-bg px-2 py-1" /></div>
          <button onClick={saveConfed} className="w-full rounded bg-fm-accent px-2 py-1.5 text-sm font-bold text-black">Guardar federación</button>
        </div>
      </div>
    </div>
  );
}
