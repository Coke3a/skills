type ApiResult<T> =
  | { ok: true; data: T }
  | { ok: false; status: number; message: string };

export async function apiGet<T>(
  path: string,
  init?: RequestInit,
): Promise<ApiResult<T>> {
  const response = await fetch(`${process.env.API_BASE_URL}${path}`, {
    ...init,
    headers: {
      Accept: "application/json",
      ...init?.headers,
    },
  });

  if (!response.ok) {
    return {
      ok: false,
      status: response.status,
      message: "Request failed.",
    };
  }

  return {
    ok: true,
    data: (await response.json()) as T,
  };
}
