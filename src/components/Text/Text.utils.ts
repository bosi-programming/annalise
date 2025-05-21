import type { TextProps } from "./Text";

export function getSizeClass(size: TextProps['size']) {
  if(size) {
    return `text-${size}`
  }
  return 'text-big'
}
