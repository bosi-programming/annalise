import type { ReactNode } from "react"

export interface TextProps {
  children: ReactNode
  size?: 'big' | 'medium' | 'small' | 'details' | 'terms' | 'h1' | 'h2' | 'h3'
  weight?: 'normal' | 'bold'
  className?: string
  as?: 'p' | 'h1' | 'h2' | 'h3' | 'h4' | 'h5' | 'h6' | 'span';
}

export function Text({ children, size = 'big', weight = 'normal', as = 'p', className }: TextProps) {
  const sizeClass = `text-${size}`
  const weightClass = `font-${weight}`
  const finalClass = `${sizeClass} ${weightClass} ${className}`
  const Component = as;

  return (
    <Component className={finalClass}>{children}</Component>
  )
}

