import { describe, expect, it, beforeEach } from 'vitest';
import { errors } from './errors.svelte';

describe('errors store', () => {
  beforeEach(() => {
    errors.clear();
  });

  it('pushes a toast with kind-specific copy', () => {
    errors.push('geo_blocked', 'country fence');
    expect(errors.items).toHaveLength(1);
    const e = errors.items[0];
    expect(e.kind).toBe('geo_blocked');
    expect(e.title).toMatch(/geo-blocked/i);
    expect(e.suggestion).toMatch(/VPN|cookie/i);
  });

  it('falls back to unknown copy for unexpected kind', () => {
    // @ts-expect-error intentionally bad kind to exercise fallback
    errors.push('nonsense', 'x');
    const e = errors.items[0];
    expect(e.title).toMatch(/yt-dlp|error/i);
  });

  it('dismiss removes by id', () => {
    errors.push('network', 'no net');
    const id = errors.items[0].id;
    errors.dismiss(id);
    expect(errors.items).toHaveLength(0);
  });

  it('stores downloadId when provided', () => {
    errors.push('auth_required', 'sign in', 'dl-42');
    expect(errors.items[0].downloadId).toBe('dl-42');
  });
});
