<script lang="ts">
	import Header from "$lib/components/Header.svelte";
	import Sidebar from "$lib/components/Sidebar.svelte";
</script>

<div class="outer">
	<Sidebar />

	<div class="container">
		<Header />

		<main class="main">
			<slot />
		</main>
	</div>
</div>

<style lang="scss">
	.outer {
		display: flex;
		width: 100%;
		height: 100%;
	}

	.container {
		display: flex;
		flex-flow: column;
		flex: auto;
		overflow: auto;
		width: 100%;
	}

	.main {
		flex: auto;
		overflow: auto;
		padding: 1rem;
	}
</style>
