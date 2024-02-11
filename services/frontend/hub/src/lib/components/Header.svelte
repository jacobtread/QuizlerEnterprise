<script lang="ts">
	import { base } from "$app/paths";
	import Logo from "./icons/Logo.svelte";
</script>

<header class="header">
	<div class="header__left">
		<Logo width="auto" height="2rem" />
	</div>

	<div class="header__right">
		<a href="{base}/create">Create</a>

		<div>
			Avatar
			<img src="" alt="" />
		</div>
	</div>
</header>

<style lang="scss">
	.header {
		background: #fff;

		border: 1px solid #ccc;
		padding: 0.5rem;

		display: flex;
		justify-content: space-between;

		z-index: 1;
	}
</style>
