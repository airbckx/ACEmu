/**
 * Admin sign-in state: the JWT from POST /api/auth/login, kept in
 * localStorage so a browser (or the party TV) stays signed in until the
 * token expires. `api.ts` attaches it to every request and calls
 * `handleUnauthorized` on a 401.
 */

const TOKEN_KEY = 'acemu-admin.token';
const USER_KEY = 'acemu-admin.user';

export function getToken(): string | null {
  try {
    return window.localStorage.getItem(TOKEN_KEY);
  } catch {
    return null;
  }
}

export function getUsername(): string | null {
  try {
    return window.localStorage.getItem(USER_KEY);
  } catch {
    return null;
  }
}

export function setSession(token: string, username: string): void {
  try {
    window.localStorage.setItem(TOKEN_KEY, token);
    window.localStorage.setItem(USER_KEY, username);
  } catch {
    // storage unavailable: the session lasts for this page only
    memoryToken = token;
  }
}

let memoryToken: string | null = null;

export function clearSession(): void {
  memoryToken = null;
  try {
    window.localStorage.removeItem(TOKEN_KEY);
    window.localStorage.removeItem(USER_KEY);
  } catch {
    // nothing to clear
  }
}

export function currentToken(): string | null {
  return getToken() ?? memoryToken;
}

export function isLoginPage(): boolean {
  return typeof window !== 'undefined' && window.location.pathname.startsWith('/login');
}

/** The page to go to for signing in, remembering where the user was. */
export function loginHref(): string {
  if (typeof window === 'undefined') return '/login/';
  const here = window.location.pathname + window.location.search;
  return `/login/?next=${encodeURIComponent(here)}`;
}

/** Drops the session and sends the browser to the login page (once). */
export function handleUnauthorized(): void {
  if (typeof window === 'undefined' || isLoginPage()) return;
  clearSession();
  window.location.assign(loginHref());
}
