<script lang="ts">
	import Header from "$lib/components/Header.svelte";
	import Sidebar from "$lib/components/Sidebar.svelte";
</script>

<div class="outer">
	<Header />

	<div class="container">
		<Sidebar />

		<main class="main">
			<slot />
		</main>
	</div>
</div>

<style lang="scss">
	.outer {
		display: flex;
		flex-flow: column;
		width: 100%;
		height: 100%;
	}

	.container {
		display: flex;
		flex-flow: row;
		flex: auto;
	}

	.main {
		flex: auto;
		overflow: auto;
	}
</style>
