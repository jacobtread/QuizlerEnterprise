<script lang="ts">
	import { user } from "$lib/stores/auth";
</script>

<h1>Welcome {$user.username}</h1>
<p>
	{JSON.stringify($user)}
</p>

<style lang="scss">
</style>
