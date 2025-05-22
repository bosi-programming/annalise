import { describe, expect, test } from 'vitest'
import { render } from 'vitest-browser-react'
import { Text } from './Text'

describe('Text', () => {
  test('Renders children', async () => {
    const { getByText } = render(<Text>Test</Text>)

    const element = getByText('Test')
    await expect.element(element).toBeInTheDocument()
    await expect.element(element).toHaveClass('text-big')
    await expect.element(element).toHaveClass('font-normal')
  })
  test('Renders children', async () => {
    const { getByText } = render(<Text>Test</Text>)

    await expect.element(getByText('Test')).toBeInTheDocument()
  })
})
