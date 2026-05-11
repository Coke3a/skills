import { renderHook, act } from '@testing-library/react'
import { useExampleFeature } from './useExampleFeature'

describe('useExampleFeature', () => {
  it('updates selected item when requested', () => {
    const { result } = renderHook(() => useExampleFeature())

    act(() => {
      result.current.selectItem('example-item')
    })

    expect(result.current.selectedItem).toBe('example-item')
  })
})
