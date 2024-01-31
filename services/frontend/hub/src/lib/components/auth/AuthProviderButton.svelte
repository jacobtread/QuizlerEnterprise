<script lang="ts">
	import type { ComponentType } from "svelte";

	export let icon: ComponentType;
	export let url: string;
	export let text: string;
</script>

<a class="button" href={url}>
	<svelte:component this={icon} />

	{text}
</a>

<style lang="scss">
	.button {
		display: flex;
		gap: 1rem;
		padding: 0.75rem 1rem;
		border: 1px solid #ccc;
		background-color: #f7f7f7;
		border-radius: 0.2rem;
		font-size: 1rem;
		text-align: left;
		width: 100%;
		cursor: pointer;
		color: #666;
		font-weight: normal;
		text-decoration: none;
	}
</style>
