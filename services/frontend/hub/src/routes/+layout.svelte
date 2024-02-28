<script lang="ts">
	import "$lib/assets/global.scss";
	import "@fontsource-variable/inter";

	import { QueryClient, QueryClientProvider } from "@sveltestack/svelte-query";
	// Create a client
	const queryClient = new QueryClient();
</script>

<QueryClientProvider client={queryClient}>
	<slot />
</QueryClientProvider>
