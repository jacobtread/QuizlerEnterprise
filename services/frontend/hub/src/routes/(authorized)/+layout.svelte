<script lang="ts">
	import RequireAuth from "$lib/components/RequireAuth.svelte";
</script>

<RequireAuth>
	<slot />
</RequireAuth>
