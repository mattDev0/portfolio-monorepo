const LOCAL_HOSTS = new Set(['localhost', '127.0.0.1', '::1', '0.0.0.0']);

const truthyFlags = new Set(['1', 'true', 'yes', 'local', 'dev', 'development']);
const falsyFlags = new Set(['0', 'false', 'no', 'prod', 'production']);
const viteEnv = import.meta.env || {};

const normalizeFlag = (value) => String(value ?? '').trim().toLowerCase();

const localFlag = normalizeFlag(viteEnv.VITE_LOCAL_DEV);
const browserHostname = typeof window === 'undefined' ? '' : window.location.hostname;
const runningOnLocalHost = LOCAL_HOSTS.has(browserHostname);

export const isLocalRuntime = truthyFlags.has(localFlag)
  || (!falsyFlags.has(localFlag) && (viteEnv.DEV || runningOnLocalHost));

const trimTrailingSlash = (value) => value.replace(/\/$/, '');

const productionApiBaseUrl = trimTrailingSlash(
  viteEnv.VITE_PRODUCTION_API_BASE_URL || 'https://mattdev0.tech'
);

const localApiBaseUrls = {
  rust: trimTrailingSlash(viteEnv.VITE_LOCAL_RUST_API_BASE_URL || 'http://localhost:8080'),
  java: trimTrailingSlash(viteEnv.VITE_LOCAL_JAVA_API_BASE_URL || 'http://localhost:8081'),
};

const productionApiBaseUrls = {
  rust: productionApiBaseUrl,
  java: productionApiBaseUrl,
};

const apiBaseUrls = isLocalRuntime ? localApiBaseUrls : productionApiBaseUrls;

export const apiUrl = (service, path) => {
  const normalizedPath = path.startsWith('/') ? path : `/${path}`;
  return `${apiBaseUrls[service]}${normalizedPath}`;
};
