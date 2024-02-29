<script lang="ts">
	import { page } from "$app/stores";
	import { useQuiz } from "$lib/api/quiz";
	import { user } from "$lib/stores/auth";

	const query = useQuiz(Number($page.params.quiz));
</script>

<h1>Welcome {$user.username} {JSON.stringify($query.data)}</h1>

<style lang="scss">
</style>
