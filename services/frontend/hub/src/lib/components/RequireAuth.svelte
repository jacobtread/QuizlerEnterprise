<!-- Component for requiring authentication before displaying the slot -->
<script lang="ts">
	import { goto } from "$app/navigation";
	import { loadUser, user } from "$lib/stores/auth";
	import { onMount } from "svelte";
	import Loader from "./Loader.svelte";

	const enum State {
		Loading = 0,
		Loaded = 1
	}

	let state = State.Loading;

	onMount(async () => {
		state = State.Loading;

		let loaded = await loadUser();

		if (!loaded) {
			goto("/auth/login");
		} else {
			state = State.Loaded;
		}
	});
</script>

{#if state === State.Loading}
	<Loader />
{:else if state === State.Loaded && $user}
	<slot />
{/if}
