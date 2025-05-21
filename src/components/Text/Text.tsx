import type { ReactNode } from "react"
import { getSizeClass } from "./Text.utils"

export interface TextProps {
  children: ReactNode
  size?: 'big' | 'medium' | 'small' | 'details' | 'terms' | 'h1' | 'h2' | 'h3'
  weight?: 'regular' | 'bold'
  className?: string
}

export function Text({ children, size = 'big', weight = 'bold', className }: TextProps) {
  const sizeClass = getSizeClass(size)
  const finalClass = `${sizeClass} ${className}`

  return (
    <p className={finalClass}>{children}</p>
  )
}

