<script lang="ts">
	import { base } from "$app/paths";
	import { page } from "$app/stores";
	import { type Page } from "@sveltejs/kit";

	import HomeIcon from "~icons/solar/home-bold-duotone";
	import ReportsIcon from "~icons/solar/chart-bold-duotone";
	import LibraryIcon from "~icons/solar/book-2-bold-duotone";
	import BrowseIcon from "~icons/solar/earth-bold-duotone";
	import Logo from "./icons/Logo.svelte";

	function isActivePath(page: Page, path: string): boolean {
		return page.url.pathname === `${base}${path}`;
	}
</script>

<aside class="sidebar">
	<a href={base} class="logo">
		<Logo height="4rem" width="100%" />
	</a>

	<nav class="nav">
		<div class="nav__group">
			<a href="{base}/" class:active={isActivePath($page, "/")}>
				<HomeIcon /> Home
			</a>
			<a href="{base}/browse" class:active={isActivePath($page, "/browse")}>
				<BrowseIcon /> Browse
			</a>
			<a href="{base}/library" class:active={isActivePath($page, "/library")}>
				<LibraryIcon /> Library
			</a>
			<a href="{base}/reports" class:active={isActivePath($page, "/reports")}>
				<ReportsIcon /> Reports
			</a>
		</div>
	</nav>
</aside>

<style lang="scss">
	@use "../assets/theme.scss" as theme;

	.logo {
		margin-bottom: 1rem;
	}

	.spacer {
		margin-top: auto;
	}

	.sidebar {
		overflow: auto;

		display: flex;
		flex-flow: column;
		padding: 1rem;
		width: 20rem;

		border-right: 2px solid #ccc;
		box-shadow: 0 0 10px rgba($color: #000000, $alpha: 0.15);
	}

	.nav {
		display: flex;
		flex-flow: column;
		gap: 0.5rem;

		height: 100%;

		justify-content: space-between;

		&__group {
			display: flex;
			flex-flow: column;
			gap: 0.5rem;
		}
	}

	a {
		padding: 0.75rem;
		background-color: #fff;
		border-radius: 0.2rem;
		color: #444;
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
