<script lang="ts">
	import RequireAuth from "$lib/components/RequireAuth.svelte";
</script>

<RequireAuth>
	<main class="main">
		<slot />
	</main>
</RequireAuth>

<style lang="scss">
</style>
