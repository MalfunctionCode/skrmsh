const icons = import.meta.glob('/src/lib/assets/heroes/icons/*.png', {
  eager: true,
  query: '?url',
  import: 'default'
}) as Record<string, string>;

export function heroIcon(slug: string): string | undefined {
  return icons[`/src/lib/assets/heroes/icons/${slug}.png`]
}
