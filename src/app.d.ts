// See https://kit.svelte.dev/docs/types#app

import type { ProjectCache, type Config } from "$lib/types";

// for information about these interfaces
declare global {
	namespace App {
		// interface Error {}
		// interface Locals {}
		interface PageData {
      config: Config,
      projectCache: ProjectCache
    }
		// interface Platform {}
	}
}

export {};
