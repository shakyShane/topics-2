<script context="module" lang="ts">
	import { DATA_PATH, KINDS } from '../../utils/types';
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

<template>
	{#each items as item, i}
		{item.kind}
	{/each}
</template>
