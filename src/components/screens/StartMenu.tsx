import { useEffect, useState } from "react";
import { api } from "../../api";
import { useStore } from "../../store";

export default function StartMenu() {
  const { setScreen } = useStore();
  const [hasSave, setHasSave] = useState(false);
  useEffect(() => { api.listSaveSlots().then(s => setHasSave(s.length > 0)).catch(() => {}); }, []);
  return <main className="start-menu relative min-h-screen overflow-hidden bg-[#071522] text-white">
    <div className="start-menu-bg absolute inset-0" aria-hidden="true" />
    <div className="relative z-10 mx-auto flex min-h-screen max-w-7xl flex-col p-5 sm:p-8">
      <header className="flex items-start justify-between gap-4">
        <div className="start-logo"><div className="text-4xl font-black italic leading-none sm:text-6xl">FUTSAL</div><div className="text-3xl font-black italic leading-none text-cyan-300 sm:text-5xl">MANAGER <span className="text-white">27</span></div></div>
        <div className="rounded-xl border border-white/20 bg-slate-950/50 px-4 py-2 text-right text-xs backdrop-blur"><div className="font-bold text-cyan-200">FUTSAL MANAGER 27</div><div className="text-white/60">v1.0.3</div></div>
      </header>
      <div className="flex flex-1 items-center py-8"><section className="w-full max-w-md rounded-2xl border border-cyan-300/50 bg-slate-950/60 p-3 shadow-2xl shadow-cyan-950/50 backdrop-blur-md">
        {hasSave && <button onClick={()=>setScreen("saves")} className="menu-action menu-primary"><span>CONTINUAR PARTIDA</span><b>▶</b><small>Continuar tu carrera guardada</small></button>}
        <button onClick={()=>setScreen("newgame")} className="menu-action"><span>NUEVA PARTIDA</span><b>✦</b><small>Comenzar una nueva carrera</small></button>
        {hasSave && <button onClick={()=>setScreen("saves")} className="menu-action"><span>CARGAR PARTIDA</span><b>▰</b><small>Gestionar partidas guardadas</small></button>}
        <button onClick={()=>setScreen("editor")} className="menu-action"><span>EDITOR</span><b>⚙</b><small>Editar la base de datos antes de jugar</small></button>
        <button onClick={()=>alert("Los ajustes estarán disponibles próximamente.")} className="menu-action"><span>AJUSTES</span><b>⚙</b><small>Preferencias de juego</small></button>
        <button onClick={()=>window.close()} className="menu-action"><span>SALIR</span><b>⏻</b><small>Cerrar Futsal Manager 27</small></button>
        <div className="mt-3 grid grid-cols-3 gap-2"><button className="menu-foot">Créditos</button><button className="menu-foot">Acerca de</button><button className="menu-foot">Licencias</button></div>
      </section></div>
      <footer className="text-center text-xs text-white/60">La pasión, la estrategia y la pista están en tus manos.</footer>
    </div>
  </main>;
}
