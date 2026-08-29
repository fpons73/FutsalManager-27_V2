import { useEffect, useState } from "react";
import { api, type TrainingRow, type ProgressRow, type StaffImpact, type TacticalAutomation } from "../../api";

const DAYS = ["Lunes","Martes","Miércoles","Jueves","Viernes","Sábado","Domingo"];

export default function TrainingView() {
  const [schedule, setSchedule] = useState<TrainingRow[]>([]);
  const [types, setTypes] = useState<[number,string,string,number][]>([]);
  const [progress, setProgress] = useState<ProgressRow[]>([]);
  const [saving, setSaving] = useState(false);
  const [msg, setMsg] = useState<string|null>(null);
  const [staff, setStaff] = useState<StaffImpact|null>(null);
  const [automations, setAutomations] = useState<TacticalAutomation[]>([]);
  const [autoName, setAutoName] = useState("Reacción final");

  const load = async () => {
    const [s, t, p, impact, autos] = await Promise.all([api.getTrainingSchedule(), api.getTrainingTypes(), api.getTrainingProgress(), api.getStaffImpact(), api.getTacticalAutomations()]);
    setSchedule(s);
    setTypes(t);
    setProgress(p);
    setStaff(impact);
    setAutomations(autos);
  };
  useEffect(()=>{ load(); },[]);

  const updateDay = (day:number, typeId:number) => {
    const intensity = types.find((x)=>x[0]===typeId)?.[3] ?? 70;
    setSchedule((prev)=>{
      const existing = prev.find((x)=>x.day===day);
      if (existing) return prev.map((x)=> x.day===day ? {...x, type_id:typeId, type_name: types.find((y)=>y[0]===typeId)?.[1] ?? "", category: types.find((y)=>y[0]===typeId)?.[2] ?? "", intensity} : x);
      return [...prev, { day, type_id: typeId, type_name: types.find((y)=>y[0]===typeId)?.[1] ?? "", category: types.find((y)=>y[0]===typeId)?.[2] ?? "", intensity }];
    });
  };

  const save = async () => {
    setSaving(true);
    try {
      const payload: [number,number,number][] = schedule.map((s)=>[s.day, s.type_id, s.intensity]);
      const res = await api.setTrainingSchedule(payload);
      setMsg(res);
      await load();
    } catch(e){ setMsg(String(e)); }
    finally{ setSaving(false); }
  };

  return (
    <div className="mx-auto max-w-6xl space-y-6 p-6">
      <div className="flex items-center justify-between">
        <h2 className="text-xl font-black">Entrenamientos</h2>
        <button onClick={save} disabled={saving} className="rounded-lg bg-fm-accent px-4 py-2 text-sm font-bold text-black disabled:opacity-50">{saving?"Guardando…":"Guardar"}</button>
      </div>
      {msg && <div className="rounded-lg bg-fm-accent/10 px-3 py-2 text-sm">{msg}</div>}
      {staff && <div className="grid gap-2 rounded-xl border border-fm-border bg-fm-panel p-3 text-xs sm:grid-cols-4"><div><span className="text-fm-dim">Staff técnico</span><br/><b>{staff.coach}/20</b> · +{staff.training_bonus}% entrenamiento</div><div><span className="text-fm-dim">Asistente</span><br/><b>{staff.assistant}/20</b> gestión</div><div><span className="text-fm-dim">Preparadores jóvenes</span><br/><b>{staff.youth}/20</b> · desarrollo cantera</div><div><span className="text-fm-dim">Fisioterapia</span><br/><b>{staff.physio}/20</b> · -{staff.injury_reduction}% riesgo</div></div>}

      <div className="rounded-xl border border-fm-border bg-fm-panel p-4"><div className="mb-2 flex items-center justify-between"><div><h3 className="font-black">Automatismos tácticos</h3><p className="text-xs text-fm-dim">Se entrenan semanalmente y se activan en situaciones de partido.</p></div><input value={autoName} onChange={e=>setAutoName(e.target.value)} className="w-36 rounded border border-fm-border bg-fm-bg px-2 py-1 text-xs" aria-label="Nombre del automatismo" /></div><div className="flex flex-wrap gap-2"><button onClick={async()=>{await api.setTacticalAutomation({name:autoName,triggerType:"losing_late",threshold:70,formation:"5-0",tempo:75,pressing:70,defensiveLine:65,width:60,enabled:true}); await load(); setMsg("Automatismo guardado");}} className="rounded-lg bg-fm-accent px-3 py-1.5 text-xs font-bold text-black">Guardar reacción</button>{automations.map(a=><span key={a.id} className="rounded-full border border-fm-border px-2 py-1 text-xs">{a.name} · {a.formation} · nivel {a.training_level}</span>)}</div></div>\n\n      <div className="grid gap-2 sm:grid-cols-2 lg:grid-cols-4">
        {[0,1,2,3,4,5,6].map((d)=>{
          const cur = schedule.find((s)=>s.day===d);
          const isMatch = d===5;
          const isRest = d===6;
          return (
            <div key={d} className={`rounded-xl border p-3 ${isMatch ? "border-sky-500/30 bg-sky-500/10" : isRest ? "border-fm-border bg-fm-bg opacity-70" : "border-fm-border bg-fm-panel"}`}>
              <div className="mb-2 text-xs font-bold uppercase tracking-widest text-fm-dim">{DAYS[d]} {isMatch && "· PARTIDO"} {isRest && "· DESCANSO"}</div>
              {isMatch || isRest ? (
                <div className="text-sm text-fm-dim">{isMatch ? "Partido oficial" : "Recuperación"}</div>
              ) : (
                <select value={cur?.type_id ?? 1} onChange={(e)=>updateDay(d, Number(e.target.value))} className="w-full rounded-lg border border-fm-border bg-fm-bg px-2 py-1.5 text-sm">
                  {types.map(([id,name])=> <option key={id} value={id}>{name}</option>)}
                </select>
              )}
              {cur && !isMatch && !isRest && <div className="mt-2 text-xs text-fm-dim">Intensidad {cur.intensity}% · {cur.category}</div>}
            </div>
          );
        })}
      </div>

      <div className="overflow-hidden rounded-xl border border-fm-border bg-fm-panel">
        <div className="border-b border-fm-border bg-fm-bg px-3 py-2 text-xs font-bold uppercase tracking-widest text-fm-dim">Progreso semanal · {progress.length} jugadores</div>
        <div className="overflow-x-auto">
          <table className="w-full text-sm">
            <thead className="bg-fm-bg text-xs uppercase tracking-widest text-fm-dim"><tr><th className="px-3 py-2 text-left">Jugador</th><th className="px-2 py-2">Pos</th><th className="px-2 py-2">Edad</th><th className="px-2 py-2">CA</th><th className="px-2 py-2">PA</th><th className="px-2 py-2">Gap</th><th className="px-2 py-2">Proy.</th></tr></thead>
            <tbody>
              {progress.map((p)=>(
                <tr key={p.player_id} className="border-t border-fm-border hover:bg-fm-panel2">
                  <td className="px-3 py-2 font-semibold">{p.name}</td>
                  <td className="px-2 py-2 text-center"><span className="rounded bg-fm-bg px-1.5 py-0.5 text-xs font-bold">{p.position}</span></td>
                  <td className="px-2 py-2 text-center">{p.age}</td>
                  <td className="px-2 py-2 text-center font-mono font-bold">{p.ca}</td>
                  <td className="px-2 py-2 text-center font-mono text-fm-dim">{p.pa}</td>
                  <td className="px-2 py-2 text-center font-mono">{p.pa - p.ca}</td>
                  <td className="px-2 py-2 text-center font-mono text-emerald-400">+{p.improvement.toFixed(2)}</td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
        <div className="bg-fm-bg px-3 py-2 text-xs text-fm-dim">La progresión se aplica cada lunes al avanzar días. Jóvenes con alto potencial y profesionalidad mejoran más.</div>
      </div>
    </div>
  );
}
