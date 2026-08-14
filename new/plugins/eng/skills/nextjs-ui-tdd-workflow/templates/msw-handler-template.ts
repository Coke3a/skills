import { http, HttpResponse } from 'msw'

export const exampleHandlers = [
  http.get('/api/example-items', () => {
    return HttpResponse.json({
      items: getExampleItems(),
    })
  }),
]

function getExampleItems() {
  return [
    {
      id: 'example-item-1',
      name: 'Example Item',
    },
  ]
}
