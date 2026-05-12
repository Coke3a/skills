import { render, screen } from '@testing-library/react'
import userEvent from '@testing-library/user-event'
import { ExampleButton } from './ExampleButton'

describe('ExampleButton', () => {
  it('calls the action when the user clicks the button', async () => {
    const user = userEvent.setup()
    const onAction = jest.fn()

    render(<ExampleButton onAction={onAction} />)

    await user.click(screen.getByRole('button', { name: /continue/i }))

    expect(onAction).toHaveBeenCalledTimes(1)
  })
})
