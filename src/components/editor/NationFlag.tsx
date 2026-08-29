export default function NationFlag({ path, name, small = false }: { path?: string | null; name: string; small?: boolean }) {
  if (!path) return <span className={small ? "text-xs" : "text-sm"} title={name}>🏳️</span>;
  return <img src={path.startsWith("http") || path.startsWith("/") ? path : `asset://${path}`} alt={name} title={name} className={small ? "inline-block h-3 w-5 rounded object-cover" : "inline-block h-5 w-7 rounded object-cover"} />;
}
