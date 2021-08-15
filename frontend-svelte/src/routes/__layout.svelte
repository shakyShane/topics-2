<script context="module" lang="ts">
	import { DATA_PATH, KINDS } from '../utils/types';

	export async function load({ page, fetch, status, error, session, context }) {
		const url = `http://${page.host}${DATA_PATH}`;
		const res = await fetch(url);

		if (res.ok) {
			return {
				context: {
					data: await res.json()
				}
			};
		}

		return {
			status: res.status,
			error: new Error(`Could not load ${url}`)
		};
	}
</script>

<header>
	<p>doc topics</p>
</header>
<slot />
<footer>
	<p>Do topics is by Shane Osbourne</p>
</footer>
