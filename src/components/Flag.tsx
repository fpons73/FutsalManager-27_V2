export default function Flag({ src, alt, className = "h-3 w-5" }: { src?: string | null; alt: string; className?: string }) {
  return src ? <img src={src} alt={alt} title={alt} className={`${className} rounded object-cover`} onError={(e) => { e.currentTarget.style.display = "none"; }} /> : <span role="img" aria-label={alt} title={alt} className="inline-block">🏳️</span>;
}
