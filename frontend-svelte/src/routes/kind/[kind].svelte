<script context="module" lang="ts">
	import { DATA_PATH, KINDS } from '../../utils/types';
	/** @type {IrItem[]} */
	let items;
	export async function load({ page, fetch, status, error, session, context }) {
		const kind = page.params.kind;
		const valid = KINDS.includes(kind.toLowerCase());
		items = context.data.items;
		return {
			props: {
				valid
			}
		};
	}
</script>
<script>
	/** @type {IrItem[]} */
	export let items
</script>

<template>
	<ul>
		{#each items as item}
			<li>
				<p>
					{}
				</p>
			</li>
		{/each}
	</ul>
</template>
