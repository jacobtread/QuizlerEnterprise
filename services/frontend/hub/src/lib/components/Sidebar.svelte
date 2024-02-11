<script lang="ts">
	import { base } from "$app/paths";
	import { page } from "$app/stores";
	import Globe from "$lib/components/icons/Globe.svelte";
	import Home from "$lib/components/icons/Home.svelte";
	import Library from "$lib/components/icons/Library.svelte";
	import LineChart from "$lib/components/icons/LineChart.svelte";
	import Logout from "$lib/components/icons/Logout.svelte";
	import { type Page } from "@sveltejs/kit";

	import { clearAuthToken } from "$lib/stores/auth";

	function logout() {
		clearAuthToken();
	}

	function isActivePath(page: Page, path: string): boolean {
		return page.url.pathname === `${base}${path}`;
	}
</script>

<aside class="sidebar">
	<nav class="nav">
		<a href="{base}/" class:active={isActivePath($page, "/")}>
			<Home /> Home
		</a>
		<a href="{base}/browse" class:active={isActivePath($page, "/browse")}>
			<Globe /> Browse
		</a>
		<a href="{base}/library" class:active={isActivePath($page, "/library")}>
			<Library /> Library
		</a>
		<a href="{base}/reports" class:active={isActivePath($page, "/reports")}>
			<LineChart /> Reports
		</a>
		<a href="{base}/auth/login" on:click={logout}><Logout /> Logout</a>
	</nav>
</aside>

<style lang="scss">
	@use "../assets/theme.scss" as theme;

	.sidebar {
		overflow: auto;

		display: flex;
		flex-flow: column;
		padding: 1rem;
		width: 100%;
		max-width: 16rem;

		box-shadow: 0 0 10px rgba($color: #000000, $alpha: 0.5);
	}

	.nav {
		display: flex;
		flex-flow: column;
		gap: 1rem;
	}

	.nav > a {
		padding: 0.5rem;
		background-color: #eee;
		border-radius: 0.2rem;
		color: black;
		text-decoration: none;

		display: flex;
		align-items: center;

		font-weight: bold;

		&.active {
			background-color: theme.$primaryColor;
			color: #ffffff;
		}

		&:hover {
			text-decoration: underline;
		}

		:global(svg) {
			margin-right: 0.5rem;
		}
	}
</style>
