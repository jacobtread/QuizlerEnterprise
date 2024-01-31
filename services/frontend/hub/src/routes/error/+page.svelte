<script lang="ts">
	import { onMount } from "svelte";

	let name: string | null = null;
	let description: string | null = null;
	let backURL: string | null = null;

	onMount(() => {
		const searchParams: URLSearchParams = new URLSearchParams(window.location.search);
		name = searchParams.get("name");
		description = searchParams.get("description");
		backURL = searchParams.get("back");
	});
</script>

<h1>Error</h1>
<span>{name ?? "unknown"}</span>
<p>{description ?? "Unknown error occurred"}</p>
<a href={backURL ?? "/"}>Go Back</a>
