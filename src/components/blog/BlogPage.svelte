<script lang="ts">
	import type { BasicPostData } from "@api/interfaces";

	export let posts: BasicPostData[];
	export let totalPages: number;
</script>

<div class="container">
	<h2>Blog</h2>
	{#each posts as post}
		<h3>{post.title}</h3>
		{#if post.summary}
			<div class="summary">{@html post.summary}</div>
		{/if}
	{/each}
	{totalPages}
</div>
