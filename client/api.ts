// Lightweight frontend API helper for Sabi Wallet backend
// Framework-agnostic (works with Vite, Next.js, CRA)

export type CreateWalletRequest = {
  device_id: string;
  phone: string; // +234xxxxxxxxxx
};

export type CreateWalletResponse = {
  wallet_id: string;
  invite_code: string;
  node_id: string;
};

// Resolve API base URL from common frontend env conventions
// Priority: Vite -> Next.js -> CRA -> fallback localhost
function resolveBaseUrl(): string {
  // Vite (import.meta.env)
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  const viteEnv = (typeof import.meta !== 'undefined' && (import.meta as any).env) || undefined;
  const viteBase = viteEnv?.VITE_API_BASE_URL as string | undefined;

  // Next.js / CRA (process.env.*)
  // eslint-disable-next-line no-undef
  const nextBase = typeof process !== 'undefined' ? (process.env.NEXT_PUBLIC_API_BASE_URL || process.env.REACT_APP_API_BASE_URL) : undefined;

  return (
    viteBase ||
    (typeof nextBase === 'string' && nextBase) ||
    'http://localhost:3000'
  );
}

const API_BASE = resolveBaseUrl();

async function handleJson<T>(res: Response): Promise<T> {
  const contentType = res.headers.get('content-type') || '';
  const isJson = contentType.includes('application/json');
  if (!res.ok) {
    if (isJson) {
      const err = await res.json().catch(() => ({}));
      const message = (err && (err.error || err.message)) || `HTTP ${res.status}`;
      throw new Error(message);
    }
    throw new Error(`HTTP ${res.status}`);
  }
  if (!isJson) {
    // @ts-expect-error allow non-json in rare cases
    return undefined;
  }
  return (await res.json()) as T;
}

export async function createWallet(
  body: CreateWalletRequest,
  opts?: { signal?: AbortSignal }
): Promise<CreateWalletResponse> {
  const url = `${API_BASE}/api/v1/wallets/create`;
  const res = await fetch(url, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(body),
    signal: opts?.signal,
  });
  return handleJson<CreateWalletResponse>(res);
}

// Example usage:
// const resp = await createWallet({ device_id: 'my-device', phone: '+2348012345678' });
// console.log(resp.wallet_id, resp.invite_code, resp.node_id);
