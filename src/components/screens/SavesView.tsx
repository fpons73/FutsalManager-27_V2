import { useEffect, useState } from "react";
import { api, type SaveSlot } from "../../api";
import { Panel, EmptyState, StatusBadge } from "../ui";

export default function SavesView() {
  const [slots, setSlots] = useState<SaveSlot[]>([]);
  const [loading, setLoading] = useState(true);
  const [message, setMessage] = useState("");
  const refresh = () => api.listSaveSlots().then(setSlots).finally(() => setLoading(false));
  useEffect(() => { refresh(); }, []);
  const save = async (id: number, autosave = false) => {
    try { autosave ? await api.autosaveGame() : await api.saveGame(id, `Partida ${id}`); setMessage("Partida guardada correctamente"); refresh(); }
    catch (e) { setMessage(String(e)); }
  };
  const load = async (id: number) => { if (!window.confirm("¿Cargar esta partida? Se perderán los cambios no guardados.")) return; try { await api.loadGame(id); window.location.reload(); } catch (e) { setMessage(String(e)); } };
  const backup = async () => { const destination = window.prompt("Ruta completa del backup SQLite"); if (!destination) return; try { await api.backupGame(destination); setMessage("Backup creado correctamente"); } catch (e) { setMessage(String(e)); } };
  return <div className="mx-auto max-w-5xl space-y-5 p-6">
    <div><h1 className="text-2xl font-black">Partidas guardadas</h1><p className="text-sm text-fm-dim">Gestiona tus slots sin sobrescribir la partida activa.</p></div>
    <Panel>
      <div><h2 className="mb-1 font-black">Guardado seguro</h2><p className="mb-3 text-sm text-fm-dim">El autoguardado conserva una copia independiente.</p></div><div className="flex flex-wrap gap-2"><button onClick={() => save(0, true)} className="rounded-lg bg-fm-accent px-4 py-2 text-sm font-bold text-black">Autoguardar ahora</button><button onClick={backup} className="rounded-lg border border-fm-border px-4 py-2 text-sm font-bold">Crear backup</button>{message && <span role="status" className="self-center text-sm text-fm-dim">{message}</span>}</div>
    </Panel>
    {loading ? <div className="p-8 text-center text-fm-dim" aria-live="polite">Cargando partidas…</div> : slots.length === 0 ? <EmptyState title="Sin partidas guardadas" description="Guarda tu primera partida en un slot independiente." /> : <div className="grid gap-3 sm:grid-cols-2">{slots.map(s => <Panel key={s.id}><h2 className="font-black">{s.name}</h2><p className="mb-3 text-xs text-fm-dim">Actualizada: {s.updated_at}</p><div className="flex items-center justify-between"><StatusBadge tone={s.is_autosave ? "default" : "success"}>{s.is_autosave ? "Autoguardado" : `Slot ${s.id}`}</StatusBadge><div className="flex gap-2">{!s.is_autosave && <button onClick={() => save(s.id)} className="rounded-lg border border-fm-border px-3 py-1.5 text-xs font-bold">Guardar aquí</button>}<button onClick={() => load(s.id)} className="rounded-lg bg-fm-accent px-3 py-1.5 text-xs font-bold text-black">Cargar</button></div></div></Panel>)}</div>}
  </div>;
}
