import { createExampleAction } from './actions'

describe('createExampleAction', () => {
  it('maps validation failure to a field error', async () => {
    const result = await createExampleAction({
      name: '',
    })

    expect(result).toEqual({
      ok: false,
      fieldErrors: {
        name: 'Name is required',
      },
    })
  })
})
