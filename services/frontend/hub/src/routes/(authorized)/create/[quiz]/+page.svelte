<script lang="ts">
	import { page } from "$app/stores";
	import { user } from "$lib/stores/auth";

	const quiz = $page.params.quiz;
</script>

<h1>Welcome {$user.username} {quiz}</h1>

<style lang="scss">
</style>
