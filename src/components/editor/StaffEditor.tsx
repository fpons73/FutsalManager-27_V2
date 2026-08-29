import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import ImagePicker from "./ImagePicker";
import Flag from "../Flag";

function Num({ label, v, onChange }: { label:string; v:number; onChange:(n:number)=>void }) {
  return (
    <label className="flex flex-col">
      <span className="text-[10px] uppercase tracking-wider text-fm-dim">{label}</span>
      <input type="number" min={1} max={20} value={v} onChange={(e)=>onChange(Number(e.target.value))} className="w-16 rounded border border-fm-border bg-fm-bg px-1.5 py-1 font-mono text-sm" />
    </label>
  );
}

const ROLES = ["coach","assistant","scout","physio","fitness_coach","goalkeeper_coach","technical_coach","analyst"];

export default function StaffEditor({ staff, nations, clubs, onClose }: { staff:any; nations:any[]; clubs:any[]; onClose:()=>void }) {
  const [first, setFirst] = useState(staff.first_name ?? "");
  const [last, setLast] = useState(staff.last_name ?? "");
  const [nationId, setNationId] = useState(staff.nation_id ?? 1);
  const [role, setRole] = useState(staff.role ?? "assistant");
  const [clubId, setClubId] = useState<string>(staff.club_id ? String(staff.club_id) : "");
  const [wage, setWage] = useState(staff.wage_weekly ?? 600);
  const [a, setA] = useState({ tactical: staff.tactical ?? 10, manManagement: staff.man_management ?? 10, judging: staff.judging ?? 10, motivating: staff.motivating ?? 10, workingYoungsters: staff.working_youngsters ?? 10, physioLevel: staff.physio_level ?? 10 });
  const [msg, setMsg] = useState<string | null>(null);

  const save = async () => {
    try {
      await invoke("editor_update_staff", { id: staff.id, first, last, nationId, role, clubId: clubId ? Number(clubId) : null, tactical: a.tactical, manManagement: a.manManagement, judging: a.judging, motivating: a.motivating, workingYoungsters: a.workingYoungsters, physioLevel: a.physioLevel, wageWeekly: wage });
      setMsg("Guardado");
    } catch (e) { setMsg(String(e)); }
  };

  return (
    <div className="rounded-xl border border-sky-500/30 bg-fm-panel p-4">
      <div className="mb-3 flex items-center justify-between">
        <h3 className="text-lg font-black"><Flag src={staff.flag_path} alt={staff.nation} className="mr-2 inline h-4 w-6" />Staff · {staff.first_name} {staff.last_name} <span className="text-fm-dim">(ID {staff.id})</span></h3>
        <button onClick={onClose} className="rounded-lg border border-fm-border px-3 py-1 text-sm text-fm-dim hover:text-white">Cerrar</button>
      </div>
      {msg && <div className="mb-3 rounded-lg border border-amber-500/30 bg-amber-500/10 px-3 py-2 text-sm">{msg}</div>}

      <div className="grid gap-4 lg:grid-cols-3">
        <div className="space-y-3 rounded-lg border border-fm-border bg-fm-bg p-3">
          <ImagePicker command="editor_set_staff_photo" entityId={staff.id} label="Foto" value={staff.photo_path ?? null} prefix="S" />
          <div className="space-y-2 text-sm">
            <div><label className="block text-[10px] uppercase tracking-wider text-fm-dim">Nombre</label><input value={first} onChange={(e)=>setFirst(e.target.value)} className="w-full rounded border border-fm-border bg-fm-bg px-2 py-1" /></div>
            <div><label className="block text-[10px] uppercase tracking-wider text-fm-dim">Apellidos</label><input value={last} onChange={(e)=>setLast(e.target.value)} className="w-full rounded border border-fm-border bg-fm-bg px-2 py-1" /></div>
            <div><label className="block text-[10px] uppercase tracking-wider text-fm-dim">Nacionalidad</label>
              <select value={nationId} onChange={(e)=>setNationId(Number(e.target.value))} className="w-full rounded border border-fm-border bg-fm-bg px-2 py-1">
                {nations.map((n:any)=><option key={n.id} value={n.id}>{n.name}</option>)}
              </select>
            </div>
            <div><label className="block text-[10px] uppercase tracking-wider text-fm-dim">Rol</label>
              <select value={role} onChange={(e)=>setRole(e.target.value)} className="w-full rounded border border-fm-border bg-fm-bg px-2 py-1">
                {ROLES.map((r)=> <option key={r} value={r}>{r}</option>)}
              </select>
            </div>
            <div><label className="block text-[10px] uppercase tracking-wider text-fm-dim">Club</label>
              <select value={clubId} onChange={(e)=>setClubId(e.target.value)} className="w-full rounded border border-fm-border bg-fm-bg px-2 py-1">
                <option value="">Libre</option>
                {clubs.map((c:any)=><option key={c.id} value={c.id}>{c.name}</option>)}
              </select>
            </div>
            <div><label className="block text-[10px] uppercase tracking-wider text-fm-dim">Salario/semana €</label><input type="number" value={wage} onChange={(e)=>setWage(Number(e.target.value))} className="w-full rounded border border-fm-border bg-fm-bg px-2 py-1" /></div>
            <button onClick={save} className="mt-1 w-full rounded bg-fm-accent px-2 py-1.5 text-sm font-bold text-black">Guardar</button>
          </div>
        </div>

        <div className="lg:col-span-2 rounded-lg border border-fm-border bg-fm-bg p-3">
          <div className="mb-3 text-xs font-bold uppercase tracking-widest text-fm-dim">Atributos</div>
          <div className="grid grid-cols-3 gap-2 sm:grid-cols-6">
            <Num label="Táctica" v={a.tactical} onChange={(n)=>setA({...a,tactical:n})} />
            <Num label="Gestión" v={a.manManagement} onChange={(n)=>setA({...a,manManagement:n})} />
            <Num label="Ojeo" v={a.judging} onChange={(n)=>setA({...a,judging:n})} />
            <Num label="Motiv." v={a.motivating} onChange={(n)=>setA({...a,motivating:n})} />
            <Num label="Jóvenes" v={a.workingYoungsters} onChange={(n)=>setA({...a,workingYoungsters:n})} />
            <Num label="Fisio" v={a.physioLevel} onChange={(n)=>setA({...a,physioLevel:n})} />
          </div>
          <button onClick={save} className="mt-4 rounded bg-fm-accent px-4 py-1.5 text-sm font-bold text-black">Guardar</button>
        </div>
      </div>
    </div>
  );
}
