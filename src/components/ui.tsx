import type { ReactNode } from "react";

export function Panel({ children, className = "" }: { children: ReactNode; className?: string }) {
  return <section className={`rounded-2xl border border-fm-border bg-fm-panel shadow-xl shadow-slate-950/20 ${className}`}>{children}</section>;
}

export function MetricCard({ label, value, detail, tone = "text-fm-accent" }: { label: string; value: ReactNode; detail?: ReactNode; tone?: string }) {
  return <div className="rounded-xl border border-fm-border bg-fm-bg/70 p-3"><div className="text-[10px] font-bold uppercase tracking-[0.18em] text-fm-dim">{label}</div><div className={`mt-1 text-xl font-black ${tone}`}>{value}</div>{detail && <div className="mt-1 text-xs text-fm-dim">{detail}</div>}</div>;
}

export function StatusBadge({ children, tone = "default" }: { children: ReactNode; tone?: "default" | "success" | "warning" | "danger" }) {
  const colors = { default: "border-fm-border text-fm-dim", success: "border-emerald-400/30 bg-emerald-400/10 text-emerald-300", warning: "border-amber-400/30 bg-amber-400/10 text-amber-300", danger: "border-rose-400/30 bg-rose-400/10 text-rose-300" };
  return <span className={`inline-flex items-center rounded-full border px-2 py-0.5 text-[10px] font-bold uppercase tracking-wider ${colors[tone]}`}>{children}</span>;
}

export function Icon({ name, label }: { name: "home" | "users" | "calendar" | "trophy" | "market" | "search" | "finance" | "menu"; label?: string }) {
  const glyphs = { home: "⌂", users: "♙", calendar: "▦", trophy: "♛", market: "↔", search: "⌕", finance: "◈", menu: "☰" };
  return <span aria-hidden={label ? undefined : true} aria-label={label} title={label} className="inline-flex h-5 w-5 items-center justify-center text-sm font-black text-fm-accent">{glyphs[name]}</span>;
}

export function EmptyState({ title, description }: { title: string; description?: string }) {
  return <div className="rounded-xl border border-dashed border-fm-border bg-fm-bg/40 p-8 text-center"><div className="text-sm font-bold">{title}</div>{description && <div className="mt-1 text-xs text-fm-dim">{description}</div>}</div>;
}
