<script lang="ts">
  import { app } from '@tauri-apps/api';
  import { appDataDir } from '@tauri-apps/api/path';
  import { Stronghold, Client } from '@tauri-apps/plugin-stronghold';
  import { writable, type Writable } from 'svelte/store';
  // import { invoke } from "@tauri-apps/api/core";
  let password = '';
  let secret = '';
  const createVaultPath = async () => {
    return `${await appDataDir()}/stronghold/vault.hold`;
  };
  const KEY = 'key';
  let client: Client | null = null;
  let stronghold: Stronghold | null = null;
  const submitPassword = async () => {
    const vaultPath = await createVaultPath();
    log(vaultPath);
    stronghold = stronghold || (await Stronghold.load(vaultPath, password));
    const clientName = await app.getName();
    try {
      log('client loading');
      client = await stronghold.loadClient(clientName);
    } catch (err) {
      log('client creating');
      client = await stronghold.createClient(clientName);
    }
    const store = client.getStore();
    log(`accessing key=${KEY}`);
    const data = await store.get(KEY);
    if (data) {
      const decoded = new TextDecoder().decode(new Uint8Array(data));
      log(`accessed key=${KEY} data=${decoded}`);
      secret = decoded;
      log('resting');
    } else {
      log('nothing stored');
      await saveSecret();
    }
  };
  const saveSecret = async () => {
    if (!client) return;
    const store = client.getStore();
    const encoded = Array.from(new TextEncoder().encode(secret));
    log(`inserting key=${KEY}`);
    await store.insert(KEY, encoded);
    log('saving stronghold');
    await stronghold!.save();
    log('resting');
  };
  type Log = {
    text: string;
    timestamp: Date;
  };
  const logs: Writable<Log[]> = writable([]);
  const log = (l: string) => {
    const d = new Date();
    logs.update(($logs) =>
      $logs.concat({
        text: l,
        timestamp: d,
      }),
    );
  };
  $: reversedLogs = $logs.slice(0).reverse();
</script>

<div class="container">
  <div class="flex flex-col">
    <a href="/">Reload</a>
    <form class="flex w-full" on:submit|preventDefault={submitPassword}>
      <input
        id="password"
        name="password"
        placeholder="password"
        bind:value={password}
      />
      <button type="submit">Submit</button>
    </form>
    <form class="flex w-full" on:submit|preventDefault={saveSecret}>
      <input
        id="secret"
        name="secret"
        placeholder="secret"
        bind:value={secret}
      />
      <button type="submit" disabled={!client}>Save</button>
    </form>
    <ol
      class="flex flex-col"
      style="text-decoration: none; font-family: monospace"
    >
      {#each reversedLogs as log}
        <li style="white-space: pre">
          {log.timestamp.toISOString()} - {log.text}
        </li>
      {/each}
    </ol>
  </div>
</div>

<style lang="postcss">
  .flex {
    display: flex;
  }
  .flex-col {
    flex-direction: column;
  }
  .w-full {
    width: 100%;
  }
  ol {
    list-style-type: none;
  }
</style>
