<!-- Component for requiring authentication before displaying the slot -->
<script lang="ts">
	import { goto } from "$app/navigation";
	import { loadUser, user } from "$lib/stores/auth";
	import { onMount } from "svelte";
	import Loader from "./Loader.svelte";

	let loading = true;

	onMount(async () => {
		loading = true;
		const activeUser = await loadUser();
		loading = false;

		if (activeUser == null) {
			await goto("/auth/login");
		}
	});
</script>

{#if !loading && $user}
	<slot />
{:else}
	<Loader />
{/if}
