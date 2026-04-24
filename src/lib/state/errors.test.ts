import { describe, expect, it, beforeEach } from 'vitest';
import { errors } from './errors.svelte';

describe('errors store', () => {
  beforeEach(() => {
    errors.clear();
  });

  it('pushes a toast and stores the kind discriminator', () => {
    errors.push('geo_blocked', 'country fence');
    expect(errors.items).toHaveLength(1);
    const e = errors.items[0];
    expect(e.kind).toBe('geo_blocked');
    expect(e.message).toBe('country fence');
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

  it('clear empties the queue', () => {
    errors.push('not_found', 'gone');
    errors.push('shell', 'exec');
    errors.clear();
    expect(errors.items).toHaveLength(0);
  });
});
