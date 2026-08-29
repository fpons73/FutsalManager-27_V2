import type { FixtureRow } from "../../api";
import { StatusBadge } from "../ui";

export default function CupBracket({ rounds }: { rounds: Record<string, FixtureRow[]> }) {
  const names: Record<string, string> = { "1": "Primera ronda", "2": "Cuartos", "3": "Semifinal", "4": "Final" };
  return <div className="overflow-x-auto rounded-2xl border border-fm-border bg-fm-panel p-4"><div className="flex min-w-[760px] gap-4">
    {Object.entries(rounds).sort(([a],[b])=>Number(a)-Number(b)).map(([round, fixtures]) => <div key={round} className="w-52 shrink-0 space-y-3"><div className="text-xs font-black uppercase tracking-widest text-fm-accent">{names[round] ?? `Ronda ${round}`}</div>{fixtures.map((f) => <div key={f.id} className="rounded-xl border border-fm-border bg-fm-bg p-3 text-xs"><div className="flex justify-between"><span>{f.home_short}</span><strong>{f.status === "finished" ? f.home_score : "—"}</strong></div><div className="mt-1 flex justify-between"><span>{f.away_short}</span><strong>{f.status === "finished" ? f.away_score : "—"}</strong></div>{f.leg && f.leg > 1 && <div className="mt-2 text-[10px] text-fm-sky-300">Vuelta · Global {f.aggregate_home_score ?? 0}-{f.aggregate_away_score ?? 0}</div>}{f.went_to_penalties && <div className="mt-2"><StatusBadge tone="warning">Pen. {f.penalty_home_score}-{f.penalty_away_score}</StatusBadge></div>}{f.went_to_extra_time && !f.went_to_penalties && <div className="mt-2"><StatusBadge tone="warning">Prórroga</StatusBadge></div>}{f.cup_winner_id && <div className="mt-2 text-[10px] font-bold text-emerald-300">Clasificado ✓</div>}</div>)}</div>)}
  </div></div>;
}
