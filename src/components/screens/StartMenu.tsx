import { useEffect, useState } from "react";
import { api } from "../../api";
import { useStore } from "../../store";
import NewGame from "./NewGame";
import SavesView from "./SavesView";

type MenuTarget = "home" | "newgame" | "saves" | "editor";

function MenuGlyph({ children }: { children: string }) {
  return <span className="menu-glyph" aria-hidden="true">{children}</span>;
}

export default function StartMenu() {
  const { setScreen } = useStore();
  const [hasSave, setHasSave] = useState(false);
  const [panel, setPanel] = useState<"menu" | "newgame" | "saves">("menu");

  useEffect(() => {
    api.listSaveSlots().then((slots) => setHasSave(slots.length > 0)).catch(() => setHasSave(false));
  }, []);

  const navigate = (target: MenuTarget) => {
    setScreen(target);
    if (target === "newgame" || target === "saves") setPanel(target);
  };

  if (panel === "newgame") return <NewGame />;
  if (panel === "saves") return <SavesView />;

  return <main className="start-menu relative min-h-screen overflow-hidden text-white">
    <div className="start-menu-bg absolute inset-0" aria-hidden="true" />
    <div className="start-menu-overlay absolute inset-0" aria-hidden="true" />
    <div className="relative z-10 flex min-h-screen flex-col justify-between p-6 md:p-10">
      <header className="flex w-full items-start justify-between gap-6">
        <div className="start-logo drop-shadow-2xl">
          <h1 className="font-logo text-5xl font-black italic leading-[.82] tracking-tight text-white md:text-6xl">FUTSAL<br /><span className="text-4xl text-cyan-300 md:text-5xl">MANAGER <span className="text-white">27</span></span></h1>
        </div>
        <div className="flex flex-col items-end gap-2">
          <div className="text-sm font-semibold text-white/80 drop-shadow-md">v1.0.3</div>
          <div className="profile-badge flex items-center gap-3 rounded-full px-4 py-2">
            <div className="profile-avatar"><MenuGlyph>●</MenuGlyph></div>
            <div className="text-xs font-bold tracking-wide"><div>JUGADOR: ALEX CHEN</div><div className="text-white/75">EQUIPO: MAN CITY FUTSAL</div></div>
          </div>
        </div>
      </header>

      <section className="glass-menu-container flex w-full max-w-md flex-col gap-3 p-5 md:gap-4 md:p-6" aria-label="Menú principal">
        <button type="button" onClick={() => navigate("saves")} className="menu-btn menu-btn-active group">
          <span><strong>CONTINUAR PARTIDA</strong><small>{hasSave ? "Retomar la última carrera guardada" : "No hay una partida guardada todavía"}</small></span><MenuGlyph>▷</MenuGlyph>
        </button>
        <button type="button" onClick={() => navigate("newgame")} className="menu-btn group">
          <span><strong>NUEVA PARTIDA</strong><small>Nuevo equipo</small></span><MenuGlyph>♟</MenuGlyph>
        </button>
        <button type="button" onClick={() => navigate("saves")} className="menu-btn group">
          <span><strong>CARGAR PARTIDA</strong></span><MenuGlyph>▰</MenuGlyph>
        </button>
        <button type="button" onClick={() => alert("Los ajustes estarán disponibles próximamente.")} className="menu-btn group">
          <span><strong>AJUSTES</strong></span><MenuGlyph>⚙</MenuGlyph>
        </button>
        <button type="button" onClick={() => navigate("editor")} className="menu-btn menu-btn-editor group">
          <span><strong>EDITOR</strong></span><MenuGlyph>⚒</MenuGlyph>
        </button>
        <button type="button" onClick={() => window.close()} className="menu-btn menu-btn-exit group">
          <span><strong>SALIR</strong></span><MenuGlyph>⏻</MenuGlyph>
        </button>
        <div className="mt-2 flex gap-2 md:mt-3">
          <button type="button" className="menu-secondary">Créditos</button>
          <button type="button" className="menu-secondary">Acerca de</button>
          <button type="button" className="menu-secondary leading-tight">Licencias y<br />Marcas</button>
        </div>
      </section>

      <footer className="text-center text-xs text-white/70 drop-shadow-md">La pasión, la estrategia y la pista están en tus manos.</footer>
    </div>
  </main>;
}
