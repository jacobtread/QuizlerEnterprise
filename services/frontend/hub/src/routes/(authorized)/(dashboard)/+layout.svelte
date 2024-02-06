<script lang="ts">
	import { base } from "$app/paths";
	import { clearAuthToken } from "$lib/stores/auth";

	function logout() {
		clearAuthToken();
	}
</script>

<div class="outer">
	<header class="header">
		<div class="header__left"></div>

		<div class="header__right">
			<a href="{base}/create">Create</a>

			<div>
				Avatar
				<img src="" alt="" />
			</div>
		</div>
	</header>
	<div class="container">
		<aside class="sidebar">
			<nav class="nav">
				<a href="{base}/">Home</a>
				<a href="{base}/browse">Browse</a>
				<a href="{base}/library">Library</a>
				<a href="{base}/reports">Reports</a>
				<a href="{base}/auth/login" on:click={logout}>Logout</a>
			</nav>
		</aside>

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

	.header {
		height: 3rem;
		background-color: #ccc;
	}

	.container {
		display: flex;
		flex-flow: row;
		flex: auto;
	}

	.sidebar {
		background-color: #eee;
		overflow: auto;

		display: flex;
		flex-flow: column;
		padding: 1rem;
		width: 100%;
		max-width: 16rem;
	}

	.nav {
		display: flex;
		flex-flow: column;
		gap: 1rem;
	}

	.main {
		flex: auto;
		overflow: auto;
	}
</style>
